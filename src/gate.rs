//! Step 5: the definition of done for one port.
//!
//! fmt, clippy (warnings are errors), cargo tests, no unexplained `unsafe`, every function
//! ported, the built module exposes the original API, and the recorded behavior replays cleanly.

use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::bridge::Manifest;
use crate::harness::{self, CASES, DiffOptions};

#[derive(Debug, Serialize)]
pub struct Check {
    pub name: &'static str,
    pub passed: bool,
    pub detail: String,
    /// The next command or edit that clears a failed check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix: Option<String>,
}

/// Recovery guidance when an outside tool failed to start (structured `SpawnError`).
fn spawn_fix(err: &anyhow::Error) -> Option<String> {
    let spawn = err.downcast_ref::<crate::process::SpawnError>()?;
    Some(if spawn.missing {
        match crate::doctor::missing_command(&spawn.program) {
            Some(install) if !install.contains('\n') => {
                format!(
                    "install `{}` with `{install}`, or run `rust-anything doctor`",
                    spawn.program
                )
            }
            _ => format!(
                "run `rust-anything doctor` for how to install `{}`",
                spawn.program
            ),
        }
    } else {
        "run `rust-anything doctor` and check that the build tool can run".into()
    })
}

/// The fix for a failed check, by its name.
fn fix_for(check: &Check, crate_dir: &Path) -> Option<String> {
    let c = crate_dir.display();
    let manifest = format!("--manifest-path {c}/Cargo.toml");
    Some(match check.name {
        "fmt" => format!("cargo fmt {manifest}"),
        "clippy" => {
            format!("cargo clippy --all-targets {manifest} -- -D warnings, then fix each warning")
        }
        "cargo test" => format!("cargo test {manifest}"),
        "unsafe" => "add a `// SAFETY:` comment above each listed `unsafe`, or remove it".into(),
        "ported" => {
            "port the functions still stubbed in src/port.rs, then `rust-anything gate` again"
                .into()
        }
        "size" => "use the crates `rust-anything analyze` lists and fold repeated code".into(),
        "literals" => "compute these values the way the original does; a constant the original \
                       takes from a library can stay with a `// recorded-literal:` comment"
            .into(),
        "tests" => format!(
            "restore the removed tests or assertions; if the change is intended, delete \
             `{c}/{TESTS_BASELINE}` and run gate again"
        ),
        "build" => format!("fix the compile error above, then `rust-anything build {c}`"),
        "recording" => format!("rust-anything record {c}"),
        "behavior" if check.detail.contains("no recorded cases") => {
            format!("rust-anything record {c}")
        }
        "behavior" => format!(
            "rust-anything diff {c} to see each mismatch; fix the port, or list an intended difference in {}",
            crate::recording::KNOWN_DIFFS
        ),
        "known diffs" => format!(
            "remove the entries that no longer happen from {c}/{}",
            crate::recording::KNOWN_DIFFS
        ),
        "api" => {
            format!("export the missing functions from src/port.rs, then `rust-anything build {c}`")
        }
        "error cases" | "panics" => {
            format!("rust-anything diff {c} to see the cases where the port's errors differ")
        }
        _ => return None,
    })
}

/// Bindings `bridge` writes into `src/`; everything else there is the port.
const GENERATED: [&str; 8] = [
    "lib.rs",
    "js.rs",
    "pinned.rs",
    "pyconv.rs",
    "pynum.rs",
    "rbconv.rs",
    "utf16.rs",
    "wire.rs",
];

/// How many times the source's lines the port may take before `gate` calls it bloated, plus a
/// fixed allowance for the imports and signatures every port has.
pub const MAX_GROWTH: f64 = 3.0;
const ALLOWANCE: f64 = 50.0;

fn files_under(path: &Path, extensions: &[&str]) -> Vec<std::path::PathBuf> {
    if path.is_file() {
        return vec![path.to_path_buf()];
    }
    crate::walk::builder(path)
        .build()
        .flatten()
        .map(ignore::DirEntry::into_path)
        .filter(|p| {
            p.extension()
                .is_some_and(|e| extensions.contains(&e.to_string_lossy().as_ref()))
        })
        .collect()
}

/// Lines of Rust in the port against lines of the source unit: a port several times larger
/// usually re-implements what a crate already does.
/// The unit's files and the repository files they import, directly or not: a port stands in
/// for all of them.
pub(crate) fn with_local_imports(root: &Path, files: Vec<PathBuf>) -> Vec<PathBuf> {
    const TRIES: &[&str] = &[
        "",
        ".ts",
        ".tsx",
        ".mts",
        ".cts",
        ".js",
        ".mjs",
        ".cjs",
        "/index.ts",
        "/index.tsx",
        "/index.js",
        "/index.mjs",
    ];
    let resolve = |from: &Path, spec: &str| -> Option<PathBuf> {
        let base = from.parent()?.join(spec);
        let stem = base.with_extension("");
        let js_to_ts = matches!(
            base.extension().and_then(|e| e.to_str()),
            Some("js" | "mjs" | "cjs")
        );
        TRIES
            .iter()
            .map(|t| PathBuf::from(format!("{}{t}", base.display())))
            .chain(js_to_ts.then(|| stem.with_extension("ts")))
            .find(|p| p.is_file())
    };
    let mut seen: std::collections::BTreeSet<PathBuf> = files.into_iter().collect();
    let mut todo: Vec<PathBuf> = seen.iter().cloned().collect();
    while let Some(file) = todo.pop() {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        for import in crate::ts::scan::scan(&file.to_string_lossy(), &text).imports {
            if !import.specifier.starts_with('.') || import.type_only {
                continue;
            }
            if let Some(found) = resolve(&file, &import.specifier)
                && found.starts_with(root)
                && !found.components().any(|c| c.as_os_str() == "node_modules")
                && seen.insert(found.clone())
            {
                todo.push(found);
            }
        }
    }
    seen.into_iter().collect()
}

/// The original's files a port stands for: the unit, plus what it imports (TS) or calls (C).
fn source_files(manifest: &Manifest) -> Vec<PathBuf> {
    let mut files = files_under(
        &manifest.root.join(&manifest.unit),
        manifest.language.extensions(),
    );
    if manifest.language == crate::lang::Lang::Ts {
        files = with_local_imports(&manifest.root, files);
    }
    if manifest.language == crate::lang::Lang::C {
        let ported: Vec<&str> = manifest.exports.iter().map(|f| f.name.as_str()).collect();
        files.extend(crate::lang::c::called_implementations(
            &manifest.root,
            &manifest.unit,
            &ported,
        ));
    }
    files
}

/// The port's own Rust files under `src/`: everything but the generated bindings.
fn port_files(crate_dir: &Path) -> Vec<PathBuf> {
    files_under(&crate_dir.join("src"), &["rs"])
        .into_iter()
        .filter(|p| {
            p.file_name()
                .is_none_or(|n| !GENERATED.contains(&n.to_string_lossy().as_ref()))
        })
        .collect()
}

