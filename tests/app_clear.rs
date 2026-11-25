//! `app record --no-http`, `--no-args` and `--no-fresh`: settings an earlier recording saved can
//! be removed, so an app recorded as a server can be recorded as a CLI again.

mod common;

use std::path::{Path, PathBuf};

use rust_anything::app::{self, Clear, Config, RecordOptions, ReplayOptions};

const SERVER: &str = include_str!("fixtures/http/python/server.py");
const SERVER_DOCS: &str = include_str!("fixtures/http/python/README.md");
const CLI: &str = "import sys\nprint('hello', *sys.argv[1:])\n";
const CLI_INPUTS: &str = "{\"name\": \"greet\", \"args\": [\"world\"]}\n";

fn repo(name: &str) -> PathBuf {
    let repo = std::env::temp_dir().join(format!(
        "rust-anything-app-clear-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&repo);
    std::fs::create_dir_all(repo.join("data")).unwrap();
    std::fs::write(repo.join("server.py"), SERVER).unwrap();
    std::fs::write(repo.join("README.md"), SERVER_DOCS).unwrap();
    std::fs::write(repo.join("cli.py"), CLI).unwrap();
    std::fs::write(repo.join("inputs.jsonl"), CLI_INPUTS).unwrap();
    std::fs::write(repo.join("data/db.json"), "{}\n").unwrap();
    repo
}

struct Record<'a> {
    run: &'a [String],
    http: bool,
    server_args: &'a [String],
    fresh: &'a [String],
    clear: Clear,
}

fn record(repo: &Path, r: Record) -> anyhow::Result<app::RecordOutcome> {
    let inputs = repo.join("inputs.jsonl");
    // Requests are read from the README's curl lines; the inputs are the CLI's cases.
    let saved_cli = Config::load(repo).unwrap().is_none_or(|c| c.http.is_none());
    let cli = !r.http && r.server_args.is_empty() && (r.clear.http || saved_cli);
    app::record(&RecordOptions {
        repo,
        run: r.run,
        inputs: cli.then_some(inputs.as_path()),
        normalize: &[],
        name: None,
        traps: false,
        timeout_secs: None,
        http: r.http,
        port: None,
        server_args: r.server_args,
        fresh: r.fresh,
        pace_ms: None,
        float_tolerance: None,
        float_files: &[],
        effects: None,
        clear: r.clear,
    })
}

fn config(repo: &Path) -> Config {
    Config::load(repo).unwrap().unwrap()
}

fn strings(words: &[&str]) -> Vec<String> {
    words.iter().map(|w| w.to_string()).collect()
}

#[test]
fn an_app_recorded_as_a_server_can_be_recorded_as_a_cli_again() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("server-to-cli");
    let server = strings(&[python, "server.py"]);
    let cli = strings(&[python, "cli.py"]);
    let fresh = strings(&["data/db.json"]);
    record(
        &repo,
        Record {
            run: &server,
            http: true,
            server_args: &[],
            fresh: &fresh,
            clear: Clear::default(),
        },
    )
    .unwrap();
    assert!(config(&repo).http.is_some());

    // A server that breaks under its own saved command gets no advice to become a CLI.
    std::fs::write(repo.join("server.py"), "import sys\nsys.exit(3)\n").unwrap();
    let err = record(
        &repo,
        Record {
            run: &[],
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear::default(),
        },
    )
    .unwrap_err();
    let message = format!("{err:#}");
    assert!(message.contains("before listening"), "{message}");
    assert!(!message.contains("--no-http"), "{message}");

    // Without --no-http the saved server setting wins, and the error says how to leave it.
    let err = record(
        &repo,
        Record {
            run: &cli,
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear::default(),
        },
    )
    .unwrap_err();
    let message = format!("{err:#}");
    assert!(message.contains("before listening"), "{message}");
    assert!(message.contains("--no-http"), "{message}");

    let outcome = record(
        &repo,
        Record {
            run: &cli,
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear {
                http: true,
                ..Clear::default()
            },
        },
    )
    .unwrap();
    assert_eq!(outcome.cases, 1);
    let saved = config(&repo);
    assert!(saved.http.is_none());
    assert_eq!(saved.run, cli);
    assert!(saved.fresh.is_empty(), "{:?}", saved.fresh);
    let replayed = app::replay(&ReplayOptions {
        repo: &repo,
        build: None,
        timeout_secs: None,
        port: None,
        case: None,
    })
    .unwrap();
    assert_eq!(
        (replayed.passed, replayed.total),
        (1, 1),
        "{:?}",
        replayed.failures
    );
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn saved_args_and_fresh_lists_can_be_removed() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("lists");
    let server = strings(&[python, "server.py"]);
    let cli = strings(&[python, "cli.py"]);
    let fresh = strings(&["data/db.json"]);
    record(
        &repo,
        Record {
            run: &server,
            http: true,
            server_args: &strings(&["extra"]),
            fresh: &fresh,
            clear: Clear::default(),
        },
    )
    .unwrap();
    let http = config(&repo).http.unwrap();
    assert_eq!(
        (http.args, http.fresh),
        (strings(&["extra"]), fresh.clone())
    );

    // A record that names neither list keeps both; --no-args and --no-fresh remove them.
    record(
        &repo,
        Record {
            run: &[],
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear {
                server_args: true,
                fresh: true,
                ..Clear::default()
            },
        },
    )
    .unwrap();
    let http = config(&repo).http.expect("still a server");
    assert!(http.args.is_empty(), "{:?}", http.args);
    assert!(http.fresh.is_empty(), "{:?}", http.fresh);

    // A CLI's saved --fresh list goes the same way.
    record(
        &repo,
        Record {
            run: &cli,
            http: false,
            server_args: &[],
            fresh: &fresh,
            clear: Clear {
                http: true,
                ..Clear::default()
            },
        },
    )
    .unwrap();
    assert_eq!(config(&repo).fresh, fresh);
    record(
        &repo,
        Record {
            run: &[],
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear::default(),
        },
    )
    .unwrap();
    assert_eq!(config(&repo).fresh, fresh, "kept when not named");
    record(
        &repo,
        Record {
            run: &[],
            http: false,
            server_args: &[],
            fresh: &[],
            clear: Clear {
                fresh: true,
                ..Clear::default()
            },
        },
    )
    .unwrap();
    assert!(config(&repo).fresh.is_empty());
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn a_setting_cannot_be_given_and_removed_at_once() {
    let repo = repo("conflicts");
    let run = strings(&["python3", "server.py"]);
    let message = |r: Record| format!("{:#}", record(&repo, r).unwrap_err());
    let clear_http = Clear {
        http: true,
        ..Clear::default()
    };
    let m = message(Record {
        run: &run,
        http: true,
        server_args: &[],
        fresh: &[],
        clear: clear_http,
    });
    assert!(m.contains("--no-http cannot go with"), "{m}");
    let m = message(Record {
        run: &run,
        http: false,
        server_args: &strings(&["x"]),
        fresh: &[],
        clear: clear_http,
    });
    assert!(m.contains("--no-http cannot go with"), "{m}");
    let m = message(Record {
        run: &run,
        http: true,
        server_args: &strings(&["x"]),
        fresh: &[],
        clear: Clear {
            server_args: true,
            ..Clear::default()
        },
    });
    assert!(m.contains("--no-args cannot go with --arg"), "{m}");
    let m = message(Record {
        run: &run,
        http: false,
        server_args: &[],
        fresh: &strings(&["data/db.json"]),
        clear: Clear {
            fresh: true,
            ..Clear::default()
        },
    });
    assert!(m.contains("--no-fresh cannot go with --fresh"), "{m}");
    // Nothing was recorded or saved.
    assert!(Config::load(&repo).unwrap().is_none());
    let _ = std::fs::remove_dir_all(&repo);
}
