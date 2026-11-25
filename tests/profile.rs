//! Integration tests that run `profile` on real Python (cProfile) and Node (`--cpu-prof`)
//! workloads instead of hand-built profiler fixtures.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

use rust_anything::lang::Lang;
use rust_anything::profile;

// `profile::profile` keys its temp work folder by PID, so serialize calls in this test binary.
static PROFILE_LOCK: Mutex<()> = Mutex::new(());

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("rust-anything {name} {}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        TempDir(dir)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn python_bin() -> Option<&'static str> {
    for bin in ["python3", "python"] {
        if let Ok(out) = Command::new(bin).arg("--version").output()
            && out.status.success()
            && String::from_utf8_lossy(&out.stdout).starts_with("Python 3.")
        {
            return Some(bin);
        }
    }
    None
}

fn node_24_plus() -> bool {
    let Ok(out) = Command::new("node").arg("--version").output() else {
        return false;
    };
    if !out.status.success() {
        return false;
    }
    let ver = String::from_utf8_lossy(&out.stdout);
    ver.trim()
        .strip_prefix('v')
        .and_then(|v| v.split('.').next())
        .and_then(|m| m.parse::<u32>().ok())
        .is_some_and(|major| major >= 24)
}

#[test]
fn profile_python_workload_finds_hot_function() {
    let py = python_bin().expect("Python 3 is required for profiling integration tests");
    let tmp = TempDir::new("profile-py");
    std::fs::write(
        tmp.path().join("lib.py"),
        r#"def lev(a: str, b: str) -> int:
    dp = list(range(len(b) + 1))
    for i, ca in enumerate(a, 1):
        prev = dp[0]
        dp[0] = i
        for j, cb in enumerate(b, 1):
            cur = dp[j]
            dp[j] = prev if ca == cb else 1 + min(prev, dp[j], dp[j - 1])
            prev = cur
    return dp[-1]
"#,
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("app.py"),
        r#"from lib import lev

total = 0
for _ in range(3000):
    total += lev("kitten", "sitting")
assert total == 9000
"#,
    )
    .unwrap();

    let _guard = PROFILE_LOCK.lock().unwrap();
    let cmd = vec![py.to_string(), "app.py".to_string()];
    let report = profile::profile(tmp.path(), &cmd, Some(Lang::Python)).unwrap();
    assert_eq!(report.language, Lang::Python);

    let lev = report
        .hot
        .iter()
        .find(|h| h.name == "lev" && h.file.ends_with("lib.py"))
        .unwrap_or_else(|| panic!("expected `lev` in `lib.py`, got: {:?}", report.hot));
    assert!(
        lev.calls.is_some_and(|c| c >= 3000),
        "expected >= 3000 calls, got {:?}",
        lev.calls
    );
    let md = profile::markdown(&report);
    assert!(
        md.contains("rust-anything bridge"),
        "expected bridge command in markdown:\n{md}"
    );
}

#[test]
fn profile_node_workload_finds_hot_function() {
    if !node_24_plus() {
        return;
    }
    let tmp = TempDir::new("profile-node");
    std::fs::write(
        tmp.path().join("package.json"),
        r#"{"name":"profile-node-test","type":"module"}"#,
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("lib.js"),
        r#"export function lev(a, b) {
  const dp = Array.from({ length: b.length + 1 }, (_, i) => i);
  for (let i = 1; i <= a.length; i++) {
    let prev = dp[0];
    dp[0] = i;
    for (let j = 1; j <= b.length; j++) {
      const cur = dp[j];
      dp[j] = a[i - 1] === b[j - 1] ? prev : 1 + Math.min(prev, dp[j], dp[j - 1]);
      prev = cur;
    }
  }
  return dp[b.length];
}
"#,
    )
    .unwrap();
    std::fs::write(
        tmp.path().join("app.js"),
        r#"import { lev } from "./lib.js";

let total = 0;
for (let i = 0; i < 80000; i++) {
  total += lev("kitten", "sitting");
}
if (total !== 240000) throw new Error(`unexpected ${total}`);
"#,
    )
    .unwrap();

    let _guard = PROFILE_LOCK.lock().unwrap();
    let cmd = vec!["node".to_string(), "app.js".to_string()];
    let report = profile::profile(tmp.path(), &cmd, Some(Lang::Ts)).unwrap();
    assert_eq!(report.language, Lang::Ts);

    let lev = report
        .hot
        .iter()
        .find(|h| h.name == "lev" && h.file.ends_with("lib.js"))
        .unwrap_or_else(|| panic!("expected `lev` in `lib.js`, got: {:?}", report.hot));
    assert!(lev.total_ms > 0.0);
    let md = profile::markdown(&report);
    assert!(
        md.contains("rust-anything bridge"),
        "expected bridge command in markdown:\n{md}"
    );
}
