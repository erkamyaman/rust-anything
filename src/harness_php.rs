use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::bridge::Manifest;
use crate::harness::{BenchRow, CASES, DiffOutcome};
use crate::lang::php;
use crate::ts::discover::FileKind;
use crate::walk::Canonical;

const WORK_DIR: &str = ".rust-anything";

fn run(cmd: &mut Command, what: &str) -> Result<()> {
    let _scratch = crate::harness::scratch_cwd(cmd)?;
    let output = cmd.output().with_context(|| format!("running {what}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let tail: Vec<&str> = stderr
            .lines()
            .chain(stdout.lines())
            .rev()
            .take(30)
            .collect();
        bail!(
            "{what} failed ({}):\n{}",
            output.status,
            tail.into_iter().rev().collect::<Vec<_>>().join("\n")
        );
    }
    Ok(())
}

fn project_types(m: &Manifest, ported: &HashSet<&str>) -> Result<BTreeMap<String, PathBuf>> {
    let mut types = BTreeMap::new();
    for file in php::discover(&m.root)?.files {
        if file.kind != FileKind::Source || ported.contains(file.path.as_str()) {
            continue;
        }
        let path = m.root.join(file.path);
        let source = std::fs::read(&path)
            .with_context(|| format!("reading PHP source {}", path.display()))?;
        for name in php::declared_types(&String::from_utf8_lossy(&source)) {
            types
                .entry(name.to_ascii_lowercase())
                .or_insert_with(|| path.clone());
        }
    }
    Ok(types)
}

fn driver_command(
    crate_dir: &Path,
    m: &Manifest,
    mode: &str,
    cases: &Path,
    out: &Path,
    min_ms: u64,
    rust: bool,
) -> Result<Command> {
    let work = crate_dir.join(WORK_DIR);
    std::fs::create_dir_all(&work)?;
    let script = work.join("driver.php");
    std::fs::write(&script, crate::codegen_php::driver())?;
    let targets = work.join("targets.json");
    std::fs::write(&targets, crate::codegen_php::targets(m).to_string())?;
    let originals = m
        .exports
        .iter()
        .map(|f| f.file.as_str())
        .chain(m.js_only.iter().map(|j| j.file.as_str()));
    let mut cmd = Command::new("php");
    cmd.args(["-d", "memory_limit=-1", "-d", "display_errors=stderr"]);
    let files = if rust {
        let ported = originals.collect();
        let types = project_types(m, &ported)?;
        let classmap = work.join("php-classes.json");
        std::fs::write(&classmap, serde_json::to_vec(&types)?)?;
        cmd.args(["-d", "ffi.enable=1"])
            .env_remove("RUST_ANYTHING_PHP_AUTOLOAD")
            .env("RUST_ANYTHING_PHP_CLASSMAP", classmap);
        vec![
            crate_dir
                .join(crate::codegen_php::wrapper_path(m))
                .canonical()?,
        ]
    } else {
        cmd.env_remove("RUST_ANYTHING_PHP_CLASSMAP");
        let autoload = m.root.join("vendor/autoload.php");
        if autoload.is_file() {
            cmd.env("RUST_ANYTHING_PHP_AUTOLOAD", autoload);
        }
        let mut files: Vec<_> = originals.map(|file| m.root.join(file)).collect();
        files.sort();
        files.dedup();
        files
    };
    cmd.arg(&script)
        .arg(mode)
        .arg(&targets)
        .arg(cases)
        .arg(out)
        .arg(min_ms.to_string())
        .args(files);
    Ok(cmd)
}

fn driver(
    crate_dir: &Path,
    m: &Manifest,
    mode: &str,
    cases: &Path,
    out: &Path,
    min_ms: u64,
    rust: bool,
) -> Result<()> {
    let mut cmd = driver_command(crate_dir, m, mode, cases, out, min_ms, rust)?;
    run(&mut cmd, &format!("php driver ({mode})"))
}

pub fn record(crate_dir: &Path, m: &Manifest, cases: &[Value]) -> Result<PathBuf> {
    let out = crate_dir.join(CASES);
    let pending = crate_dir.join(WORK_DIR).join("recording.jsonl");
    crate::harness::record_cases(
        cases,
        &crate_dir.join(WORK_DIR),
        &pending,
        "php driver (record)",
        |inputs, part| driver_command(crate_dir, m, "record", inputs, part, 0, false),
    )?;
    let recorded: std::collections::HashSet<String> = std::fs::read_to_string(&pending)?
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l).ok())
        .filter_map(|c| c["fn"].as_str().map(str::to_string))
        .collect();
    let mut asked: Vec<&str> = cases.iter().filter_map(|c| c["fn"].as_str()).collect();
    asked.sort_unstable();
    asked.dedup();
    if let Some(name) = asked.into_iter().find(|name| !recorded.contains(*name)) {
        bail!(
            "cannot encode recorded result for {name}: no case could be written as JSON \
             (a binary string result?); kept the previous {CASES}"
        );
    }
    std::fs::rename(&pending, &out).with_context(|| format!("publishing {}", out.display()))?;
    Ok(out)
}

fn need_cases(crate_dir: &Path) -> Result<PathBuf> {
    let cases = crate_dir.join(CASES);
    if !cases.exists() {
        bail!(
            "no {CASES} in {} (run `rust-anything record` first)",
            crate_dir.display()
        );
    }
    Ok(cases)
}

pub fn built_file(crate_dir: &Path, m: &Manifest) -> Option<PathBuf> {
    let lib = m.lib_name();
    [
        format!("lib{lib}.dylib"),
        format!("lib{lib}.so"),
        format!("{lib}.dll"),
    ]
    .into_iter()
    .map(|n| crate_dir.join("php").join(n))
    .find(|p| p.exists())
}

