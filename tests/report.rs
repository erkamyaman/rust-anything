//! What the plan says about units you cannot start with.

use std::path::Path;

use rust_anything::analyze::analyze;
use rust_anything::report;

#[test]
fn a_parse_error_names_a_status_that_exists_and_an_empty_start_here_explains_itself() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/broken-source");
    let report = analyze(&root).unwrap();
    let md = report::markdown(&report, "broken-source");
    // The statuses are ready, caution, blocked and keep: there is no "partial".
    assert!(
        md.contains("marked caution"),
        "the parse-error note should name a real status:\n{md}"
    );
    assert!(!md.contains("marked partial"), "{md}");
    // Nothing is blocked here, so pointing the reader at "the blocked units" sends them nowhere:
    // the reason nothing starts is that no unit exports a function a bridge could carry.
    assert!(
        md.contains("No unit exports anything a bridge can carry"),
        "an empty start_here should say why:\n{md}"
    );
    assert!(!md.contains("See the blocked units"), "{md}");
}

#[test]
fn an_empty_start_here_with_only_kept_units_says_so() {
    let root = std::env::temp_dir().join(format!("ra-report-keep-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"ui","dependencies":{"react":"18.0.0"}}"#,
    )
    .unwrap();
    std::fs::write(
        root.join("src/Button.tsx"),
        "export function Button() { return <button>hi</button>; }\n",
    )
    .unwrap();
    let md = report::markdown(&analyze(&root).unwrap(), "ui");
    assert!(
        md.contains("every unit stays in the source language"),
        "{md}"
    );
    assert!(!md.contains("No unit exports anything"), "{md}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn a_one_line_unit_prints_one_line_not_one_lines() {
    // A package with a single one-line unit, as the issue describes.
    let root = std::env::temp_dir().join(format!("ra-report-oneline-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("package.json"),
        "{\"name\":\"one\",\"version\":\"1.0.0\"}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/math.ts"),
        "export function add(a, b) { return a + b }\n",
    )
    .unwrap();
    let report = analyze(&root).unwrap();
    let md = report::markdown(&report, "oneline");
    assert!(md.contains("1 source file (1 line)"), "{md}");
    assert!(md.contains("1 line in 1 file"), "{md}");
    assert!(!md.contains("1 lines"), "{md}");
    let _ = std::fs::remove_dir_all(&root);
}

#[test]
fn the_risk_guide_is_left_out_when_no_unit_has_a_risk() {
    let root = std::env::temp_dir().join(format!("ra-report-norisk-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(
        root.join("package.json"),
        "{\"name\":\"tiny\",\"version\":\"1.0.0\"}\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/yes.ts"),
        "export function yes(): boolean { return true }\n",
    )
    .unwrap();
    let report = analyze(&root).unwrap();
    assert!(report.units.iter().all(|u| u.risks.is_empty()));
    let md = report::markdown(&report, "tiny");
    assert!(!md.contains("## Risk guide"), "{md}");

    // A unit with a risk still gets the guide, with that risk under it.
    std::fs::write(
        root.join("src/yes.ts"),
        "export function read(s: string): unknown { return JSON.parse(s) }\n",
    )
    .unwrap();
    let report = analyze(&root).unwrap();
    let md = report::markdown(&report, "tiny");
    let guide = md.split("## Risk guide\n").nth(1).expect(&md);
    assert!(guide.trim_start().starts_with("- **"), "{md}");
    let _ = std::fs::remove_dir_all(&root);
}
