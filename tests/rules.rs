//! Port rules: `rules --add` appends to rust/RULES.md once per rule, a crate folder stands for its
//! original's repository, and the agent prompt `go` writes carries them.

use std::path::{Path, PathBuf};
use std::process::Command;

use rust_anything::bridge::{self, Manifest};
use rust_anything::go::{self, Pick};
use rust_anything::rules;

const CLI: &str = env!("CARGO_BIN_EXE_rust-anything");

fn temp(name: &str) -> PathBuf {
    let dir =
        std::env::temp_dir().join(format!("rust-anything-rules-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

/// A crate folder under `repo/rust/` whose manifest points back at `repo`.
fn crate_in(repo: &Path) -> PathBuf {
    let dir = repo.join("rust/util-rs");
    std::fs::create_dir_all(&dir).unwrap();
    let manifest: Manifest = serde_json::from_value(serde_json::json!({
        "schema": bridge::SCHEMA,
        "root": repo,
        "unit": "util.ts",
        "crate_name": "util-rs",
        "kind": "napi",
        "exports": [],
    }))
    .unwrap();
    manifest.save(&dir).unwrap();
    dir
}

#[test]
fn rules_are_added_once_and_listed() {
    let repo = temp("add");
    let added = rules::add(
        &repo,
        &[
            "Dates print in the user's local time zone.".into(),
            "Dates print in the user's local time zone.".into(),
        ],
    )
    .unwrap();
    assert_eq!(added.added, ["Dates print in the user's local time zone."]);
    assert_eq!(
        added.skipped,
        ["Dates print in the user's local time zone."]
    );

    let out = Command::new(CLI)
        .args([
            "rules",
            "--add",
            "Dates print in the user's local time zone.",
        ])
        .args([
            "--add",
            "Empty input prints nothing, not a blank line.",
            "--json",
        ])
        .arg(&repo)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(
        json["added"],
        serde_json::json!(["Empty input prints nothing, not a blank line."])
    );
    assert_eq!(json["rules"].as_array().unwrap().len(), 2);

    let text = std::fs::read_to_string(repo.join(rules::FILE)).unwrap();
    assert!(text.starts_with("# Port rules\n"), "{text}");
    assert_eq!(text.matches("- Dates print").count(), 1, "{text}");

    let shown = Command::new(CLI).arg("rules").arg(&repo).output().unwrap();
    let shown = String::from_utf8_lossy(&shown.stdout);
    assert!(
        shown.contains("- Empty input prints nothing, not a blank line."),
        "{shown}"
    );
    std::fs::remove_dir_all(&repo).unwrap();
}

#[test]
fn a_crate_folder_adds_to_its_repository_and_task_md_carries_the_rules() {
    let repo = temp("crate");
    let dir = crate_in(&repo);
    let pick = Pick {
        function: "count".into(),
        file: "util.ts".into(),
        line: 1,
        total_pct: 90.0,
        batch: None,
        caller: None,
        why: String::new(),
    };
    let before = go::task(&dir, &pick, &["node".into(), "bench.mjs".into()]);
    assert!(!before.contains("## Project rules"), "{before}");
    assert!(
        before.contains("Never hard-code recorded outputs or weaken tests"),
        "{before}"
    );

    rules::add(&dir, &["Tabs count as one column.".into()]).unwrap();
    assert!(repo.join(rules::FILE).is_file());
    assert!(!dir.join(rules::FILE).exists());
    assert_eq!(rules::load(&dir), ["Tabs count as one column."]);
    assert_eq!(rules::load(&repo), ["Tabs count as one column."]);

    let task = go::task(&dir, &pick, &["node".into(), "bench.mjs".into()]);
    let at = task.find("## Project rules").expect("rules section");
    assert!(at < task.find("## Loop").unwrap(), "{task}");
    assert!(task.contains("- Tabs count as one column.\n"), "{task}");
    std::fs::remove_dir_all(&repo).unwrap();
}
