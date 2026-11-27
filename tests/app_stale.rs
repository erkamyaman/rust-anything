//! Re-recording replaces the case set: a case the last recording wrote and this one did not is
//! removed, and a case that never came from a recording (written by hand) is kept and named.

mod common;

use std::path::{Path, PathBuf};

use rust_anything::app::{self, RecordOptions, ReplayOptions};

fn repo(tag: &str) -> PathBuf {
    let repo =
        std::env::temp_dir().join(format!("rust-anything-stale-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(&repo).unwrap();
    repo
}

fn record(repo: &Path, run: &[String], http: bool) -> app::RecordOutcome {
    record_within(repo, run, http, None)
}

fn record_within(
    repo: &Path,
    run: &[String],
    http: bool,
    timeout_secs: Option<u64>,
) -> app::RecordOutcome {
    app::record(&RecordOptions {
        repo,
        run,
        inputs: Some(&repo.join("in.jsonl")),
        normalize: &[],
        name: None,
        traps: false,
        timeout_secs,
        http: http.then_some(true),
        server_args: None,
        fresh: None,
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects: None,
        ready_secs: None,
        ready_request: None,
    })
    .unwrap()
}

fn replay(repo: &Path) -> app::ReplayOutcome {
    app::replay(&ReplayOptions {
        repo,
        build: None,
        timeout_secs: None,
        port: None,
        case: None,
    })
    .unwrap()
}

fn files(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn a_cli_case_dropped_from_the_inputs_is_removed_on_rerecord() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("cli");
    std::fs::write(
        repo.join("cli.py"),
        "import sys\nopen('out.txt', 'w').write(' '.join(sys.argv[1:]))\nprint(sys.argv[1:])\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n{\"name\":\"two\",\"args\":[\"b\"]}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    let first = record(&repo, &run, false);
    assert_eq!(first.cases, 2);
    let dir = app::cases_dir(&repo);
    assert!(dir.join("two.toml").is_file() && dir.join("two.out").is_dir());
    // A case written by hand, as the docs allow: never a recording's, so it stays.
    std::fs::copy(dir.join("one.toml"), dir.join("mine.toml")).unwrap();
    std::fs::copy(dir.join("one.stdout"), dir.join("mine.stdout")).unwrap();
    std::fs::copy(dir.join("one.stderr"), dir.join("mine.stderr")).unwrap();

    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n",
    )
    .unwrap();
    let second = record(&repo, &run, false);
    let left = files(&dir);
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.cases, 1);
    assert_eq!(second.stale, ["two"], "{second:?}");
    assert_eq!(second.kept, ["mine"], "{second:?}");
    assert!(
        !left.iter().any(|f| f.starts_with("two.")),
        "two's files are left: {left:?}"
    );
    assert!(left.contains(&"mine.toml".to_string()), "{left:?}");
    assert_eq!((again.passed, again.total), (2, 2), "{:?}", again.failures);
}

#[test]
fn a_kept_case_based_on_the_case_folder_does_not_keep_a_removed_case() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("dot-base");
    std::fs::write(repo.join("cli.py"), "import sys\nprint(sys.argv[1:])\n").unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n{\"name\":\"two\",\"args\":[\"b\"]}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    record(&repo, &run, false);
    let dir = app::cases_dir(&repo);
    let one = std::fs::read_to_string(dir.join("one.toml")).unwrap();
    let mine = one.replace("fs.sandbox = true", "fs.sandbox = true\nfs.base = \".\"");
    assert_ne!(mine, one, "{one}");
    std::fs::write(dir.join("mine.toml"), mine).unwrap();

    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n",
    )
    .unwrap();
    let second = record(&repo, &run, false);
    let left = files(&dir);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.stale, ["two"], "{second:?}");
    assert!(
        !left.iter().any(|f| f.starts_with("two.")),
        "two's files are left: {left:?}"
    );
}

const SERVER: &str = r#"import http.server, os
class H(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        body = ("at " + self.path).encode()
        self.send_response(200)
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)
    def log_message(self, *args):
        pass
http.server.HTTPServer(("127.0.0.1", int(os.environ["PORT"])), H).serve_forever()
"#;

#[test]
fn an_http_case_dropped_from_the_inputs_is_removed_on_rerecord() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("http");
    std::fs::write(repo.join("server.py"), SERVER).unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"path\":\"/one\"}\n{\"name\":\"two\",\"path\":\"/two\"}\n",
    )
    .unwrap();
    let run = [python.to_string(), "server.py".to_string()];
    let first = record(&repo, &run, true);
    assert_eq!(first.cases, 2);
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"path\":\"/one\"}\n",
    )
    .unwrap();
    let second = record(&repo, &run, true);
    let left = files(&second.dir);
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.stale, ["two"], "{second:?}");
    assert!(
        !left.iter().any(|f| f.starts_with("two.")),
        "two's files are left: {left:?}"
    );
    assert_eq!((again.passed, again.total), (1, 1), "{:?}", again.failures);
}

