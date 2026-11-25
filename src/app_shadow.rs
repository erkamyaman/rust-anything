//! `app shadow`: a proxy in front of a server and a build. Every request is answered by the
//! original; the build gets the same request afterwards and its answer is compared. The build's
//! outbound HTTP and mail go to local sinks and its files to a fresh copy, so no side effect
//! happens twice. Mismatches are logged as shapes, never values.

use std::collections::BTreeSet;
use std::io::Write;
use std::net::{SocketAddr, TcpListener, ToSocketAddrs};
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result, bail, ensure};
use serde::Serialize;

use crate::app::{self, Config, Failure, Normalizers, Problem};
use crate::effects::EffectsConfig;
use crate::http::{self, Launch, Req, Resp, Server};
use crate::mismatch::{At, Group, text_at};
use crate::walk::Canonical;

pub const SHADOW_DIR: &str = "shadow";
pub const MISMATCHES_FILE: &str = "mismatches.jsonl";
pub const INPUTS_FILE: &str = "inputs.jsonl";

/// What a running shadow reports as it goes.
pub enum Event<'a> {
    /// The proxy accepts requests at this address.
    Listening(SocketAddr),
    /// The build answered a request differently.
    Mismatch(&'a Failure),
}

pub struct ShadowOptions<'a> {
    pub repo: &'a Path,
    /// Command line of the build to compare.
    pub build: &'a str,
    /// The port the proxy listens on; a free one when `None`.
    pub listen: Option<u16>,
    /// `HOST:PORT` of an original that is already running; the saved command starts one when
    /// `None`.
    pub upstream: Option<&'a str>,
    /// Stop after this many requests; run until stopped when `None`.
    pub requests: Option<usize>,
    pub timeout_secs: Option<u64>,
    /// Append each mismatching request to `shadow/inputs.jsonl`.
    pub keep: bool,
    pub progress: Option<&'a dyn Fn(Event)>,
}

#[derive(Debug, Serialize)]
pub struct ShadowOutcome {
    pub requests: usize,
    pub matched: usize,
    /// One per mismatching request, as shapes: the path is hashed and no value is kept.
    pub mismatches: Vec<Failure>,
    /// The distinct differences among the mismatches, most requests first.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub groups: Vec<Group>,
    pub log: PathBuf,
    /// The kept requests (`--keep`), in `app record --inputs` format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inputs: Option<PathBuf>,
}

/// FNV-1a, 64 bits, as hex.
fn hash(text: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

/// The effects the build runs with: the saved ones, with both sinks on and no database
/// commands, which would reach the original's database.
fn sinks_for(saved: Option<&EffectsConfig>) -> EffectsConfig {
    let mut config = saved.cloned().unwrap_or_default();
    if !config.smtp {
        config.smtp_parts = true;
    }
    config.http = true;
    config.smtp = true;
    config.db_reset = None;
    config.db_dump = None;
    config
}

/// A problem as its shape: where the answers part, with no value kept.
fn shape(field: impl Into<String>, mut at: At) -> Problem {
    at.context = None;
    let mut problem = Problem::new(field, String::new()).at(at);
    problem.detail = problem
        .at
        .as_ref()
        .map(|at| format!("{}\n", at.shape()))
        .unwrap_or_default();
    problem
}

/// What differs between the original's answer and the build's, as shapes: status codes, header
/// names, where the body first differs and its size.
fn compare(
    norm: &Normalizers,
    noisy: &BTreeSet<String>,
    original: &Resp,
    build: &Resp,
) -> Vec<Problem> {
    let mut problems = Vec::new();
    if original.status != build.status {
        problems.push(shape(
            "status",
            At::new(&format!("{}→{}", original.status, build.status)),
        ));
    }
    for (k, expected) in &original.headers {
        if http::is_skipped_header(k) {
            continue;
        }
        let at = match build.headers.get(k) {
            None => At::new("missing key"),
            Some(_) if noisy.contains(k) => continue,
            Some(actual) => {
                let (e, a) = (norm.apply(expected), norm.apply(actual));
                if app::line_matches(&e, &a) {
                    continue;
                }
                text_at("text", e.split("[..]").next().unwrap_or_default(), &a)
            }
        };
        problems.push(shape(format!("header {k}"), at));
    }
    if app::compare(norm, &original.body, &build.body, false, false).is_some() {
        let at = app::locate(norm, &original.body, &build.body, false, false);
        problems.push(shape("body", at));
    }
    problems
}

fn append(path: &Path, line: &str) -> Result<()> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .with_context(|| format!("opening {}", path.display()))?;
    writeln!(file, "{line}").with_context(|| format!("writing {}", path.display()))
}

