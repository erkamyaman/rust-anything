use std::path::Path;
use std::process::Command;

fn project(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("rust-anything-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("pyproject.toml"),
        "[project]\nname = \"slug\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    std::fs::write(
        root.join("slug.py"),
        "def slug(text: str) -> str:\n    return text.strip().lower().replace(\" \", \"-\")\n",
    )
    .unwrap();
    root
}

fn bridge(root: &Path, extra: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rust-anything"))
        .args(["bridge", ".", "slug.py"])
        .args(extra)
        .current_dir(root)
        .output()
        .unwrap()
}

#[test]
fn bridge_keeps_the_end_state_it_was_given() {
    let root = project("end-state");
    let out = bridge(&root, &["--end-state", "scaffold"]);
    assert!(out.status.success(), "{out:?}");
    let manifest = std::fs::read_to_string(root.join("rust/slug-rs/rust-anything.json")).unwrap();
    assert!(
        manifest.contains(r#""end_state": "scaffold""#),
        "{manifest}"
    );
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn bridge_without_an_end_state_writes_none() {
    let root = project("no-end-state");
    let out = bridge(&root, &[]);
    assert!(out.status.success(), "{out:?}");
    let manifest = std::fs::read_to_string(root.join("rust/slug-rs/rust-anything.json")).unwrap();
    assert!(!manifest.contains("end_state"), "{manifest}");
    std::fs::remove_dir_all(&root).unwrap();
}