#[test]
fn a_hand_written_case_named_like_an_input_that_times_out_is_kept() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("timeout");
    std::fs::write(
        repo.join("cli.py"),
        "import sys, time\nif sys.argv[1:] == ['slow']:\n    time.sleep(30)\nprint(sys.argv[1:])\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    record(&repo, &run, false);
    let dir = app::cases_dir(&repo);
    // Written by hand: no recording ever wrote `slow`.
    std::fs::copy(dir.join("one.toml"), dir.join("slow.toml")).unwrap();
    std::fs::copy(dir.join("one.stdout"), dir.join("slow.stdout")).unwrap();
    std::fs::copy(dir.join("one.stderr"), dir.join("slow.stderr")).unwrap();

    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n{\"name\":\"slow\",\"args\":[\"slow\"]}\n",
    )
    .unwrap();
    let second = record_within(&repo, &run, false, Some(2));
    let left = files(&dir);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.timed_out, ["slow"], "{second:?}");
    assert!(second.stale.is_empty(), "{second:?}");
    assert!(left.contains(&"slow.toml".to_string()), "{left:?}");
    assert_eq!(second.kept, ["slow"], "{second:?}");
}

#[test]
fn files_a_kept_case_reads_stay_when_the_case_that_wrote_them_goes() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("base");
    std::fs::write(
        repo.join("cli.py"),
        "import os\nprint(open('x.txt').read() if os.path.exists('x.txt') else 'none')\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[]}\n{\"name\":\"two\",\"args\":[],\"files\":{\"x.txt\":\"from two\"}}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    record(&repo, &run, false);
    let dir = app::cases_dir(&repo);
    let two = std::fs::read_to_string(dir.join("two.toml")).unwrap();
    assert!(two.contains("two.in"), "{two}");
    // Written by hand, reading the files `two` was recorded with.
    std::fs::write(dir.join("mine.toml"), &two).unwrap();
    std::fs::copy(dir.join("two.stdout"), dir.join("mine.stdout")).unwrap();
    std::fs::copy(dir.join("two.stderr"), dir.join("mine.stderr")).unwrap();

    std::fs::write(repo.join("in.jsonl"), "{\"name\":\"one\",\"args\":[]}\n").unwrap();
    let second = record(&repo, &run, false);
    let left = files(&dir);
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.stale, ["two"], "{second:?}");
    assert!(!left.contains(&"two.toml".to_string()), "{left:?}");
    assert!(left.contains(&"two.in".to_string()), "{left:?}");
    assert_eq!((again.passed, again.total), (2, 2), "{:?}", again.failures);
}

#[test]
fn names_a_cli_recording_wrote_are_not_removed_from_http_cases() {
    let Some(python) = common::python() else {
        return;
    };
    // An HTTP case `two`, recorded elsewhere, to put in by hand.
    let other = repo("switch-other");
    std::fs::write(other.join("server.py"), SERVER).unwrap();
    std::fs::write(
        other.join("in.jsonl"),
        "{\"name\":\"two\",\"path\":\"/two\"}\n",
    )
    .unwrap();
    let server = [python.to_string(), "server.py".to_string()];
    let theirs = record(&other, &server, true).dir;

    let repo = repo("switch");
    std::fs::write(repo.join("cli.py"), "import sys\nprint(sys.argv[1:])\n").unwrap();
    std::fs::write(repo.join("server.py"), SERVER).unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n{\"name\":\"two\",\"args\":[\"b\"]}\n",
    )
    .unwrap();
    record(&repo, &[python.to_string(), "cli.py".to_string()], false);
    let http_dir = app::cases_dir(&repo).join("http");
    std::fs::create_dir_all(&http_dir).unwrap();
    for file in ["two.toml", "two.body"] {
        std::fs::copy(theirs.join(file), http_dir.join(file)).unwrap();
    }
    std::fs::remove_dir_all(&other).unwrap();

    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"path\":\"/one\"}\n",
    )
    .unwrap();
    let second = record(&repo, &server, true);
    let left = files(&http_dir);
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert!(second.stale.is_empty(), "{second:?}");
    assert_eq!(second.kept, ["two"], "{second:?}");
    assert!(left.contains(&"two.toml".to_string()), "{left:?}");
    assert_eq!((again.passed, again.total), (2, 2), "{:?}", again.failures);
}

