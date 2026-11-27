//! `app drift`: whether the recording still describes the original, and the port still matches
//! it. Replays the original and the build against `rust/app-cases`, and lists the documented
//! examples no case records.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::app::{self, Config, Failure, ReplayOptions, ReplayOutcome};
use crate::walk::Canonical;

pub const REPORT_FILE: &str = "drift-report.md";

pub struct DriftOptions<'a> {
    pub repo: &'a Path,
    /// Command line of the build to check; `rust/app`, built with cargo, when `None` and it exists.
    pub build: Option<&'a str>,
    pub timeout_secs: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct Drifted {
    pub case: String,
    pub args: Vec<String>,
    pub fields: Vec<String>,
    pub verdict: String,
}

#[derive(Debug, Serialize)]
pub struct DriftOutcome {
    /// The build checked, if any.
    pub build: Option<Vec<String>>,
    pub cases: usize,
    /// Cases the original no longer matches: it changed upstream.
    pub changed: Vec<Drifted>,
    /// Cases the build no longer matches.
    pub failing: Vec<Drifted>,
    /// Examples in the original's docs that no case records.
    pub new: Vec<String>,
    pub drifted: bool,
    pub report: PathBuf,
}

fn fields(f: &Failure) -> Vec<String> {
    f.problems.iter().map(|p| p.field.clone()).collect()
}

fn failed(outcome: &ReplayOutcome) -> BTreeSet<&str> {
    outcome.failures.iter().map(|f| f.case.as_str()).collect()
}

pub fn drift(opts: &DriftOptions) -> Result<DriftOutcome> {
    app::check_timeout(opts.timeout_secs)?;
    let repo = opts
        .repo
        .canonical()
        .with_context(|| format!("no folder {}", opts.repo.display()))?;
    let dir = app::cases_dir(&repo);
    let Some(config) = Config::load(&repo)? else {
        if let Ok(state) = crate::front_door::State::load(&repo)
            && state.kind == crate::front_door::AppKind::Library
        {
            bail!(
                "{} is a library: `app drift` checks CLIs and servers; use `rust-anything map --check` to find units whose original changed",
                state.name
            );
        }
        bail!(
            "no {} in {}: run `rust-anything app record` first",
            app::CONFIG_FILE,
            dir.display()
        );
    };
    let build = match opts.build {
        Some(b) => Some(b.to_string()),
        None => {
            let app_dir = repo.join(crate::front_door::APP_DIR);
            if app_dir.join("Cargo.toml").is_file() {
                let exe = crate::front_door::cargo_build(&app_dir, Some(&app_dir.join("target")))?;
                Some(app::shell_quote(&exe.to_string_lossy()))
            } else {
                None
            }
        }
    };
    let original = app::replay_unwritten(&ReplayOptions {
        repo: &repo,
        build: None,
        timeout_secs: opts.timeout_secs,
        port: None,
        case: None,
    })?;
    let port = match &build {
        Some(b) => Some(app::replay_unwritten(&ReplayOptions {
            repo: &repo,
            build: Some(b),
            timeout_secs: opts.timeout_secs,
            port: None,
            case: None,
        })?),
        None => None,
    };
    let new = app::unrecorded_examples(&repo, &config)?;

    let original_failed = failed(&original);
    let port_failed = port.as_ref().map(failed).unwrap_or_default();
    let changed: Vec<Drifted> = original
        .failures
        .iter()
        .map(|f| Drifted {
            case: f.case.clone(),
            args: f.args.clone(),
            fields: fields(f),
            verdict: match &port {
                None => "original changed",
                Some(_) if port_failed.contains(f.case.as_str()) => "original and port both differ",
                Some(_) => "original changed, port still old",
            }
            .to_string(),
        })
        .collect();
    let failing: Vec<Drifted> = port
        .iter()
        .flat_map(|p| &p.failures)
        .map(|f| Drifted {
            case: f.case.clone(),
            args: f.args.clone(),
            fields: fields(f),
            verdict: if original_failed.contains(f.case.as_str()) {
                "original and port both differ"
            } else {
                "port regression"
            }
            .to_string(),
        })
        .collect();
    let outcome = DriftOutcome {
        build: port.as_ref().map(|p| p.build.clone()),
        cases: original.total,
        drifted: !changed.is_empty() || !failing.is_empty() || !new.is_empty(),
        changed,
        failing,
        new,
        report: dir.join(REPORT_FILE),
    };
    let text = app::stable_paths(&report(&outcome, &original, port.as_ref()));
    std::fs::write(&outcome.report, text)
        .with_context(|| format!("writing {}", outcome.report.display()))?;
    Ok(outcome)
}

