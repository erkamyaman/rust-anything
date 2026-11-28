//! Error paths: where the original fails (`throw`, `raise`, an `error` result), whether a
//! recorded case reaches each of them, and whether the port panics where the original failed
//! politely.

use std::collections::BTreeSet;
use std::ops::Range;
use std::path::Path;

use regex::Regex;
use serde_json::Value;

use crate::bridge::{Manifest, snake_case};
use crate::gate::Check;
use crate::lang::Lang;

/// The marker that allows a panic in `port.rs`: the original treats that state as impossible too.
pub const INFALLIBLE: &str = "INFALLIBLE:";
const PANICS: [&str; 5] = [
    ".unwrap()",
    ".expect(",
    "panic!(",
    "unreachable!(",
    "todo!(",
];

/// An export whose original fails on some input, and where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Raise {
    pub export: String,
    pub site: String,
}

/// A panic in `port.rs`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Panic {
    pub function: String,
    pub line: usize,
    pub what: &'static str,
}

fn indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

fn is_comment(lang: Lang, line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("//")
        || t.starts_with("/*")
        || t.starts_with('*')
        || (matches!(
            lang,
            Lang::Python | Lang::Ruby | Lang::Php | Lang::Powershell
        ) && t.starts_with('#'))
}

/// The lines of the definition that starts at `start`.
fn body(lang: Lang, lines: &[&str], start: usize) -> Range<usize> {
    let Some(first) = lines.get(start) else {
        return start..start;
    };
    match lang {
        Lang::Python => {
            let base = indent(first);
            let mut end = start + 1;
            while end < lines.len() && (lines[end].trim().is_empty() || indent(lines[end]) > base) {
                end += 1;
            }
            start..end
        }
        Lang::Ruby => {
            let base = indent(first);
            let end = (start + 1..lines.len())
                .find(|&i| lines[i].trim() == "end" && indent(lines[i]) == base)
                .map_or(lines.len(), |i| i + 1);
            start..end
        }
        _ => {
            let mut depth = 0i32;
            let mut opened = false;
            for (i, line) in lines.iter().enumerate().skip(start) {
                if is_comment(lang, line) {
                    continue;
                }
                let mut in_string: Option<char> = None;
                let mut prev = ' ';
                for c in line.chars() {
                    match in_string {
                        Some(q) if c == q && prev != '\\' => in_string = None,
                        Some(_) => {}
                        None if c == '"' || c == '`' => in_string = Some(c),
                        None if c == '{' => {
                            depth += 1;
                            opened = true;
                        }
                        None if c == '}' => depth -= 1,
                        None => {}
                    }
                    prev = c;
                }
                if opened && depth <= 0 {
                    return start..i + 1;
                }
                if !opened && i > start + 10 {
                    break;
                }
            }
            start..(start + 1).min(lines.len())
        }
    }
}

/// The line, inside `range`, where the function fails on purpose.
fn raise_line(lang: Lang, lines: &[&str], range: Range<usize>) -> Option<usize> {
    let word = |w: &str| Regex::new(&format!(r"\b{w}\b")).expect("valid regex");
    let find = |re: &Regex| {
        range
            .clone()
            .find(|&i| !is_comment(lang, lines[i]) && re.is_match(lines[i]))
    };
    match lang {
        Lang::Ts | Lang::Java | Lang::Kotlin | Lang::Php | Lang::Csharp | Lang::Powershell => {
            find(&word("throw"))
        }
        Lang::Python | Lang::Ruby => find(&word("raise")),
        Lang::Go => {
            let open = range
                .clone()
                .find(|&i| lines[i].contains('{'))
                .unwrap_or(range.start);
            let signature: String = lines[range.start..=open.min(lines.len() - 1)].join(" ");
            let head = signature.split('{').next().unwrap_or_default();
            let returns_error = Regex::new(r"\)\s*(?:\([^)]*\berror\s*\)|error)\s*$")
                .expect("valid regex")
                .is_match(head.trim_end());
            returns_error.then(|| {
                find(
                    &Regex::new(r"errors\.New|fmt\.Errorf|return [^\n]*\berr\b")
                        .expect("valid regex"),
                )
                .unwrap_or(range.start)
            })
        }
        Lang::C => None,
    }
}