fn size_check(crate_dir: &Path, manifest: &Manifest) -> Option<Check> {
    let source: usize = source_files(manifest)
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .map(|t| crate::lang::syntax::loc(&t, manifest.language))
        .sum();
    if source == 0 {
        return None;
    }
    let rust: usize = port_files(crate_dir)
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .map(|t| crate::lang::syntax::code_lines(&t, &["//"]))
        .sum();
    let growth = rust as f64 / source as f64;
    let passed = rust as f64 <= MAX_GROWTH * source as f64 + ALLOWANCE;
    Some(Check {
        fix: None,
        name: "size",
        passed,
        detail: format!(
            "{} of Rust for {} of {} ({growth:.1}x){}",
            crate::report::count(rust, "line"),
            crate::report::count(source, "line"),
            manifest.language.name(),
            if passed {
                String::new()
            } else {
                format!(
                    "; over {MAX_GROWTH}x usually means code a crate already provides, or a case \
                     handled one by one: use the crates `analyze` lists and fold repeated code"
                )
            }
        ),
    })
}

/// Files a port must never read: what it is checked against.
const RECORDING_FILES: [&str; 3] = [
    CASES,
    crate::recording::KNOWN_DIFFS,
    crate::recording::RECORDED_FROM,
];
/// Shorter strings (`"error"`, `"none"`) are too common to say anything.
const MIN_STRING: usize = 8;
/// Numbers with fewer significant digits are too common to say anything.
const MIN_DIGITS: usize = 5;
/// Recorded values hard-coded in the port before `literals` fails.
const LITERALS_FAIL: usize = 3;
/// Where `gate` keeps the test counts of the last passing run.
const TESTS_BASELINE: &str = ".rust-anything/tests.json";

/// Rust source without comments, string literals emptied, with the literals it holds.
#[derive(Default)]
struct Scan {
    code: String,
    strings: Vec<(usize, String)>,
    numbers: Vec<(usize, String)>,
    /// Lines with a `// recorded-literal: <reason>` comment.
    marked: BTreeSet<usize>,
}

fn starts(chars: &[char], i: usize, s: &str) -> bool {
    s.chars()
        .enumerate()
        .all(|(k, c)| chars.get(i + k) == Some(&c))
}

/// Splits Rust source into code, string literals (unescaped) and number literals (without `_`
/// or a type suffix); with `stop_at_tests`, everything from the first `#[cfg(test)]` on is left
/// out.
fn scan_rust(text: &str, stop_at_tests: bool) -> Scan {
    let chars: Vec<char> = text.chars().collect();
    let ident = |c: char| c.is_alphanumeric() || c == '_';
    let digits = |chars: &[char], mut i: usize| {
        while chars
            .get(i)
            .is_some_and(|c| c.is_ascii_digit() || *c == '_')
        {
            i += 1;
        }
        i
    };
    let mut scan = Scan::default();
    let mut line = 1;
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        let free = i == 0 || !ident(chars[i - 1]);
        if stop_at_tests && starts(&chars, i, "#[cfg(test)]") {
            break;
        }
        if starts(&chars, i, "//") {
            let end = chars[i..]
                .iter()
                .position(|&c| c == '\n')
                .map_or(chars.len(), |p| i + p);
            if chars[i..end]
                .iter()
                .collect::<String>()
                .contains("recorded-literal:")
            {
                scan.marked.insert(line);
            }
            i = end;
            continue;
        }
        if starts(&chars, i, "/*") {
            let mut depth = 0usize;
            while i < chars.len() {
                if starts(&chars, i, "/*") {
                    depth += 1;
                    i += 2;
                } else if starts(&chars, i, "*/") {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    if chars[i] == '\n' {
                        line += 1;
                        scan.code.push('\n');
                    }
                    i += 1;
                }
            }
            continue;
        }
        let raw = match c {
            'r' if free => Some(i + 1),
            'b' | 'c' if free && chars.get(i + 1) == Some(&'r') => Some(i + 2),
            _ => None,
        }
        .and_then(|j| {
            let hashes = chars[j..].iter().take_while(|&&h| h == '#').count();
            (chars.get(j + hashes) == Some(&'"')).then_some((j + hashes + 1, hashes))
        });
        if let Some((start, hashes)) = raw {
            let close = format!("\"{}", "#".repeat(hashes));
            let end = (start..chars.len())
                .find(|&k| starts(&chars, k, &close))
                .unwrap_or(chars.len());
            let value: String = chars[start..end].iter().collect();
            scan.strings.push((line, value.clone()));
            line += value.matches('\n').count();
            scan.code.push_str("\"\"");
            i = end + close.len();
            continue;
        }
        let quoted = if c == '"' {
            Some(i + 1)
        } else if free && matches!(c, 'b' | 'c') && chars.get(i + 1) == Some(&'"') {
            Some(i + 2)
        } else {
            None
        };
        if let Some(mut j) = quoted {
            let at = line;
            let mut value = String::new();
            while j < chars.len() && chars[j] != '"' {
                if chars[j] != '\\' {
                    if chars[j] == '\n' {
                        line += 1;
                    }
                    value.push(chars[j]);
                    j += 1;
                    continue;
                }
                let escaped = chars.get(j + 1).copied().unwrap_or('\\');
                j += 2;
                match escaped {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '0' => value.push('\0'),
                    'x' => {
                        let hex: String = chars[j..(j + 2).min(chars.len())].iter().collect();
                        value.extend(u8::from_str_radix(&hex, 16).ok().map(char::from));
                        j += 2;
                    }
                    'u' if chars.get(j) == Some(&'{') => {
                        let end = (j..chars.len())
                            .find(|&k| chars[k] == '}')
                            .unwrap_or(chars.len());
                        let hex: String = chars[j + 1..end].iter().filter(|c| **c != '_').collect();
                        value.extend(u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32));
                        j = end + 1;
                    }
                    '\n' => {
                        line += 1;
                        while j < chars.len() && chars[j].is_whitespace() {
                            if chars[j] == '\n' {
                                line += 1;
                            }
                            j += 1;
                        }
                    }
                    other => value.push(other),
                }
            }
            scan.strings.push((at, value));
            scan.code.push_str("\"\"");
            i = j + 1;
            continue;
        }
        if c == '\'' {
            let end = if chars.get(i + 1) == Some(&'\\') {
                (i + 3..chars.len()).find(|&k| chars[k] == '\'')
            } else {
                (chars.get(i + 2) == Some(&'\'')).then_some(i + 2)
            };
            if let Some(end) = end {
                scan.code.push_str("' '");
                i = end + 1;
                continue;
            }
        }
        let after_range = i >= 2 && chars[i - 1] == '.' && chars[i - 2] == '.';
        if c.is_ascii_digit() && free && (i == 0 || chars[i - 1] != '.' || after_range) {
            let start = i;
            if c == '0' && matches!(chars.get(i + 1), Some('x' | 'o' | 'b')) {
                while i < chars.len() && ident(chars[i]) {
                    i += 1;
                }
                scan.code.extend(&chars[start..i]);
                continue;
            }
            i = digits(&chars, i);
            if chars.get(i) == Some(&'.') && chars.get(i + 1).is_some_and(char::is_ascii_digit) {
                i = digits(&chars, i + 1);
            }
            if matches!(chars.get(i), Some('e' | 'E')) {
                let sign = usize::from(matches!(chars.get(i + 1), Some('+' | '-')));
                if chars.get(i + 1 + sign).is_some_and(char::is_ascii_digit) {
                    i = digits(&chars, i + 1 + sign);
                }
            }
            let number: String = chars[start..i].iter().filter(|c| **c != '_').collect();
            while i < chars.len() && ident(chars[i]) {
                i += 1;
            }
            scan.code.extend(&chars[start..i]);
            scan.numbers.push((line, number));
            continue;
        }
        if c == '\n' {
            line += 1;
        }
        scan.code.push(c);
        i += 1;
    }
    scan
}

