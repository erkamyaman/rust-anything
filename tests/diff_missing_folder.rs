//! `diff` must not suggest `record` when the crate folder has no manifest.

use std::process::Command;

fn run_diff(dir: &std::path::Path) -> String {
    let out = Command::new(env!("CARGO_BIN_EXE_rust-anything"))
        .arg("diff")
        .arg(dir)
        .output()
        .unwrap();
    assert!(!out.status.success());
    String::from_utf8_lossy(&out.stderr).into_owned()
}

#[test]
fn diff_on_a_missing_folder_says_to_run_bridge_first() {
    let dir =
        std::env::temp_dir().join(format!("rust-anything-diff-missing-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);

    let err = run_diff(&dir);
    assert!(err.contains("run `rust-anything bridge` first"), "{err}");
    assert!(err.contains(&dir.display().to_string()), "{err}");
    assert!(!err.contains("run `rust-anything record` first"), "{err}");
}

#[test]
fn diff_on_a_crate_without_cases_still_says_to_record_first() {
    let dir = std::env::temp_dir().join(format!(
        "rust-anything-diff-no-cases-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("demo/fixtures/c-checksum-rs/rust-anything.json");
    std::fs::copy(fixture, dir.join("rust-anything.json")).unwrap();

    let err = run_diff(&dir);
    let _ = std::fs::remove_dir_all(&dir);
    assert!(err.contains("run `rust-anything record` first"), "{err}");
    assert!(!err.contains("run `rust-anything bridge` first"), "{err}");
}