/// Exports whose original fails on purpose somewhere, with the place.
pub fn raises(lang: Lang, root: &Path, exports: &[(String, String, u32)]) -> Vec<Raise> {
    let mut out = Vec::new();
    for (name, file, line) in exports {
        let Ok(text) = std::fs::read_to_string(root.join(file)) else {
            continue;
        };
        let lines: Vec<&str> = text.lines().collect();
        let start = (*line as usize).saturating_sub(1);
        if start >= lines.len() {
            continue;
        }
        if let Some(at) = raise_line(lang, &lines, body(lang, &lines, start)) {
            out.push(Raise {
                export: name.clone(),
                site: format!("{file}:{}", at + 1),
            });
        }
    }
    out
}

/// Names of the functions with at least one recorded case that failed.
pub fn recorded_errors(cases: &str) -> BTreeSet<String> {
    cases
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter(|c| c.get("error").is_some() || c.get("panic").is_some())
        .filter_map(|c| c["fn"].as_str().map(str::to_string))
        .collect()
}

/// `unwrap`, `expect` and panicking macros in `port.rs`, by function, outside tests and not
/// marked `// INFALLIBLE:` on the line or in the comment just above.
pub fn panics(port: &str) -> Vec<Panic> {
    let lines: Vec<&str> = port.lines().collect();
    let fn_name =
        Regex::new(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+([A-Za-z_][A-Za-z0-9_]*)")
            .expect("valid regex");
    let mut out = Vec::new();
    let mut current = String::new();
    for (i, line) in lines.iter().enumerate() {
        if line.trim_start().starts_with("#[cfg(test)]") {
            break;
        }
        if let Some(c) = fn_name.captures(line) {
            current = c[1].to_string();
        }
        let code = line.split("//").next().unwrap_or_default();
        let Some(what) = PANICS.iter().find(|p| code.contains(*p)) else {
            continue;
        };
        let marked = line.contains(INFALLIBLE)
            || lines[..i]
                .iter()
                .rev()
                .take_while(|l| l.trim_start().starts_with("//"))
                .any(|l| l.contains(INFALLIBLE));
        if !marked {
            out.push(Panic {
                function: current.clone(),
                line: i + 1,
                what,
            });
        }
    }
    out
}

/// The `error cases` and `panics` checks of `gate`.
pub fn checks(crate_dir: &Path, manifest: &Manifest) -> Vec<Check> {
    let exports: Vec<(String, String, u32)> = manifest
        .exports
        .iter()
        .map(|f| (f.name.clone(), f.file.clone(), f.line))
        .collect();
    let raised = raises(manifest.language, &manifest.root, &exports);
    let cases = std::fs::read_to_string(crate_dir.join(crate::harness::CASES)).unwrap_or_default();
    let failed = recorded_errors(&cases);
    let unreached: Vec<String> = raised
        .iter()
        .filter(|r| !failed.contains(&r.export))
        .map(|r| format!("{} fails at {}", r.export, r.site))
        .collect();
    let error_cases = Check {
        fix: None,
        name: "error cases",
        passed: unreached.is_empty(),
        detail: if raised.is_empty() {
            "the original fails on purpose nowhere".into()
        } else if unreached.is_empty() {
            if raised.len() == 1 {
                "the 1 function that fails in the original has a recorded failing case".into()
            } else {
                format!(
                    "each of the {} functions that fail in the original has a recorded failing case",
                    raised.len()
                )
            }
        } else {
            format!(
                "no recorded case reaches these error paths: add inputs that fail (record --inputs)\n{}",
                unreached.join("\n")
            )
        },
    };

    let port = std::fs::read_to_string(crate_dir.join("src/port.rs")).unwrap_or_default();
    let fails: BTreeSet<String> = manifest
        .exports
        .iter()
        .filter(|f| failed.contains(&f.name) || raised.iter().any(|r| r.export == f.name))
        .map(|f| snake_case(&f.name))
        .collect();
    let bad: Vec<String> = panics(&port)
        .into_iter()
        .filter(|p| fails.contains(&p.function))
        .map(|p| format!("src/port.rs:{} `{}` in {}", p.line, p.what, p.function))
        .collect();
    let panics_check = Check {
        fix: None,
        name: "panics",
        passed: bad.is_empty(),
        detail: if bad.is_empty() {
            "no unwrap, expect or panic where the original returns an error".into()
        } else {
            format!(
                "the original returns an error here, so the port must too (return Err), or mark a state the original also treats as impossible with `// {INFALLIBLE} <why>`\n{}",
                bad.join("\n")
            )
        },
    };
    vec![error_cases, panics_check]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(lang: Lang, file: &str, source: &str, exports: &[(&str, u32)]) -> Vec<String> {
        let dir = std::env::temp_dir().join(format!(
            "rust-anything-raises-{}-{file}",
            std::process::id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join(file), source).unwrap();
        let exports: Vec<(String, String, u32)> = exports
            .iter()
            .map(|(n, l)| (n.to_string(), file.to_string(), *l))
            .collect();
        let found = raises(lang, &dir, &exports)
            .into_iter()
            .map(|r| format!("{} {}", r.export, r.site))
            .collect();
        std::fs::remove_dir_all(&dir).unwrap();
        found
    }

    #[test]
    fn a_line_past_the_end_of_an_edited_file_is_skipped() {
        let source = "package textkit\n\nfunc Abbrev(s string) string {\n\treturn s\n}\n";
        for lang in [Lang::Go, Lang::Ts, Lang::Python] {
            assert!(check(lang, "short.go", source, &[("Humanize", 12)]).is_empty());
        }
    }

    #[test]
    fn finds_where_the_original_fails_in_each_language() {
        assert_eq!(
            check(
                Lang::Ts,
                "a.ts",
                "export function ok(x: number) {\n  return x\n}\nexport function parse(s: string) {\n  // throw in a comment\n  if (!s) { throw new Error('empty') }\n  return s\n}\n",
                &[("ok", 1), ("parse", 4)]
            ),
            ["parse a.ts:6"]
        );
        assert_eq!(
            check(
                Lang::Python,
                "a.py",
                "def ok(x):\n    return x\n\ndef parse(s):\n    if not s:\n        raise ValueError('empty')\n    return s\n",
                &[("ok", 1), ("parse", 4)]
            ),
            ["parse a.py:6"]
        );
        assert_eq!(
            check(
                Lang::Ruby,
                "a.rb",
                "module M\n  def self.ok(x)\n    x\n  end\n\n  def self.parse(s)\n    raise ArgumentError, 'empty' if s.empty?\n    s\n  end\nend\n",
                &[("ok", 2), ("parse", 6)]
            ),
            ["parse a.rb:7"]
        );
        assert_eq!(
            check(
                Lang::Go,
                "a.go",
                "package a\n\nfunc Ok(x int) int {\n\treturn x\n}\n\nfunc Parse(s string) (int, error) {\n\tif s == \"\" {\n\t\treturn 0, errors.New(\"empty\")\n\t}\n\treturn 1, nil\n}\n",
                &[("Ok", 3), ("Parse", 7)]
            ),
            ["Parse a.go:9"]
        );
        assert_eq!(
            check(
                Lang::Java,
                "A.java",
                "class A {\n  public static int ok(int x) { return x; }\n  public static int parse(String s) {\n    if (s.isEmpty()) throw new IllegalArgumentException(\"{\");\n    return 1;\n  }\n}\n",
                &[("ok", 2), ("parse", 3)]
            ),
            ["parse A.java:4"]
        );
        assert!(
            check(
                Lang::C,
                "a.c",
                "int parse(const char *s) { return s ? 1 : -1; }\n",
                &[("parse", 1)]
            )
            .is_empty()
        );
    }

    #[test]
    fn panics_outside_tests_and_markers_are_found() {
        let port = "pub fn parse_port(s: String) -> Result<i64, String> {\n    let n: i64 = s.parse().unwrap();\n    // INFALLIBLE: checked above\n    let m = Some(1).expect(\"one\");\n    Ok(n + m)\n}\n\nfn helper() -> i64 {\n    \"1\".parse().unwrap_or(0) + Some(2).unwrap() // INFALLIBLE: literal\n}\n\npub async fn later() {\n    todo!(\"x\")\n}\n\n#[cfg(test)]\nmod tests {\n    fn t() { None::<i32>.unwrap(); }\n}\n";
        let found: Vec<(String, usize, &str)> = panics(port)
            .into_iter()
            .map(|p| (p.function, p.line, p.what))
            .collect();
        assert_eq!(
            found,
            [
                ("parse_port".to_string(), 2, ".unwrap()"),
                ("later".to_string(), 13, "todo!(")
            ]
        );
        assert_eq!(
            recorded_errors(
                "{\"fn\":\"a\",\"args\":[],\"result\":1}\n{\"fn\":\"b\",\"args\":[],\"error\":{\"name\":\"E\"}}\n"
            ),
            BTreeSet::from(["b".to_string()])
        );
    }

    #[test]
    fn error_cases_detail_singular_and_plural() {
        let dir =
            std::env::temp_dir().join(format!("rust-anything-error-detail-{}", std::process::id()));
        let root = dir.join("root");
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&root).unwrap();
        std::fs::write(
            root.join("test.ts"),
            "export function parse(s: string) {\n  if (!s) throw new Error('err');\n  return s;\n}\nexport function second(s: string) {\n  if (!s) throw new Error('err2');\n  return s;\n}\n",
        )
        .unwrap();

        let mut manifest = Manifest {
            schema: String::new(),
            root: root.clone(),
            root_written: None,
            unit: "test.ts".into(),
            language: Lang::Ts,
            kind: crate::bridge::Kind::Napi,
            crate_name: "test-rs".into(),
            exports: vec![crate::ts::exports::ExportedFn {
                name: "parse".into(),
                file: "test.ts".into(),
                line: 1,
                ..Default::default()
            }],
            classes: Vec::new(),
            js_only: Vec::new(),
            batches: Vec::new(),
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        };

        // 1 function that fails: recorded
        std::fs::write(
            dir.join("cases.jsonl"),
            "{\"fn\":\"parse\",\"args\":[\"\"],\"error\":{\"name\":\"Error\"}}\n",
        )
        .unwrap();

        let checks_one = checks(&dir, &manifest);
        assert_eq!(checks_one[0].name, "error cases");
        assert!(checks_one[0].passed);
        assert_eq!(
            checks_one[0].detail,
            "the 1 function that fails in the original has a recorded failing case"
        );

        // 2 functions that fail: recorded
        manifest.exports.push(crate::ts::exports::ExportedFn {
            name: "second".into(),
            file: "test.ts".into(),
            line: 5,
            ..Default::default()
        });
        std::fs::write(
            dir.join("cases.jsonl"),
            "{\"fn\":\"parse\",\"args\":[\"\"],\"error\":{\"name\":\"Error\"}}\n{\"fn\":\"second\",\"args\":[\"\"],\"error\":{\"name\":\"Error\"}}\n",
        )
        .unwrap();

        let checks_two = checks(&dir, &manifest);
        assert_eq!(checks_two[0].name, "error cases");
        assert!(checks_two[0].passed);
        assert_eq!(
            checks_two[0].detail,
            "each of the 2 functions that fail in the original has a recorded failing case"
        );

        std::fs::remove_dir_all(&dir).unwrap();
    }
}