/// Digits of a number literal that carry information: no leading or trailing zeros.
fn significant_digits(number: &str) -> usize {
    let mantissa = number.split(['e', 'E']).next().unwrap_or(number);
    let digits: String = mantissa.chars().filter(char::is_ascii_digit).collect();
    digits.trim_start_matches('0').trim_end_matches('0').len()
}

/// Numbers any program may spell out: powers of two (and one off them), powers of ten, and the
/// float limits.
fn well_known(number: &str) -> bool {
    if number.chars().all(|c| c.is_ascii_digit()) {
        let Ok(n) = number.parse::<u128>() else {
            return false;
        };
        let mut ten = n;
        while ten >= 10 && ten % 10 == 0 {
            ten /= 10;
        }
        return ten == 1
            || n == u128::MAX
            || [Some(n), n.checked_add(1), n.checked_sub(1)]
                .into_iter()
                .flatten()
                .any(u128::is_power_of_two);
    }
    let Ok(v) = number.parse::<f64>() else {
        return false;
    };
    [
        f64::MAX,
        f64::MIN_POSITIVE,
        f64::EPSILON,
        f64::from(f32::MAX),
        f64::from(f32::MIN_POSITIVE),
        f64::from(f32::EPSILON),
    ]
    .iter()
    .any(|k| ((v - k) / k).abs() < 1e-6)
}

/// Strings and numbers the recording holds, arguments and results alike.
#[derive(Default)]
struct Recorded {
    strings: HashSet<String>,
    numbers: HashSet<u64>,
}

impl Recorded {
    fn add(&mut self, value: &serde_json::Value) {
        match value {
            serde_json::Value::String(s) if s.chars().count() >= MIN_STRING => {
                self.strings.insert(s.clone());
            }
            serde_json::Value::Number(n) => {
                if let Some(f) = n.as_f64() {
                    self.numbers.insert((f + 0.0).to_bits());
                }
            }
            serde_json::Value::Array(items) => items.iter().for_each(|v| self.add(v)),
            serde_json::Value::Object(o) => o
                .iter()
                .filter(|(k, _)| !matches!(k.as_str(), "error" | "error_type"))
                .for_each(|(_, v)| self.add(v)),
            _ => {}
        }
    }
}

fn recorded_values(crate_dir: &Path) -> Option<Recorded> {
    let text = std::fs::read_to_string(crate_dir.join(CASES)).ok()?;
    let mut recorded = Recorded::default();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let Ok(serde_json::Value::Object(case)) = serde_json::from_str(line) else {
            continue;
        };
        for (key, value) in &case {
            if !matches!(
                key.as_str(),
                "fn" | "class" | "module" | "error" | "error_type"
            ) {
                recorded.add(value);
            }
        }
    }
    Some(recorded)
}

/// Recorded values written into the port as literals although the original never spells them
/// out: a port that answers the recorded cases from a table passes `diff` without porting
/// anything. Fails on `LITERALS_FAIL` or more, or on any read of the recording itself; fewer
/// pass with their names. A literal on a line with `// recorded-literal: <reason>` is left out.
/// Recorded error text is left out too: the runtime often writes it (`division by zero`), and the
/// port has to reproduce it.
pub fn literals_check(crate_dir: &Path, manifest: &Manifest) -> Option<Check> {
    let recorded = recorded_values(crate_dir)?;
    let original: String = source_files(manifest)
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .collect::<Vec<_>>()
        .join("\n");
    let in_original = |text: &str| {
        original.contains(text)
            || serde_json::to_string(text)
                .is_ok_and(|j| original.contains(&j[1..j.len().saturating_sub(1)]))
    };
    let mut found: Vec<(String, String, String)> = Vec::new();
    let mut reads = Vec::new();
    for file in port_files(crate_dir) {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let scan = scan_rust(&text, true);
        let name = file
            .strip_prefix(crate_dir)
            .unwrap_or(&file)
            .to_string_lossy()
            .replace('\\', "/");
        let mut note = |key: String, shown: String, line: usize| {
            if !found.iter().any(|(k, _, _)| *k == key) {
                found.push((key, shown, format!("{name}:{line}")));
            }
        };
        for (line, s) in &scan.strings {
            if RECORDING_FILES.iter().any(|f| s.contains(f)) {
                reads.push(format!("{name}:{line}"));
            }
            if scan.marked.contains(line) || !recorded.strings.contains(s) || in_original(s) {
                continue;
            }
            let shown: String = if s.chars().count() > 40 {
                s.chars().take(37).chain("...".chars()).collect()
            } else {
                s.clone()
            };
            note(
                format!("s{s}"),
                serde_json::to_string(&shown).unwrap_or_default(),
                *line,
            );
        }
        for (line, n) in &scan.numbers {
            if scan.marked.contains(line) || significant_digits(n) < MIN_DIGITS || well_known(n) {
                continue;
            }
            let Ok(v) = n.parse::<f64>() else {
                continue;
            };
            let bits = (v + 0.0).to_bits();
            if recorded.numbers.contains(&bits) && !original.contains(n.as_str()) {
                note(format!("n{bits}"), n.clone(), *line);
            }
        }
    }
    let listed = found
        .iter()
        .take(5)
        .map(|(_, shown, at)| format!("{shown} ({at})"))
        .chain((found.len() > 5).then(|| format!("{} more", found.len() - 5)))
        .collect::<Vec<_>>()
        .join(", ");
    let (passed, detail) = if !reads.is_empty() {
        (
            false,
            format!(
                "the port names the recording ({}) at {}: compute results, never look them up",
                RECORDING_FILES.join(", "),
                reads.join(", ")
            ),
        )
    } else if found.len() >= LITERALS_FAIL {
        (
            false,
            format!(
                "{} recorded values are written into the port, and the original spells out none \
                 of them: {listed}",
                found.len()
            ),
        )
    } else if !found.is_empty() {
        (
            true,
            format!(
                "{} written into the port that the original never spells out: {listed}; compute \
                 {} unless the original takes {} from a library (then mark the line \
                 `// recorded-literal: <reason>`)",
                crate::report::count(found.len(), "recorded value"),
                if found.len() == 1 { "it" } else { "them" },
                if found.len() == 1 { "it" } else { "them" },
            ),
        )
    } else {
        (true, "no recorded values written into the port".into())
    };
    Some(Check {
        fix: None,
        name: "literals",
        passed,
        detail,
    })
}

