//! Port rules: behavior the port must keep exactly, learned from the user's corrections.
//!
//! They live in `<repo>/rust/RULES.md`, one `- rule` bullet each, next to the port's decisions
//! and map, and every agent prompt rust-anything writes includes them, so a correction made once
//! reaches every later unit and every agent.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use serde::Serialize;

use crate::bridge::{MANIFEST, Manifest};

pub const FILE: &str = "rust/RULES.md";

const HEADER: &str = "# Port rules\n\n\
Behavior the port must keep exactly, learned from corrections. Every agent prompt rust-anything \
writes includes these rules. Edit them by hand, or add one with `rust-anything rules --add`.\n\n";

#[derive(Debug, Serialize)]
pub struct RulesOutcome {
    pub file: PathBuf,
    pub rules: Vec<String>,
    pub added: Vec<String>,
    /// Rules given to add that the file already had.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub skipped: Vec<String>,
}

/// The repository a path stands for: a crate folder made by `bridge` stands for its original's
/// repository, any other folder for itself.
pub fn repo_of(path: &Path) -> PathBuf {
    if path.join(MANIFEST).is_file()
        && let Ok(m) = Manifest::load(path)
    {
        return m.root;
    }
    path.to_path_buf()
}

/// The rules in the repository's rules file, in order; none when it does not exist.
pub fn load(root: &Path) -> Vec<String> {
    let text = std::fs::read_to_string(repo_of(root).join(FILE)).unwrap_or_default();
    text.lines()
        .filter_map(|l| l.trim_start().strip_prefix("- "))
        .map(|r| r.trim().to_string())
        .filter(|r| !r.is_empty())
        .collect()
}

/// Appends each rule that is not in the file yet, creating the file when needed. With no rules
/// to add it only reads.
pub fn add(root: &Path, rules: &[String]) -> Result<RulesOutcome> {
    let file = repo_of(root).join(FILE);
    let mut have = load(root);
    let mut added = Vec::new();
    let mut skipped = Vec::new();
    for rule in rules {
        let rule = rule.split_whitespace().collect::<Vec<_>>().join(" ");
        anyhow::ensure!(!rule.is_empty(), "a rule needs text");
        if have.contains(&rule) {
            skipped.push(rule);
        } else {
            have.push(rule.clone());
            added.push(rule);
        }
    }
    if !added.is_empty() {
        let mut text = std::fs::read_to_string(&file).unwrap_or_else(|_| HEADER.to_string());
        if !text.is_empty() && !text.ends_with('\n') {
            text.push('\n');
        }
        for rule in &added {
            text.push_str(&format!("- {rule}\n"));
        }
        if let Some(dir) = file.parent() {
            std::fs::create_dir_all(dir).with_context(|| format!("creating {}", dir.display()))?;
        }
        std::fs::write(&file, text).with_context(|| format!("writing {}", file.display()))?;
    }
    Ok(RulesOutcome {
        file,
        rules: have,
        added,
        skipped,
    })
}

/// The `## Project rules` section of an agent prompt; empty when the repository has none.
pub fn section(root: &Path) -> String {
    let rules = load(root);
    if rules.is_empty() {
        return String::new();
    }
    let mut out = format!(
        "## Project rules\n\nFrom `{FILE}`: behavior the port must keep exactly, learned from \
         corrections. Follow every one.\n\n"
    );
    for rule in rules {
        out.push_str(&format!("- {rule}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rules_are_bullets_and_anything_else_is_kept_as_written() {
        let dir = crate::app::temp_dir("rules-load").unwrap();
        std::fs::create_dir_all(dir.join("rust")).unwrap();
        std::fs::write(
            dir.join(FILE),
            "# Port rules\n\nSome text.\n\n- keep trailing spaces\n  - nested still counts\n-\n",
        )
        .unwrap();
        assert_eq!(load(&dir), ["keep trailing spaces", "nested still counts"]);
        let out = add(&dir, &["one  more\nrule".into()]).unwrap();
        let text = std::fs::read_to_string(dir.join(FILE)).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(out.added, ["one more rule"]);
        assert!(text.starts_with("# Port rules\n\nSome text."), "{text}");
        assert!(text.ends_with("-\n- one more rule\n"), "{text}");
    }

    #[test]
    fn no_rules_no_section() {
        let dir = crate::app::temp_dir("rules-none").unwrap();
        assert_eq!(section(&dir), "");
        let out = add(&dir, &[]).unwrap();
        assert!(out.rules.is_empty());
        assert!(!dir.join(FILE).exists(), "reading writes nothing");
        assert!(add(&dir, &["  ".into()]).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