fn quoted(words: &[String]) -> String {
    words
        .iter()
        .map(|w| app::shell_quote(w))
        .collect::<Vec<_>>()
        .join(" ")
}

fn diffs(out: &mut String, drifted: &[Drifted], replay: &ReplayOutcome) {
    for d in drifted {
        *out += &format!(
            "\n### {} ({})\n\nArguments: `{}`\n",
            d.case,
            d.verdict,
            quoted(&d.args)
        );
        let Some(f) = replay.failures.iter().find(|f| f.case == d.case) else {
            continue;
        };
        for p in &f.problems {
            if let Some(at) = &p.at {
                *out += &format!("\n{}\n", at.describe().replace("```", "`\u{200b}``"));
            }
            *out += &format!(
                "\n{}:\n\n```diff\n{}```\n",
                p.field,
                p.detail.replace("```", "`\u{200b}``")
            );
        }
    }
}

fn report(
    outcome: &DriftOutcome,
    original: &ReplayOutcome,
    port: Option<&ReplayOutcome>,
) -> String {
    let mut out = String::from("# App drift\n\n");
    out += &format!(
        "Original: {}/{} cases match\n\n",
        original.passed, original.total
    );
    match port {
        Some(p) => {
            out += &format!(
                "Build: `{}`: {}/{} cases match\n\n",
                quoted(&p.build),
                p.passed,
                p.total
            )
        }
        None => out += "Build: none (pass --build, or create rust/app with `app front-door`)\n\n",
    }
    if !outcome.drifted {
        out += "No drift: the recording still describes the original, and every documented example is recorded.\n";
        return out;
    }
    if !outcome.changed.is_empty() {
        out += "## Changed upstream\n\nThe original no longer gives what was recorded. If the change is intended, record again (`rust-anything app record`) and port it.\n";
        diffs(&mut out, &outcome.changed, original);
    }
    if let Some(p) = port
        && !outcome.failing.is_empty()
    {
        out += "\n## Port failures\n\nThe build no longer matches the recording.\n";
        diffs(&mut out, &outcome.failing, p);
    }
    if !outcome.new.is_empty() {
        out += "\n## New examples\n\nDocumented but not recorded; `rust-anything app record` adds them.\n\n";
        for n in &outcome.new {
            out += &format!("- `{n}`\n");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::{DriftOutcome, Drifted, report};
    use crate::app::{Failure, Problem, ReplayOutcome};

    fn replay(build: &[&str], passed: usize, failures: Vec<Failure>) -> ReplayOutcome {
        ReplayOutcome {
            build: build.iter().map(|w| w.to_string()).collect(),
            passed,
            total: 3,
            skipped: 0,
            failures,
            fallback: None,
            report: PathBuf::new(),
            groups: Vec::new(),
        }
    }

    fn outcome(changed: Vec<Drifted>, failing: Vec<Drifted>, new: Vec<&str>) -> DriftOutcome {
        DriftOutcome {
            build: None,
            cases: 3,
            drifted: !changed.is_empty() || !failing.is_empty() || !new.is_empty(),
            changed,
            failing,
            new: new.into_iter().map(String::from).collect(),
            report: PathBuf::new(),
        }
    }

    fn drifted(case: &str) -> Drifted {
        Drifted {
            case: case.into(),
            args: vec!["--name".into(), "a b".into()],
            fields: vec!["stdout".into()],
            verdict: "changed".into(),
        }
    }

    fn failure(case: &str, detail: &str) -> Failure {
        Failure {
            case: case.into(),
            args: Vec::new(),
            problems: vec![Problem::new("stdout", detail)],
        }
    }

    #[test]
    fn no_drift_and_no_build() {
        let text = report(
            &outcome(vec![], vec![], vec![]),
            &replay(&["node", "cli.js"], 3, vec![]),
            None,
        );
        assert!(
            text.starts_with("# App drift\n\nOriginal: 3/3 cases match\n\n"),
            "{text}"
        );
        assert!(
            text.contains("Build: none (pass --build, or create rust/app with `app front-door`)\n"),
            "{text}"
        );
        assert!(
            text.contains("No drift: the recording still describes the original"),
            "{text}"
        );
        assert!(!text.contains("## "), "{text}");
    }

    #[test]
    fn a_build_reports_its_command_and_matches() {
        let port = replay(&["target/app", "--flag"], 2, vec![]);
        let text = report(
            &outcome(vec![], vec![], vec![]),
            &replay(&["node", "cli.js"], 3, vec![]),
            Some(&port),
        );
        assert!(
            text.contains("Build: `target/app --flag`: 2/3 cases match\n"),
            "{text}"
        );
    }

    #[test]
    fn a_changed_case_lists_its_arguments_and_diff() {
        let original = replay(
            &["node", "cli.js"],
            2,
            vec![failure("greet", "-hello\n+hi\n")],
        );
        let text = report(
            &outcome(vec![drifted("greet")], vec![], vec![]),
            &original,
            None,
        );
        assert!(text.contains("## Changed upstream\n"), "{text}");
        assert!(
            text.contains("\n### greet (changed)\n\nArguments: `--name 'a b'`\n"),
            "{text}"
        );
        assert!(
            text.contains("\nstdout:\n\n```diff\n-hello\n+hi\n```\n"),
            "{text}"
        );
        assert!(!text.contains("No drift"), "{text}");
        assert!(!text.contains("## Port failures"), "{text}");
        assert!(!text.contains("## New examples"), "{text}");
    }

    #[test]
    fn port_failures_use_the_builds_replay() {
        let original = replay(&["node", "cli.js"], 3, vec![]);
        let port = replay(
            &["target/app"],
            2,
            vec![failure("greet", "-hello\n+hullo\n")],
        );
        let text = report(
            &outcome(vec![], vec![drifted("greet")], vec![]),
            &original,
            Some(&port),
        );
        assert!(!text.contains("## Changed upstream"), "{text}");
        assert!(text.contains("\n## Port failures\n"), "{text}");
        assert!(text.contains("+hullo\n```\n"), "{text}");
    }

    #[test]
    fn new_examples_get_one_line_each() {
        let text = report(
            &outcome(vec![], vec![], vec!["cli --help", "cli greet x"]),
            &replay(&["node", "cli.js"], 3, vec![]),
            None,
        );
        assert!(text.contains("\n## New examples\n"), "{text}");
        assert!(
            text.contains("\n- `cli --help`\n- `cli greet x`\n"),
            "{text}"
        );
        assert!(!text.contains("No drift"), "{text}");
    }

    #[test]
    fn triple_backticks_in_a_diff_cannot_close_the_fence() {
        let original = replay(
            &["node", "cli.js"],
            2,
            vec![failure("md", "-```js\n+```ts\n")],
        );
        let text = report(
            &outcome(vec![drifted("md")], vec![], vec![]),
            &original,
            None,
        );
        let body = text.split("```diff\n").nth(1).unwrap();
        let (inside, _) = body.split_once("```\n").unwrap();
        assert_eq!(inside, "-`\u{200b}``js\n+`\u{200b}``ts\n");
        assert_eq!(text.matches("```").count(), 2, "{text}");
    }
}
