mod common;

use rust_anything::{bridge, codegen_ruby, harness};

#[test]
fn ruby_recording_fails_on_unencodable_results_and_preserves_cases() {
    if !common::have_ruby_3() {
        return;
    }

    let root =
        std::env::temp_dir().join(format!("rust-anything-ruby-record-{}", std::process::id()));
    let lib = root.join("lib");
    std::fs::create_dir_all(&lib).unwrap();
    std::fs::write(root.join("textkit.gemspec"), "").unwrap();
    let source = lib.join("textkit.rb");
    std::fs::write(
        &source,
        "module Textkit\n  def self.value(s)\n    s\n  end\nend\n",
    )
    .unwrap();
    let port = root.join("port");
    bridge::bridge(&bridge::Options {
        root: &root,
        unit: "lib/textkit.rb",
        kind: None,
        out: Some(port.clone()),
        name: None,
        force: false,
        keep: Vec::new(),
        lang: Some(rust_anything::lang::Lang::Ruby),
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })
    .unwrap();
    let opts = harness::RecordOptions {
        crate_dir: &port,
        count: 2,
        inputs: None,
        module: None,
        resolve_from: None,
        node_args: &[],
    };
    let first = harness::record(&opts).unwrap();
    assert!(first.cases > 0);
    let original = std::fs::read(&first.path).unwrap();
    assert_eq!(first.errors, 0, "{}", String::from_utf8_lossy(&original));
    std::fs::write(
        &source,
        "module Textkit\n  def self.value(s)\n    [255].pack('C').force_encoding('UTF-8')\n  end\nend\n",
    )
    .unwrap();
    let result = harness::record(&opts);
    let retained = std::fs::read(&first.path).unwrap();
    std::fs::remove_dir_all(root).unwrap();

    let err = result.unwrap_err();
    assert!(
        format!("{err:#}").contains("cannot encode recorded result"),
        "{err:#}"
    );
    assert_eq!(retained, original);
}

#[test]
fn ruby_diff_reports_missing_wrapper_target_even_if_the_error_type_matches() {
    if !common::have("ruby") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-ruby-diff-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let targets = root.join("targets.json");
    let cases = root.join("cases.jsonl");
    let summary = root.join("diff.json");
    std::fs::write(&targets, "{}").unwrap();
    std::fs::write(
        &cases,
        r#"{"fn":"removed","args":[],"error":"missing key","error_type":"KeyError"}"#,
    )
    .unwrap();

    let output = std::process::Command::new("ruby")
        .arg("-e")
        .arg(codegen_ruby::driver())
        .arg("diff")
        .arg(targets)
        .arg(cases)
        .arg(&summary)
        .arg("0")
        .env("RUST_ANYTHING_WRAPPER", root.join("wrapper.rb"))
        .output()
        .unwrap();
    let report = std::fs::read(&summary);
    std::fs::remove_dir_all(root).unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&report.unwrap()).unwrap();
    assert_eq!(report["total"], 1);
    assert_eq!(report["passed"], 0);
    assert_eq!(report["failed"], 1);
    assert_eq!(report["mismatches"][0]["fn"], "removed");
    assert_eq!(report["mismatches"][0]["actual"]["error_type"], "KeyError");
}
