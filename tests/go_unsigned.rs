mod common;

use rust_anything::{bridge, harness};

const SOURCE: &str = "package u

func Hash(x uint64) uint64 { return x*0x9E3779B97F4A7C15 ^ (x >> 31) }

func Scale(n uint, by uintptr) uintptr { return uintptr(n) * by }

func Sum(xs []uint64) uint64 {
\tvar total uint64
\tfor _, x := range xs {
\t\ttotal += x
\t}
\treturn total
}

func Widen(x int64, by uint32) int64 { return x + int64(by) }
";

const PORT: &str = "#![forbid(unsafe_code)]

pub fn hash(x: u64) -> Result<u64, String> {
    Ok(x.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ (x >> 31))
}

pub fn scale(n: u64, by: usize) -> Result<usize, String> {
    Ok((n as usize).wrapping_mul(by))
}

pub fn sum(xs: Vec<u64>) -> Result<u64, String> {
    Ok(xs.iter().fold(0, |total, x| total.wrapping_add(*x)))
}

pub fn widen(x: i64, by: i64) -> Result<i64, String> {
    Ok(x.wrapping_add(by))
}
";

/// `Hash(1)`: above `i64::MAX`, so a port that returns `i64` cannot give it back.
const HASH_OF_ONE: &str = "11400714819323198485";

#[test]
fn unsigned_64_bit_go_integers_keep_their_range_in_the_port() {
    let root =
        std::env::temp_dir().join(format!("rust-anything-go-unsigned-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("u")).unwrap();
    std::fs::write(root.join("go.mod"), "module example.com/u\n\ngo 1.21\n").unwrap();
    std::fs::write(root.join("u/u.go"), SOURCE).unwrap();
    let out = root.join("port");
    bridge::bridge(&bridge::Options {
        root: &root,
        unit: "u/",
        kind: None,
        out: Some(out.clone()),
        name: Some("u-rs".into()),
        force: false,
        keep: Vec::new(),
        lang: None,
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })
    .unwrap();
    let port = std::fs::read_to_string(out.join("src/port.rs")).unwrap();
    for stub in [
        "pub fn hash(x: u64) -> Result<u64, String>",
        "pub fn scale(n: u64, by: usize) -> Result<usize, String>",
        "pub fn sum(xs: Vec<u64>) -> Result<u64, String>",
        // Types that fit in `i64` keep it.
        "pub fn widen(x: i64, by: i64) -> Result<i64, String>",
    ] {
        assert!(port.contains(stub), "missing `{stub}` in:\n{port}");
    }
    if !common::have("go") {
        let _ = std::fs::remove_dir_all(&root);
        return;
    }
    std::fs::write(out.join("src/port.rs"), PORT).unwrap();
    let recorded = harness::record(&harness::RecordOptions {
        crate_dir: &out,
        count: 10,
        inputs: None,
        module: None,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap();
    let cases = std::fs::read_to_string(out.join("cases.jsonl")).unwrap();
    assert!(cases.contains(HASH_OF_ONE), "{cases}");
    harness::build(&out).unwrap();
    let diff = harness::diff(&harness::DiffOptions {
        crate_dir: &out,
        module: None,
        ignore_key_order: false,
        float_tolerance: None,
        node_args: &[],
    })
    .unwrap();
    assert_eq!(diff.passed, diff.total, "{diff:?}");
    assert_eq!(diff.total, recorded.cases);
    let _ = std::fs::remove_dir_all(&root);
}
