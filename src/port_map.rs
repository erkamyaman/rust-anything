use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde::Serialize;

use crate::analyze::Status;
use crate::bridge::{MANIFEST, Manifest, SCHEMA};
use crate::front_door::APP_DIR;
use crate::harness::CASES;
use crate::recording;

pub const MAP_FILE: &str = "rust/PORT_MAP.md";

#[derive(Debug, Serialize)]
pub struct PortMap {
    pub root: PathBuf,
    pub ports: Vec<Port>,
    pub not_ported: Vec<Unported>,
    pub glue: Vec<Glue>,
    /// Crate folders under `rust/` whose manifest could not be read. `map` has to skip them, and a
    /// skipped port is not a missing port: the file says so instead of listing the unit as unported.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub crate_errors: Vec<String>,
    /// Units a broken manifest still names: they are covered, so they belong in the total even
    /// though no `Port` could be built for them.
    #[serde(skip_serializing_if = "is_zero")]
    pub covered_by_broken: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub analyze_error: Option<String>,
}

fn is_zero(n: &usize) -> bool {
    *n == 0
}

#[derive(Debug, Serialize)]
pub struct Port {
    pub unit: String,
    #[serde(rename = "crate")]
    pub crate_dir: String,
    pub language: String,
    pub kind: String,
    pub in_rust: Vec<String>,
    pub kept_in_original: Vec<String>,
    pub cases: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_diff: Option<(usize, usize)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed_upstream: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Unported {
    pub unit: String,
    pub loc: usize,
    pub readiness: u32,
    pub exports: usize,
    /// `analyze`'s verdict: ready, caution, blocked or keep.
    pub status: Status,
    /// Why `analyze` calls it blocked or keep, when it says.
    pub reasons: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Glue {
    pub file: String,
    pub falls_back: bool,
}

impl PortMap {
    /// Units with at least one port. `ports` has a row per crate, and one unit can have several
    /// (napi and wasm, or a second `bridge --out`).
    pub fn ported_units(&self) -> usize {
        let mut units: Vec<&str> = self.ports.iter().map(|p| p.unit.as_str()).collect();
        units.sort_unstable();
        units.dedup();
        units.len()
    }

    pub fn changed(&self) -> usize {
        self.ports
            .iter()
            .filter(|p| p.changed_upstream.is_some())
            .count()
    }
}

pub fn map(root: &Path, extra: &[PathBuf]) -> Result<PortMap> {
    let root =
        std::fs::canonicalize(root).with_context(|| format!("mapping {}", root.display()))?;
    // A file is not a repository: `analyze` says so, and a map over a file used to come out empty
    // and exit 0, or fail on the first directory read without naming what it read.
    if !root.is_dir() {
        bail!(
            "{} is a file; `map` takes the folder of a project's source code",
            root.display()
        );
    }
    let mut crate_dirs = manifests(&root);
    for dir in extra {
        let dir =
            std::fs::canonicalize(dir).with_context(|| format!("mapping {}", dir.display()))?;
        if !dir.is_dir() {
            bail!("{} is a file; `--crates` takes folders", dir.display());
        }
        crate_dirs.extend(manifests(&dir));
    }
    crate_dirs.sort();
    crate_dirs.dedup();

    let mut ports = Vec::new();
    let mut crate_errors = Vec::new();
    let mut covered_by_broken: Vec<String> = Vec::new();
    for dir in crate_dirs {
        let m = match Manifest::load(&dir) {
            Ok(m) => m,
            Err(e) => {
                // The manifest did not load, but its JSON can still say what it covers and where:
                // a folder that belongs to another repository stays skipped without a word, and a
                // local one still covers its unit, so the unit is not reported as unported.
                let raw: Option<serde_json::Value> = std::fs::read_to_string(dir.join(MANIFEST))
                    .ok()
                    .and_then(|t| serde_json::from_str(&t).ok());
                // Foreign only when the root actually resolves somewhere else: a `root` that does
                // not exist says nothing about ownership, and its unit still belongs here.
                let foreign = raw
                    .as_ref()
                    .and_then(|v| v["root"].as_str())
                    .and_then(|r| std::fs::canonicalize(dir.join(r)).ok())
                    .is_some_and(|resolved| resolved != root);
                let other_schema = raw
                    .as_ref()
                    .and_then(|v| v["schema"].as_str())
                    .is_some_and(|s| s != SCHEMA);
                if foreign || other_schema {
                    continue;
                }
                if let Some(unit) = raw.as_ref().and_then(|v| v["unit"].as_str()) {
                    covered_by_broken.push(unit.to_string());
                }
                // With `/` on every OS, as a readable port's `crate_dir` is.
                let shown = dir
                    .strip_prefix(&root)
                    .unwrap_or(&dir)
                    .to_string_lossy()
                    .replace('\\', "/");
                crate_errors.push(format!("`{shown}`: {e:#}"));
                continue;
            }
        };
        if std::fs::canonicalize(&m.root).ok().as_deref() != Some(root.as_path()) {
            continue;
        }
        ports.push(port(&root, &dir, &m));
    }
    ports.sort_by(|a, b| a.unit.cmp(&b.unit).then(a.crate_dir.cmp(&b.crate_dir)));

    let (not_ported, broken_units, analyze_error) = match crate::analyze::analyze(&root) {
        Ok(report) => {
            let order = |id: &str| {
                report
                    .start_here
                    .iter()
                    .position(|s| s == id)
                    .unwrap_or(usize::MAX)
            };
            let unported: Vec<_> = report
                .units
                .iter()
                .filter(|u| !ports.iter().any(|p| covers(&p.unit, &u.id, &u.files)))
                .collect();
            // A unit a broken manifest still names is covered, but it is still a unit of this
            // repository: the report counts it, or the summary reads as if it did not exist.
            let (covered, mut units): (Vec<_>, Vec<_>) = unported.into_iter().partition(|u| {
                covered_by_broken
                    .iter()
                    .any(|unit| covers(unit, &u.id, &u.files))
            });
            units.sort_by(|a, b| {
                order(&a.id)
                    .cmp(&order(&b.id))
                    .then(b.priority.cmp(&a.priority))
                    .then(a.id.cmp(&b.id))
            });
            let units = units
                .into_iter()
                .map(|u| Unported {
                    unit: u.id.clone(),
                    loc: u.loc,
                    readiness: u.readiness,
                    exports: u.exports,
                    status: u.status,
                    reasons: u.reasons.clone(),
                })
                .collect();
            (units, covered.len(), None)
        }
        Err(e) => (Vec::new(), 0, Some(format!("{e:#}"))),
    };

    Ok(PortMap {
        glue: glue(&root),
        root,
        ports,
        not_ported,
        crate_errors,
        covered_by_broken: broken_units,
        analyze_error,
    })
}

fn covers(unit: &str, id: &str, files: &[String]) -> bool {
    unit == id || files.iter().any(|f| f == unit)
}

fn port(root: &Path, dir: &Path, m: &Manifest) -> Port {
    let in_rust = m
        .exports
        .iter()
        .map(|f| f.name.clone())
        .chain(m.classes.iter().map(|c| c.name.clone()))
        .collect();
    let kept_in_original = m.js_only.iter().map(|j| j.name.clone()).collect();
    let cases = std::fs::read_to_string(dir.join(CASES))
        .map(|t| t.lines().filter(|l| !l.trim().is_empty()).count())
        .unwrap_or(0);
    Port {
        unit: m.unit.clone(),
        crate_dir: dir
            .strip_prefix(root)
            .unwrap_or(dir)
            .to_string_lossy()
            .replace('\\', "/"),
        language: format!("{:?}", m.language).to_lowercase(),
        kind: format!("{:?}", m.kind).to_lowercase(),
        in_rust,
        kept_in_original,
        cases,
        last_diff: recording::last_diff(dir),
        changed_upstream: recording::stale(dir, m),
    }
}
fn manifests(dir: &Path) -> Vec<PathBuf> {
    ignore::WalkBuilder::new(dir)
        .hidden(false)
        .filter_entry(|e| {
            !matches!(
                e.file_name().to_str(),
                Some("node_modules" | "target" | ".git" | "vendor" | ".venv" | "venv")
            )
        })
        .build()
        .filter_map(Result::ok)
        .filter(|e| e.file_name() == MANIFEST && e.file_type().is_some_and(|t| t.is_file()))
        .filter_map(|e| e.path().parent().map(Path::to_path_buf))
        .collect()
}

fn glue(root: &Path) -> Vec<Glue> {
    let units = root.join(APP_DIR).join("src/units");
    let Ok(entries) = std::fs::read_dir(&units) else {
        return Vec::new();
    };
    let mut glue: Vec<Glue> = entries
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().is_some_and(|x| x == "rs") && e.file_name() != "mod.rs")
        .map(|e| Glue {
            file: format!("{APP_DIR}/src/units/{}", e.file_name().to_string_lossy()),
            falls_back: std::fs::read_to_string(e.path()).is_ok_and(|t| t.contains("original::")),
        })
        .collect();
    glue.sort_by(|a, b| a.file.cmp(&b.file));
    glue
}

/// The status word `analyze` uses, for the table and the list under it.
fn status_name(status: Status) -> &'static str {
    match status {
        Status::Ready => "ready",
        Status::Caution => "caution",
        Status::Blocked => "blocked",
        Status::Keep => "keep",
    }
}