pub fn diff(crate_dir: &Path, m: &Manifest) -> Result<DiffOutcome> {
    let cases = need_cases(crate_dir)?;
    if built_file(crate_dir, m).is_none() {
        bail!(
            "no built library in {} (run `rust-anything build {}` first)",
            crate_dir.join("php").display(),
            crate_dir.display()
        );
    }
    let out = crate_dir.join(WORK_DIR).join("diff.json");
    driver(crate_dir, m, "diff", &cases, &out, 0, true)?;
    Ok(serde_json::from_str(&std::fs::read_to_string(&out)?)?)
}

pub fn bench(crate_dir: &Path, m: &Manifest, min_ms: u64) -> Result<Vec<BenchRow>> {
    let cases = need_cases(crate_dir)?;
    let mut timings = Vec::new();
    for rust in [false, true] {
        let out = crate_dir.join(WORK_DIR).join(if rust {
            "bench-rust.json"
        } else {
            "bench-original.json"
        });
        driver(crate_dir, m, "bench", &cases, &out, min_ms, rust)?;
        timings.push(serde_json::from_str(&std::fs::read_to_string(&out)?)?);
    }
    Ok(crate::harness::bench_rows(m, &timings[0], &timings[1]))
}

pub fn build(crate_dir: &Path, m: &Manifest) -> Result<PathBuf> {
    let lib = crate::harness_c::build(crate_dir, m)?;
    let name = lib.file_name().context("library has no file name")?;
    std::fs::create_dir_all(crate_dir.join("php"))?;
    let target = crate_dir.join("php").join(name);
    std::fs::copy(&lib, &target)?;
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lang::Lang;

    const SEP: &str = "\\";

    fn manifest(root: &Path) -> Manifest {
        Manifest {
            schema: String::new(),
            root: root.to_path_buf(),
            root_written: None,
            unit: "src/Text.php".into(),
            language: Lang::Php,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "text-tools".into(),
            exports: Vec::new(),
            classes: Vec::new(),
            js_only: Vec::new(),
            batches: Vec::new(),
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        }
    }

    fn write(root: &Path, rel: &str, text: &str) {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }

    #[test]
    fn names_are_lower_cased_and_the_ported_file_is_left_out() {
        let root = crate::app::temp_dir("php-types-a").unwrap();
        write(&root, "src/Text.php", "<?php\nclass TextTools {}\n");
        write(
            &root,
            "src/Shapes.php",
            "<?php\ninterface Drawable {}\ntrait HasArea {}\nclass Circle {}\n",
        );
        let ported: HashSet<&str> = HashSet::from(["src/Text.php"]);
        let got = project_types(&manifest(&root), &ported);
        std::fs::remove_dir_all(&root).ok();
        let got = got.unwrap();
        let names: Vec<&str> = got.keys().map(String::as_str).collect();
        assert_eq!(names, ["circle", "drawable", "hasarea"]);
        for path in got.values() {
            assert_eq!(path, &root.join("src/Shapes.php"));
        }
    }

    #[test]
    fn nothing_ported_keeps_every_file() {
        let root = crate::app::temp_dir("php-types-b").unwrap();
        write(&root, "src/Text.php", "<?php\nclass TextTools {}\n");
        write(&root, "src/Shapes.php", "<?php\nclass Circle {}\n");
        let got = project_types(&manifest(&root), &HashSet::new());
        std::fs::remove_dir_all(&root).ok();
        let got = got.unwrap();
        assert_eq!(got["texttools"], root.join("src/Text.php"));
        assert_eq!(got["circle"], root.join("src/Shapes.php"));
        assert_eq!(got.len(), 2);
    }

    #[test]
    fn namespaced_names_are_kept_whole_and_lower_cased() {
        let root = crate::app::temp_dir("php-types-c").unwrap();
        write(
            &root,
            "src/Text.php",
            "<?php\nnamespace App\\Util;\nclass TextTools {}\n",
        );
        let got = project_types(&manifest(&root), &HashSet::new());
        std::fs::remove_dir_all(&root).ok();
        let got = got.unwrap();
        let key = format!("app{SEP}util{SEP}texttools");
        assert_eq!(got.keys().collect::<Vec<_>>(), [&key]);
        assert_eq!(got[&key], root.join("src/Text.php"));
    }

    #[test]
    fn the_first_file_wins_when_two_declare_the_same_name() {
        let root = crate::app::temp_dir("php-types-d").unwrap();
        write(&root, "src/B.php", "<?php\nclass Shared {}\n");
        write(&root, "src/A.php", "<?php\nclass SHARED {}\n");
        let got = project_types(&manifest(&root), &HashSet::new());
        std::fs::remove_dir_all(&root).ok();
        let got = got.unwrap();
        assert_eq!(got.len(), 1);
        assert_eq!(got["shared"], root.join("src/A.php"));
    }

    #[test]
    fn test_files_and_non_php_files_are_skipped() {
        let root = crate::app::temp_dir("php-types-e").unwrap();
        write(&root, "src/Text.php", "<?php\nclass TextTools {}\n");
        write(
            &root,
            "tests/TextToolsTest.php",
            "<?php\nclass TextToolsTest {}\n",
        );
        write(&root, "notes.txt", "class NotPhp {}\n");
        let got = project_types(&manifest(&root), &HashSet::new());
        std::fs::remove_dir_all(&root).ok();
        let got = got.unwrap();
        assert_eq!(got.keys().collect::<Vec<_>>(), ["texttools"]);
    }

    #[test]
    fn an_empty_project_has_no_types() {
        let root = crate::app::temp_dir("php-types-f").unwrap();
        let got = project_types(&manifest(&root), &HashSet::new());
        std::fs::remove_dir_all(&root).ok();
        assert!(got.unwrap().is_empty());
    }
}
