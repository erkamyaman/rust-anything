//! `docs/types.md` lists, per language, what a source type becomes in `port.rs` and why a type is
//! refused. Every row is checked here against the mappings `bridge` uses, so the tables cannot
//! drift from the code.

use std::collections::HashMap;

use rust_anything::bridge::{RType, js_only_reason, param_type};
use rust_anything::codegen_go::unsigned_port_type;
use rust_anything::lang::{c, csharp, go, java, kotlin, php, powershell, python, ruby};
use rust_anything::ts::exports::{ExportedFn, Param};
use rust_anything::ts::jsdoc;

const DOC: &str = include_str!("../docs/types.md");

/// The sections with a table, by heading.
const LANGUAGES: &[&str] = &[
    "TypeScript",
    "JavaScript",
    "Python",
    "Go",
    "C",
    "Java",
    "Kotlin",
    "C#",
    "Ruby",
    "PHP",
    "PowerShell",
];

/// What the port makes of a type: its Rust type, or the reason it is refused. The reason is
/// `None` where it is given outside the type mapping (C and PowerShell parameters), so the row's
/// note is not compared.
type Port = Result<String, Option<String>>;

/// A dialect type as `bridge` takes it: refused for the reasons every language shares, else the
/// Rust type `port.rs` declares.
fn bridged(dialect: Result<String, String>) -> Port {
    let dialect = dialect.map_err(Some)?;
    let f = ExportedFn {
        returns: dialect.clone(),
        ..Default::default()
    };
    match js_only_reason(&f, &[]) {
        Some(reason) => Err(Some(reason)),
        None => Ok(RType::from_ts(&dialect).rust()),
    }
}

fn port(language: &str, source: &str) -> Port {
    match language {
        "TypeScript" => bridged(Ok(source.to_string())),
        "JavaScript" => {
            let jsdoc = source
                .strip_prefix('{')
                .and_then(|s| s.strip_suffix('}'))
                .expect("a JSDoc type is written in braces");
            // Each row is a `@param` tag on an untyped parameter, made into a parameter as
            // `exports.rs` does: `{T=}` makes it optional, and `{...T}` goes on a rest parameter
            // (`...xs`), which is optional too. A type the JSDoc reader cannot turn into a TS type
            // leaves the parameter untyped.
            let (ts_type, optional) =
                jsdoc::to_ts(jsdoc).unwrap_or_else(|| ("unknown".to_string(), false));
            let rest = jsdoc.starts_with("...");
            bridged(Ok(ts_type.clone())).map(|_| {
                param_type(&Param {
                    ts_type,
                    optional: optional || rest,
                    rest,
                    ..Default::default()
                })
                .rust()
            })
        }
        "Python" => bridged(Ok(python::to_ts_type(source))),
        "Go" => bridged(go::go_to_ts(source, &HashMap::new()))
            .map(|rust| unsigned_port_type(source).unwrap_or(rust)),
        "C" => c::scalar(source)
            .map(|(_, _, abi, _)| abi.to_string())
            .ok_or(None),
        "Java" => bridged(java::java_to_ts_param(source)),
        "Kotlin" => bridged(kotlin::kotlin_to_ts_param(source)),
        "C#" => bridged(csharp::csharp_to_ts(source)),
        "Ruby" => bridged(Ok(ruby::yard_to_ts(source))),
        "PHP" => bridged(Ok(php::php_to_ts(source))),
        "PowerShell" => {
            let ty = source
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
                .expect("a PowerShell type is written in brackets");
            powershell::ps_to_ts(ty).map_or(Err(None), |t| bridged(Ok(t)))
        }
        other => panic!("no mapping for the section `{other}`"),
    }
}

/// The text of each code span in a table cell.
fn code(cell: &str) -> Vec<&str> {
    cell.split('`').skip(1).step_by(2).collect()
}

/// A cell's text as a reason reads, for comparing: no backticks, single spaces.
fn plain(text: &str) -> String {
    text.replace('`', "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// The cells of a table row, with `\|` read as a `|` inside a cell.
fn cells(row: &str) -> Vec<String> {
    let row = row.trim().trim_start_matches('|').trim_end_matches('|');
    row.replace("\\|", "\0")
        .split('|')
        .map(|cell| cell.replace('\0', "|").trim().to_string())
        .collect()
}

/// Every table row, as (section, source cell, port cell, notes cell).
fn rows() -> Vec<(String, String, String, String)> {
    let mut section = String::new();
    let mut out = Vec::new();
    for line in DOC.lines().map(|l| l.trim_end_matches('\r')) {
        if let Some(heading) = line.strip_prefix("## ") {
            section = heading.trim().to_string();
            continue;
        }
        if !line.starts_with('|') || line.starts_with("|---") || line.starts_with("| Source type") {
            continue;
        }
        let row = cells(line);
        assert_eq!(row.len(), 3, "a row has three cells: {line}");
        let [source, port, notes] = <[String; 3]>::try_from(row).unwrap();
        out.push((section.clone(), source, port, notes));
    }
    out
}

#[test]
fn every_row_of_the_type_tables_matches_the_mapping() {
    let rows = rows();
    let mut wrong = Vec::new();
    for (section, source, port_cell, notes) in &rows {
        assert!(
            LANGUAGES.contains(&section.as_str()),
            "a table under `## {section}`: only the language sections have one"
        );
        let sources = code(source);
        assert!(!sources.is_empty(), "no source type in `{source}`");
        for source in sources {
            let got = port(section, source);
            let fine = if port_cell == "refused" {
                match &got {
                    Err(Some(reason)) => plain(reason).contains(&plain(notes)),
                    Err(None) => true,
                    Ok(_) => false,
                }
            } else {
                code(port_cell)
                    .first()
                    .is_some_and(|want| got.as_deref() == Ok(*want))
            };
            if !fine {
                wrong.push(format!(
                    "{section}: `{source}` is listed as {port_cell} ({notes}), the mapping gives {got:?}"
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "docs/types.md disagrees with the code:\n{}",
        wrong.join("\n")
    );
    for language in LANGUAGES {
        assert!(
            rows.iter().any(|(section, ..)| section == language),
            "docs/types.md has no table for {language}"
        );
    }
}

#[test]
fn a_refused_row_says_why() {
    for (section, source, port, notes) in rows() {
        if port == "refused" {
            assert!(
                !notes.is_empty(),
                "{section}: `{source}` is refused without a reason"
            );
        }
    }
}
