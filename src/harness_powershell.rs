use crate::walk::Canonical;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde_json::Value;

use crate::bridge::Manifest;
use crate::harness::{BenchRow, CASES, DiffOutcome};

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

fn files(crate_dir: &Path, m: &Manifest, rust: bool) -> Result<Vec<PathBuf>> {
    if rust {
        return Ok(vec![
            crate_dir
                .join(crate::codegen_powershell::wrapper_path(m))
                .canonical()?,
        ]);
    }
    let mut files: Vec<PathBuf> = m
        .exports
        .iter()
        .map(|f| m.root.join(&f.file))
        .chain(m.js_only.iter().map(|j| m.root.join(&j.file)))
        .collect();
    files.sort();
    files.dedup();
    Ok(files)
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
    let script = work.join("driver.ps1");
    std::fs::write(&script, crate::codegen_powershell::driver())?;
    let targets = work.join("targets.json");
    std::fs::write(&targets, crate::codegen_powershell::targets(m).to_string())?;
    let mut cmd = Command::new("pwsh");
    cmd.args(["-NoProfile", "-NonInteractive", "-File"]);
    if !rust {
        crate::enable::original_only(&mut cmd, m);
    }
    cmd.arg(&script)
        .arg(mode)
        .arg(&targets)
        .arg(cases)
        .arg(out)
        .arg(min_ms.to_string())
        .args(files(crate_dir, m, rust)?);
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
    run(&mut cmd, &format!("pwsh driver ({mode})"))
}

pub fn record(crate_dir: &Path, m: &Manifest, cases: &[Value]) -> Result<PathBuf> {
    let out = crate_dir.join(CASES);
    let pending = crate_dir.join(WORK_DIR).join("recording.jsonl");
    crate::harness::record_cases(
        cases,
        &crate_dir.join(WORK_DIR),
        &pending,
        "pwsh driver (record)",
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
             (a value JSON cannot hold?); kept the previous {CASES}"
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
    .map(|n| crate_dir.join("ps").join(n))
    .find(|p| p.exists())
}

pub fn diff(crate_dir: &Path, m: &Manifest) -> Result<DiffOutcome> {
    let cases = need_cases(crate_dir)?;
    if built_file(crate_dir, m).is_none() {
        bail!(
            "no built library in {} (run `rust-anything build {}` first)",
            crate_dir.join("ps").display(),
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
    std::fs::create_dir_all(crate_dir.join("ps"))?;
    let target = crate_dir.join("ps").join(name);
    std::fs::copy(&lib, &target)?;
    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bridge::JsOnly;
    use crate::lang::Lang;
    use crate::ts::exports::ExportedFn;

    fn export(name: &str, file: &str) -> ExportedFn {
        ExportedFn {
            name: name.into(),
            file: file.into(),
            ..Default::default()
        }
    }

    fn manifest(root: &Path, exports: Vec<ExportedFn>, js_only: Vec<JsOnly>) -> Manifest {
        Manifest {
            schema: String::new(),
            root: root.to_path_buf(),
            root_written: None,
            unit: "TextTools.psm1".into(),
            language: Lang::Powershell,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "text-tools".into(),
            exports,
            classes: Vec::new(),
            js_only,
            batches: Vec::new(),
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
        }
    }

    /// A fresh directory from `app::temp_dir`, removed when the guard goes out of scope.
    struct TempDir(PathBuf);

    impl TempDir {
        fn new(tag: &str) -> Self {
            TempDir(crate::app::temp_dir(tag).unwrap())
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            if let Err(e) = std::fs::remove_dir_all(&self.0)
                && !std::thread::panicking()
            {
                panic!("removing {}: {e}", self.0.display());
            }
        }
    }

    fn js_only(name: &str, file: &str) -> JsOnly {
        JsOnly {
            name: name.into(),
            file: file.into(),
            line: 1,
            reason: String::new(),
        }
    }

    #[test]
    fn original_files_are_joined_to_the_root_sorted_and_unique() {
        let root = PathBuf::from("proj");
        let m = manifest(
            &root,
            vec![export("b", "z/Tools.psm1"), export("a", "z/Tools.psm1")],
            vec![js_only("c", "a/Other.ps1")],
        );
        let got = files(Path::new("unused"), &m, false).unwrap();
        assert_eq!(
            got,
            vec![root.join("a/Other.ps1"), root.join("z/Tools.psm1")]
        );
    }

    #[test]
    fn original_files_with_no_functions_are_empty() {
        let m = manifest(Path::new("proj"), Vec::new(), Vec::new());
        assert!(files(Path::new("unused"), &m, false).unwrap().is_empty());
    }

    #[test]
    fn a_js_only_function_in_an_export_file_is_listed_once() {
        let root = PathBuf::from("proj");
        let m = manifest(
            &root,
            vec![export("a", "Tools.psm1")],
            vec![js_only("b", "Tools.psm1")],
        );
        let got = files(Path::new("unused"), &m, false).unwrap();
        assert_eq!(got, vec![root.join("Tools.psm1")]);
    }

    #[test]
    fn the_port_lists_only_the_generated_wrapper() {
        let tmp = TempDir::new("ps-files");
        let crate_dir = &tmp.0;
        let m = manifest(
            Path::new("proj"),
            vec![export("a", "Tools.psm1")],
            vec![js_only("b", "Other.ps1")],
        );
        let wrapper = crate_dir.join(crate::codegen_powershell::wrapper_path(&m));
        std::fs::create_dir_all(wrapper.parent().unwrap()).unwrap();
        std::fs::write(&wrapper, "# wrapper").unwrap();
        // A path through `child/..` is never canonical, so the result must have been canonicalized.
        std::fs::create_dir(crate_dir.join("child")).unwrap();
        let got = files(&crate_dir.join("child").join(".."), &m, true);
        let expected = wrapper.canonical().unwrap();
        assert_eq!(got.unwrap(), vec![expected]);
    }

    #[test]
    fn the_port_fails_when_the_wrapper_is_missing() {
        let tmp = TempDir::new("ps-nowrap");
        let m = manifest(Path::new("proj"), Vec::new(), Vec::new());
        assert!(files(&tmp.0, &m, true).is_err());
    }
}
