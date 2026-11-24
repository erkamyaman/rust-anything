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
    app::record(&RecordOptions {
        repo,
        run,
        inputs: Some(&repo.join("in.jsonl")),
        normalize: &[],
        name: None,
        traps: false,
        timeout_secs: None,
        http,
        server_args: &[],
        fresh: &[],
        pace_ms: None,
        port: None,
        float_tolerance: None,
        float_files: &[],
        effects: None,
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