pub fn markdown(map: &PortMap) -> String {
    let mut out = String::new();
    let _ = render(&mut out, map);
    out
}

fn render(out: &mut String, map: &PortMap) -> std::fmt::Result {
    let name = map.root.file_name().map_or_else(
        || map.root.display().to_string(),
        |n| n.to_string_lossy().into(),
    );
    writeln!(out, "# Port map: {name}\n")?;
    let ported = map.ported_units();
    let total = ported + map.not_ported.len() + map.covered_by_broken;
    write!(
        out,
        "{} of {} {} a Rust port",
        ported,
        if map.analyze_error.is_some() {
            "the".to_string()
        } else {
            total.to_string()
        },
        // One unit reads "1 of 1 unit has", not "units have".
        if total == 1 && map.analyze_error.is_none() {
            "unit has"
        } else {
            "units have"
        }
    )?;
    match map.changed() {
        _ if map.ports.is_empty() => writeln!(out, ".\n")?,
        0 => writeln!(out, "; no original changed since it was recorded.\n")?,
        n => writeln!(
            out,
            "; **{n} changed upstream** since they were recorded: re-record them and run `diff`.\n"
        )?,
    }

    if !map.crate_errors.is_empty() {
        writeln!(out, "## Crate folders that could not be read\n")?;
        writeln!(
            out,
            "A folder under `rust/` whose manifest does not parse: fix its `rust-anything.json` and run `map` again. A unit the file still names is not listed as unported, so it does not read as work to do:\n"
        )?;
        for e in &map.crate_errors {
            writeln!(out, "- {e}")?;
        }
        writeln!(out)?;
    }

    writeln!(out, "## Ports\n")?;
    if map.ports.is_empty() {
        writeln!(out, "None yet: `rust-anything bridge` starts one.\n")?;
    } else {
        writeln!(
            out,
            "| Unit | Crate | In Rust | Kept in the original | Cases | Last diff | Original |"
        )?;
        writeln!(out, "| --- | --- | --- | --- | ---: | ---: | --- |")?;
        for p in &map.ports {
            writeln!(
                out,
                "| `{}` | `{}` | {} | {} | {} | {} | {} |",
                p.unit,
                p.crate_dir,
                list(&p.in_rust),
                list(&p.kept_in_original),
                p.cases,
                p.last_diff
                    .map_or("not run".into(), |(m, t)| format!("{m}/{t}")),
                if p.changed_upstream.is_some() {
                    format!("**changed**: `rust-anything record {}`", p.crate_dir)
                } else {
                    "unchanged".into()
                }
            )?;
        }
        writeln!(out)?;
    }

    if !map.glue.is_empty() {
        writeln!(out, "## Whole app\n")?;
        writeln!(out, "| Glue | Runs |")?;
        writeln!(out, "| --- | --- |")?;
        for g in &map.glue {
            writeln!(
                out,
                "| `{}` | {} |",
                g.file,
                if g.falls_back {
                    "Rust, handing some cases to the original"
                } else {
                    "Rust only"
                }
            )?;
        }
        writeln!(out)?;
    }

    writeln!(out, "## Not ported yet\n")?;
    if let Some(e) = &map.analyze_error {
        writeln!(out, "`analyze` could not list the units: {e}\n")?;
    } else if total == 0 {
        writeln!(out, "No unit to port was found.\n")?;
    } else if map.not_ported.is_empty() {
        writeln!(out, "Every unit `analyze` found has a port.\n")?;
    } else {
        writeln!(out, "In the order `analyze` suggests.\n")?;
        writeln!(out, "| Unit | Status | Lines | Exports | Readiness |")?;
        writeln!(out, "| --- | --- | ---: | ---: | ---: |")?;
        for u in &map.not_ported {
            writeln!(
                out,
                "| `{}` | {} | {} | {} | {} |",
                u.unit,
                status_name(u.status),
                u.loc,
                u.exports,
                u.readiness
            )?;
        }
        writeln!(out)?;
        // A unit that stays in the original is not work to do: say so in its own short list,
        // with the reason `analyze` gave, or the table above reads as remaining work.
        let left: Vec<&Unported> = map
            .not_ported
            .iter()
            .filter(|u| !matches!(u.status, Status::Keep | Status::Blocked))
            .collect();
        let stays: Vec<&Unported> = map
            .not_ported
            .iter()
            .filter(|u| matches!(u.status, Status::Keep | Status::Blocked))
            .collect();
        if !stays.is_empty() {
            writeln!(
                out,
                "{} of these {} no port on purpose ({} blocked, {} kept in the original), so {} {} left to port:\n",
                stays.len(),
                if stays.len() == 1 {
                    "unit has"
                } else {
                    "units have"
                },
                map.not_ported
                    .iter()
                    .filter(|u| matches!(u.status, Status::Blocked))
                    .count(),
                map.not_ported
                    .iter()
                    .filter(|u| matches!(u.status, Status::Keep))
                    .count(),
                left.len(),
                if left.len() == 1 {
                    "unit is"
                } else {
                    "units are"
                },
            )?;
            for u in stays {
                let why = if u.reasons.is_empty() {
                    String::new()
                } else {
                    let reasons: Vec<&str> =
                        u.reasons.iter().map(|n| n.trim_end_matches('.')).collect();
                    format!(", {}", reasons.join("; "))
                };
                writeln!(out, "- `{}`: {}{why}", u.unit, status_name(u.status))?;
            }
            writeln!(out)?;
        }
    }
    writeln!(
        out,
        "Regenerate with `rust-anything map --write`. Commit this file: when the original changes \
         upstream, the next map marks every port whose recording it outdated."
    )
}