fn upstream_addr(text: &str) -> Result<SocketAddr> {
    text.to_socket_addrs()
        .with_context(|| format!("--upstream `{text}`: give HOST:PORT"))?
        .next()
        .with_context(|| format!("--upstream `{text}`: no address"))
}

pub fn shadow(opts: &ShadowOptions) -> Result<ShadowOutcome> {
    app::check_timeout(opts.timeout_secs)?;
    ensure!(opts.requests != Some(0), "--requests must be at least 1");
    let repo = opts
        .repo
        .canonical()
        .with_context(|| format!("no folder {}", opts.repo.display()))?;
    let dir = app::cases_dir(&repo);
    let Some(config) = Config::load(&repo)? else {
        bail!(
            "no {} in {}: record the server first (`rust-anything app record --http`)",
            app::CONFIG_FILE,
            dir.display()
        );
    };
    let Some(http_config) = config.http.clone() else {
        bail!(
            "the recording in {} is a CLI's: `app shadow` covers servers only",
            dir.display()
        );
    };
    let command = app::shell_split(opts.build)?;
    ensure!(!command.is_empty(), "the build command is empty");
    let norm = Normalizers::new(&config)?;
    let timeout_secs = opts.timeout_secs.unwrap_or(config.timeout_secs);
    ensure!(
        timeout_secs >= 1,
        "--timeout must be at least 1 second (saved timeout_secs in {} is 0)",
        app::CONFIG_FILE
    );
    let timeout = Duration::from_secs(timeout_secs);
    let noisy = http::noisy_headers(&repo);
    let ready = http::ready_timeout(&http_config);

    let listener = TcpListener::bind(("127.0.0.1", opts.listen.unwrap_or(0)))
        .with_context(|| format!("listening on port {}", opts.listen.unwrap_or(0)))?;
    let original = match opts.upstream {
        Some(text) => (None, upstream_addr(text)?),
        None => {
            let launch = Launch {
                args: &http_config.args,
                files: None,
                pace: None,
                effects: None,
                ready,
            };
            let server = match http_config.port {
                Some(port) => Server::start(&config.run, &launch, &repo, port),
                None => http::start_ephemeral(&config.run, &launch, &repo, http::free_port),
            }
            .context("starting the original (--upstream uses one that is already running)")?;
            let addr = server.addr;
            (Some(server), addr)
        }
    };
    let (_original_server, upstream) = original;
    let scratch = app::temp_dir("shadow")?;
    let files = http::snapshot_to(&repo, &http_config.fresh, &scratch.join("files"))?;
    let sinks = sinks_for(config.effects.as_ref());
    let launch = Launch {
        args: &http_config.args,
        files: files.as_deref(),
        pace: None,
        effects: Some(&sinks),
        ready,
    };
    let build = http::start_ephemeral(&command, &launch, &repo, http::free_port)
        .context("starting the build")?;

    let shadow_dir = dir.join(SHADOW_DIR);
    std::fs::create_dir_all(&shadow_dir)
        .with_context(|| format!("creating {}", shadow_dir.display()))?;
    let mut outcome = ShadowOutcome {
        requests: 0,
        matched: 0,
        mismatches: Vec::new(),
        groups: Vec::new(),
        log: shadow_dir.join(MISMATCHES_FILE),
        inputs: opts.keep.then(|| shadow_dir.join(INPUTS_FILE)),
    };
    if let Some(progress) = opts.progress {
        progress(Event::Listening(listener.local_addr()?));
    }
    for client in listener.incoming() {
        let Ok(mut client) = client else {
            continue;
        };
        client.set_read_timeout(Some(timeout))?;
        client.set_write_timeout(Some(timeout))?;
        let Ok(req) = http::read_request(&mut client) else {
            let _ = client.write_all(
                b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
            continue;
        };
        outcome.requests += 1;
        match http::exchange(upstream, &req, timeout) {
            Err(_) => {
                let _ = client.write_all(
                    b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
                );
            }
            Ok((answer, raw)) => {
                let _ = client.write_all(&raw);
                drop(client);
                let problems = match http::exchange(build.addr, &req, timeout) {
                    Ok((built, _)) => compare(&norm, &noisy, &answer, &built),
                    Err(_) => vec![shape("response", At::new("no response"))],
                };
                build.drain_effects(&repo);
                if problems.is_empty() {
                    outcome.matched += 1;
                } else {
                    let failure = mismatch(&req, problems);
                    record(&outcome, &failure, &req)?;
                    if let Some(progress) = opts.progress {
                        progress(Event::Mismatch(&failure));
                    }
                    outcome.mismatches.push(failure);
                }
            }
        }
        if opts.requests.is_some_and(|n| outcome.requests >= n) {
            break;
        }
    }
    drop(build);
    let _ = std::fs::remove_dir_all(&scratch);
    outcome.groups = crate::mismatch::replay_groups(&outcome.mismatches);
    Ok(outcome)
}

fn mismatch(req: &Req, problems: Vec<Problem>) -> Failure {
    let path = hash(&req.path);
    Failure {
        case: format!("{} {path}", req.method),
        args: vec![req.method.clone(), path],
        problems,
    }
}

/// Appends the mismatch's shape to the log and, with `--keep`, the request to the inputs.
fn record(outcome: &ShadowOutcome, failure: &Failure, req: &Req) -> Result<()> {
    append(&outcome.log, &serde_json::to_string(failure)?)?;
    if let Some(inputs) = &outcome.inputs {
        let mut kept = req.clone();
        kept.headers.retain(|k, _| !k.eq_ignore_ascii_case("host"));
        append(inputs, &serde_json::to_string(&kept)?)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn config(normalize: &[&str]) -> Config {
        serde_json::from_value(serde_json::json!({
            "schema": "rust-anything/app-cases@1",
            "name": "server",
            "run": ["node", "server.js"],
            "normalize": normalize,
        }))
        .unwrap()
    }

    fn resp(status: u16, headers: &[(&str, &str)], body: &str) -> Resp {
        Resp {
            status,
            headers: headers
                .iter()
                .map(|(k, v)| (k.to_string(), v.to_string()))
                .collect::<BTreeMap<_, _>>(),
            body: body.as_bytes().to_vec(),
        }
    }

    fn fields(problems: &[Problem]) -> Vec<&str> {
        problems.iter().map(|p| p.field.as_str()).collect()
    }

    #[test]
    fn answers_compare_by_status_headers_and_body_as_shapes() {
        let norm = Normalizers::new(&config(&["timestamps"])).unwrap();
        let none = BTreeSet::new();
        let a = resp(
            200,
            &[
                ("content-type", "text/plain"),
                ("date", "x"),
                ("x-at", "2024-01-02T03:04:05Z"),
            ],
            "at 2024-01-02T03:04:05Z: secret\n",
        );
        let same = resp(
            200,
            &[
                ("content-type", "text/plain"),
                ("x-at", "2025-06-07T08:09:10Z"),
            ],
            "at 2025-06-07T08:09:10Z: secret\n",
        );
        assert!(compare(&norm, &none, &a, &same).is_empty());
        let other = resp(500, &[("x-at", "2025-06-07T08:09:10Z")], "at 2025: oops\n");
        let problems = compare(&norm, &none, &a, &other);
        assert_eq!(fields(&problems), ["status", "header content-type", "body"]);
        let text = format!("{problems:?}");
        assert!(!text.contains("secret") && !text.contains("oops"), "{text}");
        assert_eq!(problems[0].detail, "status (200→500)\n");
        assert_eq!(problems[1].detail, "header content-type (missing key)\n");
        assert_eq!(
            problems[2].detail,
            "body (text), line 1, column 4, char 3, length 16 expected, 14 got\n"
        );
    }

    #[test]
    fn a_header_the_original_varied_on_is_checked_for_presence_only() {
        let norm = Normalizers::new(&config(&[])).unwrap();
        let noisy: BTreeSet<String> = ["set-cookie".to_string()].into();
        let a = resp(200, &[("set-cookie", "id=1")], "");
        assert!(compare(&norm, &noisy, &a, &resp(200, &[("set-cookie", "id=2")], "")).is_empty());
        assert_eq!(
            fields(&compare(&norm, &noisy, &a, &resp(200, &[], ""))),
            ["header set-cookie"]
        );
    }

    #[test]
    fn the_build_gets_both_sinks_and_no_database_commands() {
        let saved = EffectsConfig::from_args(
            &["smtp-raw".into()],
            &["HOOK=http-url".into()],
            Some("reset"),
            Some("dump"),
        )
        .unwrap()
        .unwrap();
        let sinks = sinks_for(Some(&saved));
        assert!(sinks.http && sinks.smtp && !sinks.smtp_parts);
        assert_eq!(sinks.env["HOOK"], "http-url");
        assert!(sinks.db_reset.is_none() && sinks.db_dump.is_none());
        let fresh = sinks_for(None);
        assert!(fresh.http && fresh.smtp && fresh.smtp_parts);
    }

    #[test]
    fn a_mismatch_names_the_request_by_method_and_path_hash() {
        let req = Req {
            method: "GET".to_string(),
            path: "/users/alice".to_string(),
            headers: BTreeMap::new(),
            body: None,
            body_base64: None,
            body_file: None,
        };
        let failure = mismatch(&req, Vec::new());
        assert_eq!(failure.case, format!("GET {}", hash("/users/alice")));
        assert_eq!(failure.case.len(), "GET ".len() + 16);
        assert!(!format!("{failure:?}").contains("alice"));
        assert_ne!(hash("/users/alice"), hash("/users/bob"));
    }
}
