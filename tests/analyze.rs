use std::path::Path;

use rust_anything::analyze::{Bridge, Report, Status, UnitReport, analyze};
use rust_anything::risks::Risk;

fn mini() -> Report {
    analyze(&Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini")).unwrap()
}

fn unit<'r>(report: &'r Report, id: &str) -> &'r UnitReport {
    report
        .units
        .iter()
        .find(|u| u.id == id)
        .unwrap_or_else(|| panic!("no unit {id}"))
}

#[test]
fn groups_files_into_units_and_skips_fixtures() {
    let report = mini();
    let ids: Vec<&str> = report.units.iter().map(|u| u.id.as_str()).collect();
    assert_eq!(
        ids,
        [
            "packages/a/src/fs.ts",
            "packages/a/src/parse/",
            "packages/a/src/svelte.ts",
            "packages/a/src/util.ts",
            "packages/a/src/view/",
            "packages/b/src/index.ts",
        ]
    );
    assert_eq!(unit(&report, "packages/a/src/parse/").files.len(), 2);
    assert_eq!(report.summary.test_files, 1);
}

#[test]
fn resolves_imports_into_waves_and_credits_tests() {
    let report = mini();
    let util = unit(&report, "packages/a/src/util.ts");
    assert_eq!(util.wave, 0);
    assert_eq!(util.direct_tests, 1);
    assert_eq!(util.fan_in, 2);
    let fs = unit(&report, "packages/a/src/fs.ts");
    assert_eq!(fs.wave, 1);
    assert_eq!(fs.internal_deps, ["packages/a/src/util.ts"]);
    assert_eq!(
        unit(&report, "packages/b/src/index.ts").workspace_deps,
        ["a"]
    );
}

#[test]
fn finds_risks_with_locations() {
    let report = mini();
    let util = unit(&report, "packages/a/src/util.ts");
    let regex = util
        .risks
        .iter()
        .find(|r| r.risk == Risk::RegexSyntax)
        .unwrap();
    assert_eq!(regex.samples, ["packages/a/src/util.ts:1"]);
    assert!(util.risks.iter().any(|r| r.risk == Risk::StringOffsets));
}

#[test]
fn picks_status_and_bridge() {
    let report = mini();
    let fs = unit(&report, "packages/a/src/fs.ts");
    assert_eq!((fs.status, fs.bridge), (Status::Ready, Bridge::NapiRs));
    let view = unit(&report, "packages/a/src/view/");
    assert_eq!((view.status, view.bridge), (Status::Keep, Bridge::KeepInJs));
    assert_eq!(
        unit(&report, "packages/a/src/svelte.ts").status,
        Status::Blocked
    );
    // `b` uses no Node API, so its code can run anywhere.
    assert_eq!(
        unit(&report, "packages/b/src/index.ts").bridge,
        Bridge::WasmBindgen
    );
    let order: Vec<(&str, usize)> = report
        .packages
        .iter()
        .map(|p| (p.name.as_str(), p.order))
        .collect();
    assert_eq!(order, [("a", 1), ("b", 2)]);
    assert!(!report.start_here.is_empty());
    assert!(
        !report
            .start_here
            .iter()
            .any(|id| id.ends_with("svelte.ts") || id.ends_with("view/"))
    );
}

#[test]
fn json_output_is_versioned() {
    let json = serde_json::to_value(mini()).unwrap();
    assert_eq!(json["schema"], "rust-anything/analyze@1");
    assert_eq!(json["units"][0]["status"], "ready");
    assert_eq!(json["units"][0]["bridge"], "napi-rs");
}

fn whole(dir: &str) -> rust_anything::whole_app::WholeApp {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(dir);
    let report = analyze(&root).unwrap();
    rust_anything::whole_app::whole_app(&root, &report)
}

#[test]
fn whole_app_finds_the_shape_and_entry_points_in_every_language() {
    let cases = [
        ("cli/ts", "cli", "tcli.js"),
        ("cli/python", "cli", "tcli.py"),
        ("cli/c", "cli", "tcli.c"),
        ("cli/go", "cli", "main.go"),
        ("cli/ruby", "cli", "tcli.rb"),
        ("cli/java", "cli", "Tcli.java"),
        ("http/ts", "server", "server.js"),
        ("http/python", "server", "server.py"),
        ("http/c", "server", "server.c"),
        ("http/go", "server", "main.go"),
        ("http/ruby", "server", "server.rb"),
        ("http/java", "server", "Server.java"),
        ("go", "cli", "cmd/strkit"),
        ("c", "cli", "src/main.c"),
        ("mini", "library", "a"),
        ("python", "library", "textkit"),
        ("ruby", "library", "textkit"),
        ("java", "library", "textkit"),
    ];
    for (dir, shape, path) in cases {
        let w = whole(dir);
        assert_eq!(w.shape, shape, "{dir}: {:?}", w.entry_points);
        assert!(
            w.entry_points
                .iter()
                .any(|e| e.path.ends_with(path) || e.name.as_deref() == Some(path)),
            "{dir}: {:?}",
            w.entry_points
        );
        assert_eq!(w.decisions[0].topic, "shape");
        assert!(
            w.dependencies.iter().all(|d| d.fit != "unknown"),
            "{dir}: {:?}",
            w.dependencies
        );
    }
}