#[test]
fn a_recorded_case_whose_input_times_out_now_is_removed() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("timeout-ours");
    let flag = repo.join("slow.flag");
    std::fs::write(
        repo.join("cli.py"),
        format!(
            "import os, sys, time\nif sys.argv[1:] == ['slow'] and os.path.exists({:?}):\n    time.sleep(30)\nprint(sys.argv[1:])\n",
            flag.display().to_string()
        ),
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[\"a\"]}\n{\"name\":\"slow\",\"args\":[\"slow\"]}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    let first = record(&repo, &run, false);
    assert_eq!(first.cases, 2, "{first:?}");
    std::fs::write(&flag, "").unwrap();
    let second = record_within(&repo, &run, false, Some(2));
    let left = files(&app::cases_dir(&repo));
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.timed_out, ["slow"], "{second:?}");
    assert_eq!(second.stale, ["slow"], "{second:?}");
    assert!(
        !left.iter().any(|f| f.starts_with("slow.")),
        "slow's files are left: {left:?}"
    );
}

#[test]
fn a_folder_inside_a_removed_case_s_files_that_a_kept_case_reads_stays() {
    a_kept_case_reads_inside_two_in("nested-base", "two.in/sub");
}

#[test]
fn a_base_spelled_through_the_parent_folder_is_resolved() {
    a_kept_case_reads_inside_two_in("parent-base", "../app-cases/two.in/sub");
}

/// `two` is recorded with `sub/x.txt`, a hand-written `mine` reads `sub` through `base`, and
/// `two` is dropped: its `.in/` must stay for `mine`.
fn a_kept_case_reads_inside_two_in(tag: &str, base: &str) {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo(tag);
    std::fs::write(
        repo.join("cli.py"),
        "import os\nprint(open('x.txt').read() if os.path.exists('x.txt') else 'none')\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[]}\n{\"name\":\"two\",\"args\":[],\"files\":{\"sub/x.txt\":\"from two\"}}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    record(&repo, &run, false);
    let dir = app::cases_dir(&repo);
    let two = std::fs::read_to_string(dir.join("two.toml")).unwrap();
    assert!(two.contains("\"two.in\""), "{two}");
    // Written by hand, reading a folder inside the files `two` was recorded with.
    std::fs::write(
        dir.join("mine.toml"),
        two.replace("\"two.in\"", &format!("{base:?}")),
    )
    .unwrap();
    std::fs::write(dir.join("mine.stdout"), "from two\n").unwrap();
    std::fs::copy(dir.join("two.stderr"), dir.join("mine.stderr")).unwrap();

    std::fs::write(repo.join("in.jsonl"), "{\"name\":\"one\",\"args\":[]}\n").unwrap();
    let second = record(&repo, &run, false);
    let kept = dir.join("two.in").join("sub").join("x.txt").is_file();
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.stale, ["two"], "{second:?}");
    assert!(kept, "two.in/sub/x.txt was removed");
    assert_eq!((again.passed, again.total), (2, 2), "{:?}", again.failures);
}

#[test]
fn a_removed_case_s_folder_that_a_kept_base_passes_through_stays() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("through-base");
    std::fs::write(
        repo.join("cli.py"),
        "import os\nprint(open('x.txt').read() if os.path.exists('x.txt') else 'none')\n",
    )
    .unwrap();
    std::fs::write(
        repo.join("in.jsonl"),
        "{\"name\":\"one\",\"args\":[]}\n{\"name\":\"two\",\"args\":[],\"files\":{\"x.txt\":\"from two\"}}\n",
    )
    .unwrap();
    let run = [python.to_string(), "cli.py".to_string()];
    record(&repo, &run, false);
    let dir = app::cases_dir(&repo);
    let two = std::fs::read_to_string(dir.join("two.toml")).unwrap();
    assert!(two.contains("\"two.in\""), "{two}");
    // Written by hand: files of its own, reached through `two.in` (on Unix, `two.in/..` only
    // resolves while `two.in` exists).
    std::fs::create_dir_all(dir.join("shared")).unwrap();
    std::fs::write(dir.join("shared").join("x.txt"), "from shared").unwrap();
    std::fs::write(
        dir.join("mine.toml"),
        two.replace("\"two.in\"", "\"two.in/../shared\""),
    )
    .unwrap();
    std::fs::write(dir.join("mine.stdout"), "from shared\n").unwrap();
    std::fs::copy(dir.join("two.stderr"), dir.join("mine.stderr")).unwrap();

    std::fs::write(repo.join("in.jsonl"), "{\"name\":\"one\",\"args\":[]}\n").unwrap();
    let second = record(&repo, &run, false);
    let kept = dir.join("two.in").is_dir();
    let again = replay(&repo);
    std::fs::remove_dir_all(&repo).unwrap();

    assert_eq!(second.stale, ["two"], "{second:?}");
    assert!(kept, "two.in was removed, which mine's base goes through");
    assert_eq!((again.passed, again.total), (2, 2), "{:?}", again.failures);
}