/// Tests and assertions in the crate (`src/` without the bindings, and `tests/`).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestCounts {
    #[serde(default)]
    pub tests: usize,
    #[serde(default)]
    pub asserts: usize,
    #[serde(default)]
    pub ignored: usize,
    #[serde(default)]
    pub should_panic: usize,
}

pub fn test_counts(crate_dir: &Path) -> TestCounts {
    let patterns = [
        r"#\[\s*(?:\w+::)*test\s*\]",
        r"\b(?:debug_)?assert(?:_\w+)?!",
        r"#\[\s*ignore\b",
        r"#\[\s*should_panic\b",
    ]
    .map(|p| regex::Regex::new(p).ok());
    let tests = crate_dir.join("tests");
    let mut files = port_files(crate_dir);
    if tests.is_dir() {
        files.extend(files_under(&tests, &["rs"]));
    }
    let mut n = [0usize; 4];
    for file in files {
        let Ok(text) = std::fs::read_to_string(&file) else {
            continue;
        };
        let code = scan_rust(&text, false).code;
        for (count, re) in n.iter_mut().zip(&patterns) {
            *count += re.as_ref().map_or(0, |re| re.find_iter(&code).count());
        }
    }
    let [tests, asserts, ignored, should_panic] = n;
    TestCounts {
        tests,
        asserts,
        ignored,
        should_panic,
    }
}

/// Saves the crate's test counts as the baseline the next `tests` check compares with.
pub fn save_tests_baseline(crate_dir: &Path) -> Result<()> {
    let path = crate_dir.join(TESTS_BASELINE);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(
        &path,
        serde_json::to_string_pretty(&test_counts(crate_dir))? + "\n",
    )?;
    Ok(())
}

/// Fails when tests or assertions were removed, or tests ignored, since the last passing gate:
/// weakening the tests is not a way to pass.
pub fn tests_check(crate_dir: &Path) -> Check {
    let now = test_counts(crate_dir);
    let counts = format!(
        "{}, {}",
        crate::report::count(now.tests, "test"),
        crate::report::count(now.asserts, "assertion")
    );
    let baseline: Option<TestCounts> = std::fs::read_to_string(crate_dir.join(TESTS_BASELINE))
        .ok()
        .and_then(|t| serde_json::from_str(&t).ok());
    let Some(before) = baseline else {
        return Check {
            fix: None,
            name: "tests",
            passed: true,
            detail: format!("{counts}; saved as the baseline once gate passes"),
        };
    };
    let mut worse = Vec::new();
    if now.tests < before.tests {
        worse.push(format!("tests {} -> {}", before.tests, now.tests));
    }
    if now.asserts < before.asserts {
        worse.push(format!("assertions {} -> {}", before.asserts, now.asserts));
    }
    if now.ignored > before.ignored {
        worse.push(format!("ignored {} -> {}", before.ignored, now.ignored));
    }
    Check {
        fix: None,
        name: "tests",
        passed: worse.is_empty(),
        detail: if worse.is_empty() {
            format!("{counts}, none removed since the last passing gate")
        } else {
            format!("{} since the last passing gate", worse.join(", "))
        },
    }
}

#[derive(Debug, Serialize)]
pub struct GateOutcome {
    pub passed: bool,
    pub checks: Vec<Check>,
}

/// Format the text output for a `GateOutcome`. Every check's detail begins in the same
/// column by padding check names to the longest check name.
pub fn text(outcome: &GateOutcome) -> String {
    use std::fmt::Write as _;
    let mut out = String::new();
    let width = outcome
        .checks
        .iter()
        .map(|c| c.name.len())
        .max()
        .unwrap_or(0);
    for check in &outcome.checks {
        let mark = if check.passed { "pass" } else { "FAIL" };
        let first = check.detail.lines().next().unwrap_or_default();
        let _ = writeln!(out, "{mark}  {:<width$} {first}", check.name);
        if !check.passed {
            // Continuation lines start in the detail's column: the mark, two spaces, the
            // padded name and one space.
            let indent = " ".repeat(mark.len() + 2 + width + 1);
            for line in check.detail.lines().skip(1) {
                let _ = writeln!(out, "{indent}{line}");
            }
            if let Some(fix) = &check.fix {
                let _ = writeln!(out, "{indent}fix: {fix}");
            }
        }
    }
    let _ = writeln!(
        out,
        "{}",
        if outcome.passed {
            "Gate passed."
        } else {
            "Gate failed."
        }
    );
    out
}

