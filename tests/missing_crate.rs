//! `disable`, `report` and `shadow` on a crate folder that does not exist name the folder and
//! say to run `bridge` first, like the commands that read the manifest straight away.

use std::path::PathBuf;

use rust_anything::{card, enable, shadow};

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
fn report_on_a_missing_folder_says_to_bridge_first() {
    let dir = missing("report");
    let err = card::report(&card::Options {
        crate_dir: &dir,
        out: None,
        min_ms: 1,
        node_args: &[],
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
