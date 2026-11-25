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
    let server_cases = config(&repo).recorded;
    assert!(!server_cases.is_empty());
    assert!(config(&repo).http.is_some());
    // A request written by hand next to the recorded ones.
    let http_dir = app::cases_dir(&repo).join("http");
    std::fs::write(
        http_dir.join("by-hand.toml"),
        "[request]
method = \"GET\"
path = \"/hello\"
",
    )
    .unwrap();

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
    // The server recording's requests are gone, so a later --http cannot replay them; the one
    // written by hand stays.
    for name in &server_cases {
        assert!(!http_dir.join(format!("{name}.toml")).exists(), "{name}");
        assert!(
            outcome.stale.contains(&format!("http/{name}")),
            "{:?}",
            outcome.stale
        );
    }
    assert!(http_dir.join("by-hand.toml").is_file());
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

/// Records `server.py` as a server and returns the names of the requests it wrote.
fn record_server(repo: &Path, python: &str) -> Vec<String> {
    record(
        repo,
        Record {
            run: &strings(&[python, "server.py"]),
            http: true,
            server_args: &[],
            fresh: &[],
            clear: Clear::default(),
        },
    )
    .unwrap();
    let names = config(repo).recorded;
    assert!(!names.is_empty());
    names
}

fn record_cli(repo: &Path, python: &str, clear: Clear) -> app::RecordOutcome {
    record(
        repo,
        Record {
            run: &strings(&[python, "cli.py"]),
            http: false,
            server_args: &[],
            fresh: &[],
            clear,
        },
    )
    .unwrap()
}

/// Keeps a recorded request from being removed while it lives: on Windows its `.toml` is held
/// open without delete sharing, elsewhere its folder is made read-only.
struct Pinned {
    #[cfg(windows)]
    _file: std::fs::File,
    #[cfg(not(windows))]
    dir: PathBuf,
}

impl Pinned {
    fn new(dir: &Path, name: &str) -> Pinned {
        #[cfg(windows)]
        {
            use std::os::windows::fs::OpenOptionsExt;
            const FILE_SHARE_READ: u32 = 1;
            let file = std::fs::OpenOptions::new()
                .read(true)
                .share_mode(FILE_SHARE_READ)
                .open(dir.join(format!("{name}.toml")))
                .unwrap();
            Pinned { _file: file }
        }
        #[cfg(not(windows))]
        {
            use std::os::unix::fs::PermissionsExt;
            let _ = name;
            std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o555)).unwrap();
            Pinned {
                dir: dir.to_path_buf(),
            }
        }
    }
}

#[cfg(not(windows))]
impl Drop for Pinned {
    fn drop(&mut self) {
        use std::os::unix::fs::PermissionsExt;
        let _ = std::fs::set_permissions(&self.dir, std::fs::Permissions::from_mode(0o755));
    }
}

#[test]
fn a_switch_cut_short_is_finished_by_the_next_recording() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("cut-short");
    let server_cases = record_server(&repo, python);
    let http_dir = app::cases_dir(&repo).join("http");
    let no_http = Clear {
        http: true,
        ..Clear::default()
    };

    // The removal of the server's requests fails partway: the CLI config is saved, and still lists
    // the requests left to remove.
    let pinned = Pinned::new(&http_dir, &server_cases[0]);
    let err = record(
        &repo,
        Record {
            run: &strings(&[python, "cli.py"]),
            http: false,
            server_args: &[],
            fresh: &[],
            clear: no_http,
        },
    )
    .unwrap_err();
    drop(pinned);
    let saved = config(&repo);
    assert!(saved.http.is_none(), "{err:#}");
    assert!(
        saved
            .recorded
            .contains(&format!("http/{}", server_cases[0])),
        "{:?}",
        saved.recorded
    );

    // The next recording, without --no-http, removes the rest.
    let outcome = record_cli(&repo, python, Clear::default());
    for name in &server_cases {
        assert!(!http_dir.join(format!("{name}.toml")).exists(), "{name}");
    }
    assert!(
        outcome.stale.contains(&format!("http/{}", server_cases[0])),
        "{:?}",
        outcome.stale
    );
    assert_eq!(config(&repo).recorded, outcome.recorded);
    let _ = std::fs::remove_dir_all(&repo);
}

#[test]
fn a_folder_a_cli_case_reads_stays_when_the_server_cases_go() {
    let Some(python) = common::python() else {
        return;
    };
    let repo = repo("kept-base");
    let server_cases = record_server(&repo, python);
    let first = &server_cases[0];
    // A CLI case written by hand that reads a folder named after a recorded request.
    let cases = app::cases_dir(&repo);
    std::fs::create_dir_all(cases.join(format!("http/{first}.in"))).unwrap();
    std::fs::write(cases.join(format!("http/{first}.in/data.txt")), "x\n").unwrap();
    std::fs::write(
        cases.join("by-hand.toml"),
        format!("bin.name = \"cli\"\nargs = []\n\n[fs]\nbase = \"http/{first}.in\"\n"),
    )
    .unwrap();

    record_cli(
        &repo,
        python,
        Clear {
            http: true,
            ..Clear::default()
        },
    );
    assert!(!cases.join(format!("http/{first}.toml")).exists());
    assert!(cases.join(format!("http/{first}.in/data.txt")).is_file());
    let _ = std::fs::remove_dir_all(&repo);
}