pub fn gate(crate_dir: &Path, node_args: &[String]) -> Result<GateOutcome> {
    let manifest = Manifest::load(crate_dir)?;
    let lockfile = lockfile_check(crate_dir);
    let mut checks = rust_checks(crate_dir, &crate::cache::target_for(crate_dir)?.dir);
    checks.extend(lockfile);
    checks.extend(advisories_check(crate_dir));
    checks.push(ported_check(crate_dir));

    checks.extend(size_check(crate_dir, &manifest));
    checks.extend(literals_check(crate_dir, &manifest));
    checks.push(tests_check(crate_dir));
    checks.extend(crate::error_paths::checks(crate_dir, &manifest));
    let built = harness::build(crate_dir);
    let built_ok = built.is_ok();
    let (build_detail, build_fix) = match built {
        Ok(path) => (format!("built {}", path.display()), None),
        Err(e) => (last_lines(&format!("{e:#}"), 15), spawn_fix(&e)),
    };
    checks.push(Check {
        fix: build_fix,
        name: "build",
        passed: built_ok,
        detail: build_detail,
    });

    if let Some(why) = crate::recording::stale(crate_dir, &manifest) {
        checks.push(Check {
            fix: None,
            name: "recording",
            passed: false,
            detail: why,
        });
    }
    if let Some(why) = crate::recording::cases_tampered(crate_dir) {
        checks.push(Check {
            fix: None,
            name: "recording",
            passed: false,
            detail: why,
        });
    }

    if let Some(line) = crate::recording::saved_coverage(crate_dir)
        .and_then(|c| crate::recording::coverage_line(&c))
    {
        checks.push(Check {
            fix: None,
            name: "coverage",
            passed: true,
            detail: line,
        });
    }

    if !crate_dir.join(CASES).exists() {
        checks.push(Check {
            fix: None,
            name: "behavior",
            passed: false,
            detail: "no recorded cases: run `rust-anything record` first".into(),
        });
    } else if built_ok {
        match harness::diff(&DiffOptions {
            crate_dir,
            module: None,
            ignore_key_order: false,
            float_tolerance: None,
            node_args,
        }) {
            Ok(d) => {
                checks.push(Check {
                    fix: None,
                    name: "api",
                    passed: d.missing.is_empty(),
                    detail: if d.missing.is_empty() {
                        format!(
                            "exports {}",
                            manifest
                                .exports
                                .iter()
                                .map(|f| f.name.clone())
                                .chain(manifest.classes.iter().map(|c| format!("class {}", c.name)))
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    } else {
                        format!("missing exports: {}", d.missing.join(", "))
                    },
                });
                checks.push(Check {
                    fix: None,
                    name: "behavior",
                    passed: d.failed == 0,
                    detail: if d.accepted > 0 {
                        format!(
                            "{}/{} recorded cases match ({} by {})",
                            d.passed,
                            d.total,
                            d.accepted,
                            crate::recording::KNOWN_DIFFS
                        )
                    } else {
                        format!("{}/{} recorded cases match", d.passed, d.total)
                    },
                });
                if !d.stale_known.is_empty() {
                    checks.push(Check {
                        fix: None,
                        name: "known diffs",
                        passed: false,
                        detail: format!(
                            "{} entr{} of {} no longer differ{}: remove {}",
                            d.stale_known.len(),
                            if d.stale_known.len() == 1 { "y" } else { "ies" },
                            crate::recording::KNOWN_DIFFS,
                            if d.stale_known.len() == 1 { "s" } else { "" },
                            d.stale_known
                                .iter()
                                .map(|e| format!(
                                    "{}({})",
                                    e["fn"].as_str().unwrap_or("?"),
                                    e["args"]
                                ))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    });
                }
            }
            Err(e) => checks.push(Check {
                fix: None,
                name: "behavior",
                passed: false,
                detail: format!("{e:#}"),
            }),
        }
    }

    for check in checks.iter_mut().filter(|c| !c.passed && c.fix.is_none()) {
        check.fix = fix_for(check, crate_dir);
    }
    let passed = checks.iter().all(|c| c.passed);
    if passed {
        save_tests_baseline(crate_dir)?;
    }
    Ok(GateOutcome { passed, checks })
}

fn lockfile_check(crate_dir: &Path) -> Option<Check> {
    if !crate_dir.join("Cargo.lock").exists() {
        return Some(Check {
            fix: None,
            name: "lockfile",
            passed: true,
            detail: "warning: no Cargo.lock yet; keep the one the build writes, so builds are reproducible".into(),
        });
    }
    let mut cmd = Command::new("cargo");
    cmd.args([
        "metadata",
        "--locked",
        "--offline",
        "--format-version",
        "1",
        "--manifest-path",
    ])
    .arg(crate_dir.join("Cargo.toml"));
    let out = crate::process::output(&mut cmd, crate::process::BUILD).ok()?;
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stale = !out.status.success() && stderr.contains("--locked");
    Some(Check {
        fix: None,
        name: "lockfile",
        passed: true,
        detail: if stale {
            "warning: Cargo.lock is out of date with Cargo.toml; keep the refreshed one the build wrote".into()
        } else {
            "Cargo.lock matches Cargo.toml".into()
        },
    })
}

fn advisories_check(crate_dir: &Path) -> Option<Check> {
    let manifest = crate_dir.join("Cargo.toml");
    let attempts: [(&str, Vec<std::ffi::OsString>); 2] = [
        (
            "cargo-deny",
            vec![
                "deny".into(),
                "--manifest-path".into(),
                manifest.clone().into(),
                "check".into(),
                "advisories".into(),
            ],
        ),
        (
            "cargo-audit",
            vec![
                "audit".into(),
                "--file".into(),
                crate_dir.join("Cargo.lock").into(),
            ],
        ),
    ];
    for (tool, args) in attempts {
        if !crate::setup::on_path(tool) {
            continue;
        }
        let mut cmd = Command::new("cargo");
        cmd.args(&args);
        let out = crate::process::output(&mut cmd, crate::process::BUILD).ok()?;
        let text = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        return Some(Check {
            fix: None,
            name: "advisories",
            passed: true,
            detail: if out.status.success() {
                format!("no known advisories ({tool})")
            } else {
                format!(
                    "warning: {tool} reports advisories\n{}",
                    last_lines(&text, 15)
                )
            },
        });
    }
    None
}

/// The checks that hold for any Rust crate, bindings or not: fmt, clippy (warnings are errors),
/// cargo test, and `unsafe` only with a `// SAFETY:` comment.
pub fn rust_checks(crate_dir: &Path, target_dir: &Path) -> Vec<Check> {
    let cargo = |args: &[&str]| -> (bool, String, Option<String>) {
        let mut cmd = Command::new("cargo");
        cmd.args(args)
            .arg("--manifest-path")
            .arg(crate_dir.join("Cargo.toml"))
            .env("CARGO_TARGET_DIR", target_dir);
        // Lint arguments go after `--`, so the manifest path must come before them.
        match crate::process::output(&mut cmd, crate::process::BUILD) {
            Ok(o) => (
                o.status.success(),
                last_lines(&String::from_utf8_lossy(&o.stderr), 15),
                None,
            ),
            Err(e) => (false, format!("{e:#}"), spawn_fix(&e)),
        }
    };
    let mut checks = Vec::new();

    let (ok, out, fix) = cargo(&["fmt", "--check"]);
    checks.push(Check {
        name: "fmt",
        passed: ok,
        detail: if ok {
            "formatted".into()
        } else if fix.is_some() {
            out
        } else {
            format!("run `cargo fmt`\n{out}")
        },
        fix,
    });

    let mut clippy = Command::new("cargo");
    clippy
        .args(["clippy", "--all-targets", "--manifest-path"])
        .arg(crate_dir.join("Cargo.toml"))
        .args(["--", "-D", "warnings"])
        .env("CARGO_TARGET_DIR", target_dir);
    let (ok, out, fix) = match crate::process::output(&mut clippy, crate::process::BUILD) {
        Ok(o) => (
            o.status.success(),
            last_lines(&String::from_utf8_lossy(&o.stderr), 15),
            None,
        ),
        Err(e) => (false, format!("{e:#}"), spawn_fix(&e)),
    };
    checks.push(Check {
        fix,
        name: "clippy",
        passed: ok,
        detail: if ok { "no warnings".into() } else { out },
    });

    let (ok, out, fix) = cargo(&["test", "--quiet"]);
    checks.push(Check {
        fix,
        name: "cargo test",
        passed: ok,
        detail: if ok { "passed".into() } else { out },
    });

    let (unexplained, explained) = unsafe_blocks(&crate_dir.join("src"));
    checks.push(Check {
        fix: None,
        name: "unsafe",
        passed: unexplained.is_empty(),
        detail: if unexplained.is_empty() {
            format!("{explained} unsafe uses, each with a `// SAFETY:` comment")
        } else {
            format!(
                "unsafe without a `// SAFETY:` comment: {}",
                unexplained.join(", ")
            )
        },
    });

    checks
}

/// No function in `src/port.rs` is still `unimplemented!`.
pub fn ported_check(crate_dir: &Path) -> Check {
    let path = crate_dir.join("src/port.rs");
    let port = match std::fs::read_to_string(&path) {
        Ok(port) => port,
        Err(error) => {
            return Check {
                fix: None,
                name: "ported",
                passed: false,
                detail: format!("cannot read {}: {error}", path.display()),
            };
        }
    };
    let todo = stubs(&port);
    Check {
        fix: None,
        name: "ported",
        passed: todo == 0,
        detail: if todo == 0 {
            "nothing in port.rs is `unimplemented!`, `todo!` or only a `panic!`".into()
        } else {
            format!(
                "{} in port.rs still `unimplemented!`, `todo!` or a function that only panics",
                crate::report::count(todo, "place")
            )
        },
    }
}

/// Stubs left in a port: `unimplemented!`, `todo!`, and functions whose whole body is a `panic!`.
fn stubs(port: &str) -> usize {
    let code: String = port
        .lines()
        .map(|l| l.split("//").next().unwrap_or_default())
        .collect::<Vec<_>>()
        .join("\n");
    let macros = code.matches("unimplemented!(").count() + code.matches("todo!(").count();
    let panics = regex::Regex::new(r"\bfn\s+\w+[^{;]*\{\s*panic!\([^;{}]*\)\s*;?\s*\}")
        .map(|re| re.find_iter(&code).count())
        .unwrap_or(0);
    macros + panics
}

/// `unsafe` uses in Rust sources, split into those without and with a `// SAFETY:` comment just above.
fn unsafe_blocks(src: &Path) -> (Vec<String>, usize) {
    let mut unexplained = Vec::new();
    let mut explained = 0;
    let mut stack = vec![src.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "rs") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            let lines: Vec<&str> = text.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let code = strip_code(line);
                let uses_unsafe = code
                    .split(|c: char| !(c.is_alphanumeric() || c == '_'))
                    .any(|word| word == "unsafe");
                if !uses_unsafe {
                    continue;
                }
                let has_safety = lines[..i]
                    .iter()
                    .rev()
                    .take_while(|l| l.trim_start().starts_with("//"))
                    .any(|l| l.contains("SAFETY:"));
                if has_safety {
                    explained += 1;
                } else {
                    unexplained.push(format!("{}:{}", path.display(), i + 1));
                }
            }
        }
    }
    (unexplained, explained)
}

