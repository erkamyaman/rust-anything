//! The skill tells an agent which commands to run, so a command it names that the CLI does not
//! have sends every run down a dead end. These checks read the skill files like an agent would.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn skill_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("skills/rust-anything")
}

/// Subcommand names listed under `Commands:` of `rust-anything <args> --help`.
fn subcommands(args: &[&str]) -> BTreeSet<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_rust-anything"))
        .args(args)
        .arg("--help")
        .output()
        .unwrap();
    let text = String::from_utf8_lossy(&out.stdout).into_owned();
    let mut names = BTreeSet::new();
    let mut in_commands = false;
    for line in text.lines() {
        if line.starts_with("Commands:") {
            in_commands = true;
        } else if in_commands {
            let Some(rest) = line.strip_prefix("  ") else {
                break;
            };
            if let Some(name) = rest.split_whitespace().next() {
                names.insert(name.to_string());
            }
        }
    }
    names
}

/// The code in a markdown file: fenced lines whole, and inline backtick spans.
fn code_snippets(markdown: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut in_fence = false;
    for line in markdown.lines() {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
        } else if in_fence {
            found.push(line.to_string());
        } else {
            found.extend(
                line.split('`')
                    .skip(1)
                    .step_by(2)
                    .map(|span| span.to_string()),
            );
        }
    }
    found
}

fn skill_files() -> Vec<(String, String)> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(skill_dir()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "md") {
            let name = path.file_name().unwrap().to_string_lossy().into_owned();
            files.push((name, std::fs::read_to_string(&path).unwrap()));
        }
    }
    files.sort();
    files
}

#[test]
fn skill_front_matter_names_the_folder_and_describes_when_to_use_it() {
    let text = std::fs::read_to_string(skill_dir().join("SKILL.md")).unwrap();
    let rest = text.strip_prefix("---\n").expect("front matter");
    let (front, _) = rest.split_once("\n---\n").expect("front matter end");
    let field = |key: &str| {
        front
            .lines()
            .find_map(|l| l.strip_prefix(&format!("{key}: ")))
            .unwrap_or_else(|| panic!("front matter has no `{key}`"))
    };
    assert_eq!(field("name"), "rust-anything");
    let description = field("description");
    assert!(description.len() <= 1024, "{} chars", description.len());
    assert!(description.contains("Use when"), "{description}");
}

#[test]
fn every_command_the_skill_names_exists() {
    let top = subcommands(&[]);
    let app = subcommands(&["app"]);
    assert!(
        top.contains("gate") && app.contains("record"),
        "{top:?} {app:?}"
    );
    let word = |w: Option<&&str>| {
        w.filter(|c| c.chars().all(|ch| ch.is_ascii_lowercase() || ch == '-'))
            .map(|c| c.to_string())
    };
    let mut missing = Vec::new();
    for (file, text) in skill_files() {
        for snippet in code_snippets(&text) {
            let words: Vec<&str> = snippet.split_whitespace().collect();
            for (i, w) in words.iter().enumerate() {
                let Some(cmd) = word(words.get(i + 1)).filter(|_| *w == "rust-anything") else {
                    continue;
                };
                let sub = word(words.get(i + 2)).filter(|_| cmd == "app");
                match sub {
                    Some(sub) if top.contains(&cmd) && !app.contains(&sub) => {
                        missing.push(format!("{file}: rust-anything app {sub}"));
                    }
                    _ if !top.contains(&cmd) => {
                        missing.push(format!("{file}: rust-anything {cmd}"));
                    }
                    _ => {}
                }
            }
        }
    }
    assert!(
        missing.is_empty(),
        "the skill names commands the CLI lacks: {missing:?}"
    );
}

#[test]
fn links_between_skill_files_resolve() {
    let mut broken = Vec::new();
    for (file, text) in skill_files() {
        for part in text.split("](").skip(1) {
            let target = part.split(')').next().unwrap();
            if target.starts_with("http") || target.starts_with('#') {
                continue;
            }
            let path = target.split('#').next().unwrap();
            if !skill_dir().join(path).exists() {
                broken.push(format!("{file}: {target}"));
            }
        }
    }
    assert!(broken.is_empty(), "{broken:?}");
}