fn list(items: &[String]) -> String {
    if items.is_empty() {
        "-".into()
    } else {
        items
            .iter()
            .map(|i| format!("`{i}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_map() -> PortMap {
        PortMap {
            root: PathBuf::from("empty"),
            ports: Vec::new(),
            not_ported: Vec::new(),
            glue: Vec::new(),
            crate_errors: Vec::new(),
            covered_by_broken: 0,
            analyze_error: None,
        }
    }

    #[test]
    fn a_unit_that_stays_in_the_original_is_not_counted_as_work_to_do() {
        // `analyze` says why a unit is blocked or kept; without that column and the list under the
        // table, `map` reads as if every unported unit were remaining work.
        let mut map = empty_map();
        map.not_ported = vec![
            Unported {
                unit: "src/util.ts".into(),
                loc: 19,
                readiness: 94,
                exports: 3,
                status: crate::analyze::Status::Ready,
                reasons: Vec::new(),
            },
            Unported {
                unit: "src/view/".into(),
                loc: 3,
                readiness: 100,
                exports: 0,
                status: crate::analyze::Status::Keep,
                reasons: vec!["UI code: keep in the original language.".into()],
            },
            Unported {
                unit: "src/svelte.ts".into(),
                loc: 4,
                readiness: 92,
                exports: 0,
                status: crate::analyze::Status::Blocked,
                reasons: vec!["`svelte` has no Rust equivalent.".into()],
            },
        ];
        let text = markdown(&map);
        assert!(
            text.contains("| Unit | Status | Lines | Exports | Readiness |"),
            "{text}"
        );
        assert!(text.contains("| `src/util.ts` | ready |"), "{text}");
        assert!(text.contains("| `src/view/` | keep |"), "{text}");
        assert!(text.contains("| `src/svelte.ts` | blocked |"), "{text}");
        // The two that stay are named with their reason, and only the ready one counts as work.
        assert!(
            text.contains("2 of these units have no port on purpose (1 blocked, 1 kept in the original), so 1 unit is left to port:"),
            "{text}"
        );
        assert!(
            text.contains("- `src/svelte.ts`: blocked, `svelte` has no Rust equivalent\n"),
            "{text}"
        );
        assert!(
            text.contains("- `src/view/`: keep, UI code: keep in the original language\n"),
            "{text}"
        );
    }

    #[test]
    fn an_empty_map_says_no_unit_was_found() {
        let text = markdown(&empty_map());
        assert!(text.contains("No unit to port was found."), "{text}");
        assert!(
            !text.contains("Every unit `analyze` found has a port."),
            "{text}"
        );
    }
}