fn last_lines(text: &str, n: usize) -> String {
    let lines: Vec<&str> = text.lines().filter(|l| !l.trim().is_empty()).collect();
    lines[lines.len().saturating_sub(n)..].join("\n")
}

/// Strips string and char literals and comments from a line of Rust source code so keyword
/// searches (like `unsafe`) only match real code, not strings or comments.
fn strip_code(line: &str) -> String {
    let bytes = line.as_bytes();
    let mut out = String::with_capacity(line.len());
    let mut i = 0;
    while i < bytes.len() {
        // Line comment: everything from here to end of line is ignored.
        if bytes[i..].starts_with(b"//") {
            break;
        }

        // Block comment: /* ... */
        if bytes[i..].starts_with(b"/*") {
            i += 2;
            while i < bytes.len() && !bytes[i..].starts_with(b"*/") {
                i += 1;
            }
            if i < bytes.len() {
                i += 2;
            }
            out.push(' ');
            continue;
        }

        // Raw string: optional 'b' or 'c', then 'r', zero or more '#', then '"'.
        let prev_is_ident = i > 0 && (bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'_');
        let mut j = i;
        if !prev_is_ident
            && (bytes[j] == b'b' || bytes[j] == b'c')
            && j + 1 < bytes.len()
            && bytes[j + 1] == b'r'
        {
            j += 1;
        }
        if !prev_is_ident && bytes[j] == b'r' {
            let mut k = j + 1;
            while k < bytes.len() && bytes[k] == b'#' {
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b'"' {
                let hashes = k - (j + 1);
                i = k + 1;
                while i < bytes.len() {
                    if bytes[i] == b'"' {
                        let rest = &bytes[i + 1..];
                        if rest.len() >= hashes && rest[..hashes].iter().all(|&b| b == b'#') {
                            i += 1 + hashes;
                            break;
                        }
                    }
                    i += 1;
                }
                out.push(' ');
                continue;
            }
        }

        // Standard string: optional 'b' or 'c', then '"'.
        let is_byte_or_c_str = !prev_is_ident
            && (bytes[i] == b'b' || bytes[i] == b'c')
            && i + 1 < bytes.len()
            && bytes[i + 1] == b'"';
        if bytes[i] == b'"' || is_byte_or_c_str {
            if is_byte_or_c_str {
                i += 1;
            }
            i += 1; // skip opening '"'
            while i < bytes.len() {
                if bytes[i] == b'\\' {
                    i += if i + 1 < bytes.len() { 2 } else { 1 };
                } else if bytes[i] == b'"' {
                    i += 1;
                    break;
                } else {
                    i += 1;
                }
            }
            out.push(' ');
            continue;
        }

        // Char literal: 'x', '\n', '\'', etc.
        let is_byte_char =
            !prev_is_ident && bytes[i] == b'b' && i + 1 < bytes.len() && bytes[i + 1] == b'\'';
        if bytes[i] == b'\'' || is_byte_char {
            let quote_pos = if is_byte_char { i + 1 } else { i };
            let rest = &line[quote_pos + 1..];
            if let Some(close_rel) = rest.find('\'')
                && close_rel <= 8
                && !rest[..close_rel].contains(' ')
            {
                i = quote_pos + 1 + close_rel + 1;
                out.push(' ');
                continue;
            }
        }

        out.push(bytes[i] as char);
        i += 1;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{CASES, Check, GateOutcome, fix_for, spawn_fix, stubs, unsafe_blocks};

    #[test]
    fn counts_every_kind_of_stub() {
        let port = "pub fn a() -> u8 { todo!() }\n\
                    pub fn b() -> u8 { unimplemented!(\"later\") }\n\
                    pub fn c(x: u8) -> u8 {\n    panic!(\"not ported\")\n}\n\
                    pub fn d(x: u8) -> u8 { if x > 1 { panic!(\"big\") } x }\n\
                    // todo!() in a comment\n";
        assert_eq!(stubs(port), 3);
        assert_eq!(stubs("pub fn ok() -> u8 { 1 }"), 0);
    }

    #[test]
    fn failed_checks_say_what_to_run() {
        let check = |name, detail: &str| Check {
            fix: None,
            name,
            passed: false,
            detail: detail.into(),
        };
        let dir = std::path::Path::new("port");
        assert_eq!(
            fix_for(&check("recording", "changed"), dir).as_deref(),
            Some("rust-anything record port")
        );
        assert_eq!(
            fix_for(&check("behavior", "no recorded cases: run"), dir).as_deref(),
            Some("rust-anything record port")
        );
        assert!(fix_for(&check("fmt", ""), dir).is_some_and(|f| f.starts_with("cargo fmt")));
        assert_eq!(fix_for(&check("coverage", ""), dir), None);

        let mut missing_cmd = std::process::Command::new("cargo");
        missing_cmd.env_remove("PATH");
        let missing_err = crate::process::spawn_error(
            &missing_cmd,
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::NotFound, "not found"),
        );
        let missing_fix = spawn_fix(&missing_err).unwrap();
        assert!(
            !missing_fix.contains("fix the compile error above"),
            "{missing_fix}"
        );
        assert!(missing_fix.contains("cargo"), "{missing_fix}");
        assert!(
            missing_fix.contains("rust-anything doctor"),
            "{missing_fix}"
        );
        assert!(!missing_fix.contains('\n'), "{missing_fix}");

        let denied_err = crate::process::spawn_error(
            &std::process::Command::new("cargo"),
            "cargo build",
            std::io::Error::new(std::io::ErrorKind::PermissionDenied, "denied"),
        );
        let spawn_hint = spawn_fix(&denied_err).unwrap();
        assert!(
            !spawn_hint.contains("fix the compile error above"),
            "{spawn_hint}"
        );
        assert!(spawn_hint.contains("rust-anything doctor"), "{spawn_hint}");

        let compile_err =
            anyhow::anyhow!("cargo build failed (exit status: 101)\n\"not found on PATH\"");
        assert!(spawn_fix(&compile_err).is_none());
        let compile_fix = fix_for(&check("build", &compile_err.to_string()), dir).unwrap();
        assert!(
            compile_fix.starts_with("fix the compile error above"),
            "{compile_fix}"
        );
    }

    #[test]
    fn requires_safety_comments() {
        let dir = std::env::temp_dir().join(format!("rust-anything-gate-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("a.rs"),
            "#![forbid(unsafe_code)]\n\
             // SAFETY: checked above\n\
             unsafe { x() }\n\
             let y = unsafe { z() };\n\
             // unsafe in a comment\n\
             let _err = Err(\"RangeError: unsafe input\".into());\n\
             let _raw = r#\"unsafe \"quoted\" string\"#;\n\
             let _with_real = \"a string with unsafe\"; unsafe { real_work() };\n",
        )
        .unwrap();
        let (unexplained, explained) = unsafe_blocks(&dir);
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(explained, 1);
        assert_eq!(unexplained.len(), 2);
        assert!(unexplained[0].ends_with("a.rs:4"), "{}", unexplained[0]);
        assert!(unexplained[1].ends_with("a.rs:8"), "{}", unexplained[1]);
    }

    #[test]
    fn strip_code_removes_strings_chars_and_comments() {
        assert_eq!(super::strip_code("// unsafe comment").trim(), "");
        assert_eq!(
            super::strip_code("let s = \"unsafe inside string\";"),
            "let s =  ;"
        );
        assert_eq!(
            super::strip_code("let s = r#\"unsafe in raw\"#;"),
            "let s =  ;"
        );
        assert_eq!(
            super::strip_code("let s = \"unsafe\"; unsafe { }"),
            "let s =  ; unsafe { }"
        );
        assert_eq!(
            super::strip_code("let s = \"http://example.com\"; unsafe { }"),
            "let s =  ; unsafe { }"
        );
    }

    #[test]
    fn missing_port_source_fails_the_ported_check() {
        let dir = crate::app::temp_dir("gate-port").unwrap();
        let check = super::ported_check(&dir);
        std::fs::remove_dir(dir).unwrap();
        assert!(!check.passed);
        assert!(check.detail.contains("cannot read"));
        assert!(check.detail.contains("port.rs"));
    }

    #[test]
    fn ported_check_counts_one_place_and_several_places() {
        let detail = |port: &str| {
            let dir = crate::app::temp_dir("gate-ported").unwrap();
            std::fs::create_dir_all(dir.join("src")).unwrap();
            std::fs::write(dir.join("src/port.rs"), port).unwrap();
            let check = super::ported_check(&dir);
            std::fs::remove_dir_all(dir).unwrap();
            assert!(!check.passed);
            check.detail
        };
        let one = detail("pub fn a() -> i32 { todo!() }\n");
        assert!(one.starts_with("1 place in port.rs "), "{one}");
        let two = detail("pub fn a() -> i32 { todo!() }\npub fn b() -> i32 { unimplemented!() }\n");
        assert!(two.starts_with("2 places in port.rs "), "{two}");
    }

    #[test]
    fn scan_finds_literals_outside_comments_and_tests() {
        let port = "// \"a comment string\" 123456\n\
                    /* 987654 /* nested */ \"no\" */\n\
                    pub fn f(x: &str) -> u64 { if x == \"tab\\there\\u{e9}\" { 1_234_567u64 } else { 0x123456 } }\n\
                    pub fn g<'a>(c: char) -> &'a str { if c == '\"' { r#\"say \"hi\" now\"# } else { b\"bytes!!!\"; \"x\" } }\n\
                    pub fn h() -> f64 { 3.14159e-2 + (0..100000).count() as f64 } // recorded-literal: pi\n\
                    pub fn i() -> f64 { t.0 + 2.5f32 as f64 }\n\
                    #[cfg(test)]\nmod tests { const X: &str = \"only in tests\"; }\n";
        let scan = super::scan_rust(port, true);
        let strings: Vec<(usize, &str)> =
            scan.strings.iter().map(|(l, s)| (*l, s.as_str())).collect();
        assert_eq!(
            strings,
            [
                (3, "tab\there\u{e9}"),
                (4, "say \"hi\" now"),
                (4, "bytes!!!"),
                (4, "x")
            ]
        );
        let numbers: Vec<(usize, &str)> =
            scan.numbers.iter().map(|(l, n)| (*l, n.as_str())).collect();
        assert_eq!(
            numbers,
            [
                (3, "1234567"),
                (5, "3.14159e-2"),
                (5, "0"),
                (5, "100000"),
                (6, "2.5")
            ]
        );
        assert!(scan.marked.contains(&5) && scan.marked.len() == 1);
        assert!(!scan.code.contains("comment") && !scan.code.contains("only in tests"));
        let all = super::scan_rust(port, false);
        assert_eq!(all.strings.last().unwrap().1, "only in tests");
    }

    #[test]
    fn common_numbers_are_never_suspicious() {
        use super::{significant_digits, well_known};
        assert_eq!(significant_digits("1234500"), 5);
        assert_eq!(significant_digits("0.00012345"), 5);
        assert_eq!(significant_digits("1.25e10"), 3);
        for n in [
            "65536",
            "65535",
            "4294967295",
            "2147483647",
            "9007199254740991",
            "18446744073709551615",
            "1000000",
            "1.7976931348623157e308",
            "3.4028235e38",
            "2.220446049250313e-16",
        ] {
            assert!(well_known(n), "{n}");
        }
        for n in ["12345", "4294967299", "3.14159", "99999"] {
            assert!(!well_known(n), "{n}");
        }
    }

    #[test]
    fn literals_leave_out_recorded_error_text() {
        let dir = crate::app::temp_dir("gate-literals-errors").unwrap();
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::write(dir.join("words.py"), "def ratio(a, b):\n    return a / b\n").unwrap();
        std::fs::write(
            dir.join(CASES),
            "{\"fn\":\"ratio\",\"args\":[1,0],\"error\":\"ZeroDivisionError: division by zero\",\"error_type\":\"ZeroDivisionError\"}\n\
             {\"class\":\"Ratio\",\"ctor\":[0],\"calls\":[],\"results\":[{\"error\":\"OverflowError: stack is full\"}]}\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("src/port.rs"),
            "pub fn ratio(a: f64, b: f64) -> Result<f64, String> {\n    if b == 0.0 { return Err(\"ZeroDivisionError: division by zero\".into()) }\n    if a > 1e9 { return Err(\"OverflowError: stack is full\".into()) }\n    Ok(a / b)\n}\n",
        )
        .unwrap();
        let manifest: crate::bridge::Manifest = serde_json::from_value(serde_json::json!({
            "schema": crate::bridge::SCHEMA,
            "root": dir,
            "unit": "words.py",
            "crate_name": "demo",
            "kind": "pyo3",
            "exports": [],
        }))
        .unwrap();
        let check = super::literals_check(&dir, &manifest).unwrap();
        assert!(check.passed, "{check:?}");
        assert_eq!(check.detail, "no recorded values written into the port");
    }

    #[test]
    fn literals_fail_from_three_recorded_values_the_original_does_not_spell() {
        let dir = crate::app::temp_dir("gate-literals").unwrap();
        std::fs::create_dir_all(dir.join("src")).unwrap();
        std::fs::write(
            dir.join("lib.ts"),
            "export const greet = (n: string) => `hello there, ${n}` + \"spelled out\" + 31337\n",
        )
        .unwrap();
        std::fs::write(
            dir.join(CASES),
            "{\"fn\":\"greet\",\"args\":[\"somebody else\"],\"ok\":\"hello there, somebody else\"}\n\
             {\"fn\":\"greet\",\"args\":[\"x\"],\"ok\":[\"spelled out\",\"second answer\",31337,271828,\"third answer\"]}\n",
        )
        .unwrap();
        let manifest: crate::bridge::Manifest = serde_json::from_value(serde_json::json!({
            "schema": crate::bridge::SCHEMA,
            "root": dir,
            "unit": "lib.ts",
            "crate_name": "demo",
            "kind": "napi",
            "exports": [],
        }))
        .unwrap();
        let check = |port: &str| {
            std::fs::write(dir.join("src/port.rs"), port).unwrap();
            super::literals_check(&dir, &manifest).unwrap()
        };
        let honest = check(
            "pub fn greet(n: &str) -> String { format!(\"hello there, {n}\") + \"spelled out\" }\n",
        );
        assert!(honest.passed, "{honest:?}");
        assert_eq!(honest.detail, "no recorded values written into the port");

        let two = check(
            "pub fn greet(n: &str) -> String {\n    if n == \"somebody else\" { return \"second answer\".into() }\n    String::new()\n}\n",
        );
        assert!(two.passed, "{two:?}");
        assert!(
            two.detail.starts_with("2 recorded values"),
            "{}",
            two.detail
        );
        assert!(
            two.detail.contains("\"somebody else\" (src/port.rs:2)"),
            "{}",
            two.detail
        );

        let three = check(
            "pub fn greet(n: &str) -> String {\n    if n == \"somebody else\" { return \"second answer\".into() }\n    let _ = 271_828;\n    String::new()\n}\n",
        );
        assert!(!three.passed, "{three:?}");
        assert!(
            three.detail.contains("271828 (src/port.rs:3)"),
            "{}",
            three.detail
        );

        let marked = check(
            "pub fn greet(n: &str) -> String {\n    if n == \"somebody else\" { return \"second answer\".into() }\n    let _ = 271_828; // recorded-literal: e from a table\n    String::new()\n}\n",
        );
        assert!(marked.passed, "{marked:?}");

        let reads = check("const C: &str = include_str!(\"../cases.jsonl\");\n");
        assert!(!reads.passed, "{reads:?}");
        assert!(reads.detail.contains("src/port.rs:1"), "{}", reads.detail);
        std::fs::remove_dir_all(dir).unwrap();
    }

    #[test]
    fn size_check_says_1_line_for_singular_source() {
        let dir = crate::app::temp_dir("gate-size").unwrap();
        let src_dir = dir.join("src");
        std::fs::create_dir_all(&src_dir).unwrap();
        std::fs::write(src_dir.join("add.ts"), "export const add = 1;\n").unwrap();
        std::fs::write(src_dir.join("port.rs"), "pub fn add() -> i32 { 1 }\n").unwrap();
        let manifest: crate::bridge::Manifest = serde_json::from_value(serde_json::json!({
            "schema": crate::bridge::SCHEMA,
            "root": dir,
            "unit": "src",
            "crate_name": "demo",
            "kind": "napi",
            "exports": [],
        }))
        .unwrap();
        let check = super::size_check(&dir, &manifest).unwrap();
        std::fs::remove_dir_all(dir).unwrap();
        assert!(
            check
                .detail
                .contains("1 line of Rust for 1 line of TypeScript/JavaScript"),
            "{}",
            check.detail
        );
    }

    #[test]
    fn text_aligns_all_check_details_including_error_cases_and_known_diffs() {
        let outcome = GateOutcome {
            passed: true,
            checks: vec![
                Check {
                    name: "tests",
                    passed: true,
                    detail: "0 tests, 0 assertions; saved as the baseline once gate passes".into(),
                    fix: None,
                },
                Check {
                    name: "error cases",
                    passed: true,
                    detail: "the 1 function that fails in the original has a recorded failing case"
                        .into(),
                    fix: None,
                },
                Check {
                    name: "panics",
                    passed: true,
                    detail: "no unwrap, expect or panic where the original returns an error".into(),
                    fix: None,
                },
                Check {
                    name: "known diffs",
                    passed: true,
                    detail: "all recorded diffs match".into(),
                    fix: None,
                },
            ],
        };
        let text = super::text(&outcome);
        let lines: Vec<&str> = text.lines().take(4).collect();
        // Column where the detail starts (after "pass  " (6 chars) + 11-char name + " " (1 char) = 18 chars)
        let col = "pass  error cases ".len();
        assert_eq!(col, 18);
        for line in &lines {
            assert!(line.len() > col, "line too short: {line}");
            assert_eq!(
                &line[col..col + 2],
                match line.split_whitespace().nth(1).unwrap() {
                    "tests" => "0 ",
                    "error" => "th",
                    "panics" => "no",
                    "known" => "al",
                    other => panic!("unexpected check: {other}"),
                }
            );
        }
        assert!(text.contains("Gate passed."));
    }

    #[test]
    fn text_aligns_continuation_and_fix_lines_with_the_detail() {
        let outcome = GateOutcome {
            passed: false,
            checks: vec![
                Check {
                    name: "ported",
                    passed: false,
                    detail: "1 place in port.rs still `todo!`\nsecond line of the detail".into(),
                    fix: Some("port the functions still stubbed in src/port.rs".into()),
                },
                Check {
                    name: "error cases",
                    passed: true,
                    detail: "every failing function has a recorded failing case".into(),
                    fix: None,
                },
            ],
        };
        let text = super::text(&outcome);
        let col = "FAIL  error cases ".len();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(&lines[0][col..], "1 place in port.rs still `todo!`");
        assert_eq!(
            lines[1],
            format!("{}second line of the detail", " ".repeat(col))
        );
        assert_eq!(
            lines[2],
            format!(
                "{}fix: port the functions still stubbed in src/port.rs",
                " ".repeat(col)
            )
        );
        assert_eq!(
            &lines[3][col..],
            "every failing function has a recorded failing case"
        );
        assert_eq!(lines[4], "Gate failed.");
    }
}