#[test]
fn whole_app_lists_declared_dependencies_keeps_and_decisions() {
    let python = whole("python");
    let requests = python
        .dependencies
        .iter()
        .find(|d| d.name == "requests")
        .unwrap();
    assert!(requests.declared);
    assert_eq!(requests.fit, "good");

    let mini = whole("mini");
    let topics: Vec<&str> = mini.decisions.iter().map(|d| d.topic.as_str()).collect();
    assert!(topics.contains(&"keep packages/a/src/view/"), "{topics:?}");
    assert!(
        topics.contains(&"blocked packages/a/src/svelte.ts"),
        "{topics:?}"
    );
    assert!(topics.contains(&"dependency svelte"), "{topics:?}");
    assert!(mini.decisions.iter().all(|d| !d.recommendation.is_empty()));
    let ids: Vec<&str> = mini.decisions.iter().map(|d| d.id.as_str()).collect();
    assert_eq!(ids[..3], ["D1", "D2", "D3"]);
}

#[test]
fn whole_app_ports_units_that_import_each_other_as_one_piece() {
    let root = std::env::temp_dir().join(format!("rust-anything-tangled-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(root.join("package.json"), r#"{"name":"tangled"}"#).unwrap();
    std::fs::write(
        root.join("src/ping.ts"),
        "import { pong } from './pong'\nexport function ping(n: number): number { return n > 0 ? pong(n - 1) : 0 }\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/pong.ts"),
        "import { ping } from './ping'\nexport function pong(n: number): number { return n > 0 ? ping(n - 1) : 1 }\n",
    )
    .unwrap();
    std::fs::write(
        root.join("src/leaf.ts"),
        "export function leaf(s: string): string { return s.trim() }\n",
    )
    .unwrap();
    let report = analyze(&root).unwrap();
    let w = rust_anything::whole_app::whole_app(&root, &report);
    let tangled: Vec<&Vec<String>> = w
        .pieces
        .iter()
        .filter(|p| p.tangled)
        .map(|p| &p.units)
        .collect();
    assert_eq!(
        tangled,
        [&vec!["src/ping.ts".to_string(), "src/pong.ts".to_string()]]
    );
    assert_eq!(w.pieces.len(), 2);
    assert!(w.decisions.iter().any(|d| d.topic == "tangled units"));
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn discovery_ignores_parent_rules_but_honors_project_rules() {
    let parent = std::env::temp_dir().join(format!(
        "rust-anything-ignore-boundary-{}",
        std::process::id()
    ));
    let root = parent.join("project");
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::write(parent.join(".gitignore"), "*\n").unwrap();
    std::fs::write(root.join(".gitignore"), "src/ignored.*\n").unwrap();
    std::fs::write(root.join("package.json"), r#"{"name":"project"}"#).unwrap();
    std::fs::write(root.join("src/kept.ts"), "export function kept() {}").unwrap();
    std::fs::write(root.join("src/ignored.ts"), "export function ignored() {}").unwrap();
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"project\"\n",
    )
    .unwrap();
    std::fs::write(root.join("src/kept.py"), "def kept():\n    pass\n").unwrap();
    std::fs::write(root.join("src/ignored.py"), "def ignored():\n    pass\n").unwrap();
    let ts = rust_anything::analyze::analyze_lang(&root, rust_anything::lang::Lang::Ts);
    let python = analyze(&root);
    std::fs::remove_dir_all(parent).unwrap();
    let (ts, python) = (ts.unwrap(), python.unwrap());
    let files = ts
        .units
        .iter()
        .chain(&python.units)
        .flat_map(|unit| &unit.files)
        .collect::<Vec<_>>();
    assert!(files.iter().any(|file| file.as_str() == "src/kept.ts"));
    assert!(files.iter().any(|file| file.as_str() == "src/kept.py"));
    assert!(files.iter().all(|file| !file.starts_with("src/ignored.")));
}
