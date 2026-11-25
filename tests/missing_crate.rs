//! `disable`, `report` and `shadow` on a crate folder that does not exist name the folder and
//! say to run `bridge` first, like the commands that read the manifest straight away.

use std::path::PathBuf;

use rust_anything::{card, enable, harness, shadow};

fn missing(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!(
        "rust-anything-missing-{name}-{}",
        std::process::id()
    ))
}

fn assert_hint(err: anyhow::Error, dir: &std::path::Path) {
    let msg = format!("{err:#}");
    assert!(msg.contains(&dir.display().to_string()), "{msg}");
    assert!(msg.contains("run `rust-anything bridge` first"), "{msg}");
}

#[test]
fn disable_on_a_missing_folder_says_to_bridge_first() {
    let dir = missing("disable");
    assert_hint(enable::disable(&dir).unwrap_err(), &dir);
}

#[test]
fn report_takes_resolve_from_like_bench_and_fuzz() {
    // A repo whose node_modules lives elsewhere: `bench` and `fuzz` take `--resolve-from`, and
    // `report` runs bench internally, so it has to take it too.
    let dir = missing("report-resolve");
    let err = card::report(&card::Options {
        crate_dir: &dir,
        out: None,
        min_ms: 1,
        node_args: &[],
        resolve_from: Some(&dir),
    })
    .unwrap_err();
    // The folder is missing, so the report stops before any npm lookup: the flag is accepted and
    // carried, which is what this asserts.
    assert_hint(err, &dir);
}

#[test]
fn report_on_a_missing_folder_says_to_bridge_first() {
    let dir = missing("report");
    let err = card::report(&card::Options {
        crate_dir: &dir,
        out: None,
        min_ms: 1,
        node_args: &[],
        resolve_from: None,
    })
    .unwrap_err();
    assert_hint(err, &dir);
}

#[test]
fn shadow_on_a_missing_folder_says_to_bridge_first() {
    let dir = missing("shadow");
    assert_hint(shadow::shadow(&dir, None).unwrap_err(), &dir);
}

#[test]
fn a_folder_without_a_manifest_still_names_the_manifest() {
    let dir = missing("no-manifest");
    std::fs::create_dir_all(&dir).unwrap();
    let err = enable::disable(&dir).unwrap_err();
    let _ = std::fs::remove_dir_all(&dir);
    let msg = format!("{err:#}");
    assert!(msg.contains("rust-anything.json"), "{msg}");
    assert!(msg.contains("run `rust-anything bridge` first"), "{msg}");
}

#[test]
fn enable_partial_without_cases_says_to_record_first() {
    // `enable --partial` and `enable --only` (without `--force`) count passes per function through
    // `diff_full_with_counts`, which read `cases.jsonl` straight away and failed with a bare
    // "No such file or directory" instead of the hint plain `enable` gives.
    let dir = missing("no-cases");
    std::fs::create_dir_all(&dir).unwrap();
    let err = harness::diff_full_with_counts(&harness::DiffOptions {
        crate_dir: &dir,
        module: None,
        ignore_key_order: false,
        node_args: &[],
        float_tolerance: None,
    })
    .unwrap_err();
    let _ = std::fs::remove_dir_all(&dir);
    let msg = format!("{err:#}");
    assert!(
        msg.contains(&format!("no cases.jsonl in {}", dir.display())),
        "{msg}"
    );
    assert!(msg.contains("run `rust-anything record` first"), "{msg}");
}

#[test]
fn bench_and_report_reject_zero_min_ms() {
    let dir = missing("zero-min-ms");
    let err = harness::bench(&harness::BenchOptions {
        crate_dir: &dir,
        min_ms: 0,
        resolve_from: None,
        node_args: &[],
    })
    .unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("--min-ms"), "{msg}");
    assert!(msg.contains("at least 1"), "{msg}");

    let err = card::report(&card::Options {
        crate_dir: &dir,
        out: None,
        min_ms: 0,
        node_args: &[],
        resolve_from: None,
    })
    .unwrap_err();
    let msg = format!("{err:#}");
    assert!(msg.contains("--min-ms"), "{msg}");
    assert!(msg.contains("at least 1"), "{msg}");
}
