//! HTTP servers as a black box: start the original (or any build) on a port, send the recorded
//! requests in order, and store or compare what it answers.

use std::collections::{BTreeMap, BTreeSet};
use std::io::{Read, Write};
use std::net::{Ipv6Addr, SocketAddr, TcpListener, TcpStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use toml_edit::{DocumentMut, Item, Table, value};

use crate::app::{
    self, Config, Failure, Fallback, Normalizers, Problem, RecordOptions, RecordOutcome,
    ReplayOutcome,
};

pub const HTTP_DIR: &str = "http";
const READY: Duration = Duration::from_secs(60);
const SKIP_HEADERS: [&str; 12] = [
    "date",
    "host",
    "server",
    "connection",
    "keep-alive",
    "transfer-encoding",
    "content-length",
    "x-powered-by",
    "etag",
    "last-modified",
    "age",
    "via",
];
const LOCAL_HOSTS: [&str; 4] = ["localhost", "127.0.0.1", "0.0.0.0", "[::1]"];

/// `http` in `rust/app-cases/rust-anything.json`: the app is a server.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HttpConfig {
    /// A port the original always listens on. When unset, every start gets a free port through
    /// the `PORT` environment variable and `{port}` in the command.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<u16>,
    /// Arguments every start gets after its command: the original's and every build's.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
    /// Repository paths copied (from the snapshot in `http/files/`) into a fresh working
    /// directory before every start, so state the server writes never leaks between runs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fresh: Vec<String>,
    /// Milliseconds to wait after each response before the next request.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pace_ms: Option<u64>,
}

/// The `http` config of a recording: the saved one, changed by what this record was given.
pub(crate) fn merge_config(saved: Option<HttpConfig>, opts: &RecordOptions) -> Option<HttpConfig> {
    let asked =
        opts.http || opts.port.is_some() || !opts.server_args.is_empty() || opts.pace_ms.is_some();
    let mut config = match (saved, asked) {
        (Some(c), _) => c,
        (None, true) => HttpConfig::default(),
        (None, false) => return None,
    };
    if opts.port.is_some() {
        config.port = opts.port;
    }
    if !opts.server_args.is_empty() {
        config.args = opts.server_args.to_vec();
    }
    if !opts.fresh.is_empty() {
        config.fresh = opts.fresh.to_vec();
    }
    if let Some(pace) = opts.pace_ms {
        config.pace_ms = (pace > 0).then_some(pace);
    }
    Some(config)
}

pub(crate) const FILES_DIR: &str = "files";

pub(crate) fn copy_tree(from: &Path, to: &Path) -> Result<()> {
    if from.is_dir() {
        std::fs::create_dir_all(to)?;
        for entry in std::fs::read_dir(from)?.flatten() {
            copy_tree(&entry.path(), &to.join(entry.file_name()))?;
        }
    } else {
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(from, to)
            .with_context(|| format!("copying {} to {}", from.display(), to.display()))?;
    }
    Ok(())
}

/// Copies the `fresh` paths from the repository into `http/files/`, replacing the last snapshot.
fn snapshot(repo: &Path, fresh: &[String]) -> Result<Option<PathBuf>> {
    let dir = app::cases_dir(repo).join(HTTP_DIR).join(FILES_DIR);
    snapshot_to(repo, fresh, &dir)
}

/// Copies the `fresh` paths from the repository into `dir`, replacing the last snapshot.
pub(crate) fn snapshot_to(repo: &Path, fresh: &[String], dir: &Path) -> Result<Option<PathBuf>> {
    if fresh.is_empty() {
        return Ok(None);
    }
    let dir = dir.to_path_buf();
    let _ = std::fs::remove_dir_all(&dir);
    for path in fresh {
        let rel = Path::new(path);
        ensure!(
            rel.is_relative()
                && rel
                    .components()
                    .all(|c| matches!(c, std::path::Component::Normal(_))),
            "--fresh `{path}`: give a path inside the repository, relative to it"
        );
        let from = repo.join(rel);
        ensure!(from.exists(), "--fresh `{path}`: no such file or folder");
        let to = if path.ends_with('/') && from.is_dir() {
            dir.clone()
        } else {
            dir.join(rel)
        };
        copy_tree(&from, &to)?;
    }
    Ok(Some(dir))
}

/// The snapshot replay starts every build from, when the recording has `fresh` paths.
fn saved_snapshot(repo: &Path, config: &Config) -> Result<Option<PathBuf>> {
    if config.http.as_ref().is_none_or(|h| h.fresh.is_empty()) {
        return Ok(None);
    }
    let dir = app::cases_dir(repo).join(HTTP_DIR).join(FILES_DIR);
    ensure!(
        dir.is_dir(),
        "no {}: record again to snapshot the fresh files",
        dir.display()
    );
    Ok(Some(dir))
}

fn get() -> String {
    "GET".to_string()
}

/// One request, as recorded and as written in an `--inputs` line.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Req {
    #[serde(default = "get")]
    pub method: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    /// Base64-encoded body for non-UTF-8 payloads (images, PDFs, protobufs, binary uploads).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_base64: Option<String>,
    /// Path to a file with raw request body bytes (under `http/` in `.toml` cases, or relative to
    /// the inputs file / repo in `--inputs`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_file: Option<String>,
}

impl Req {
    fn from_body_bytes(bytes: Vec<u8>) -> (Option<String>, Option<String>) {
        match String::from_utf8(bytes) {
            Ok(text) => (Some(text), None),
            Err(err) => (None, Some(encode_base64(&err.into_bytes()))),
        }
    }

    fn body_bytes(&self) -> Result<Option<Vec<u8>>> {
        if let Some(b64) = &self.body_base64 {
            Ok(Some(decode_base64_strict(b64)?))
        } else if let Some(text) = &self.body {
            Ok(Some(text.as_bytes().to_vec()))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Deserialize)]
struct HttpInput {
    #[serde(default)]
    name: Option<String>,
    #[serde(flatten)]
    req: Req,
}

#[derive(Debug, Clone)]
struct Resp {
    status: u16,
    headers: BTreeMap<String, String>,
    body: Vec<u8>,
}

pub fn free_port() -> Result<u16> {
    Ok(TcpListener::bind("127.0.0.1:0")?.local_addr()?.port())
}

/// The loopback addresses a server may listen on. A server that binds to `localhost` can get only
/// `::1` (Node 17+ may pick it), so both are tried, IPv4 first.
fn local(port: u16) -> [SocketAddr; 2] {
    [
        SocketAddr::from(([127, 0, 0, 1], port)),
        SocketAddr::from((Ipv6Addr::LOCALHOST, port)),
    ]
}

/// The first loopback address that accepts a connection on `port`.
fn listening(port: u16, wait: Duration) -> Option<SocketAddr> {
    local(port)
        .into_iter()
        .find(|addr| TcpStream::connect_timeout(addr, wait).is_ok())
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn fill(stream: &mut impl Read, buf: &mut Vec<u8>, need: usize) -> Result<()> {
    let mut chunk = [0u8; 8192];
    while buf.len() < need {
        let n = stream.read(&mut chunk)?;
        ensure!(n > 0, "the connection closed in the middle of the response");
        buf.extend_from_slice(&chunk[..n]);
    }
    Ok(())
}

fn chunk_line_end(stream: &mut impl Read, buf: &mut Vec<u8>) -> Result<usize> {
    loop {
        if let Some(end) = find(buf, b"\r\n") {
            return Ok(end);
        }
        let need = buf.len() + 1;
        fill(stream, buf, need)?;
    }
}

fn read_chunked(stream: &mut impl Read, mut buf: Vec<u8>) -> Result<Vec<u8>> {
    let mut body = Vec::new();
    loop {
        let line_end = chunk_line_end(stream, &mut buf)?;
        let size_text = String::from_utf8_lossy(&buf[..line_end]).into_owned();
        let size = usize::from_str_radix(size_text.split(';').next().unwrap_or("").trim(), 16)
            .with_context(|| format!("bad chunk size `{size_text}`"))?;
        buf.drain(..line_end + 2);
        if size == 0 {
            loop {
                let line_end = chunk_line_end(stream, &mut buf)?;
                if line_end == 0 {
                    return Ok(body);
                }
                buf.drain(..line_end + 2);
            }
        }
        let end = size.checked_add(2).context("chunk size is too large")?;
        fill(stream, &mut buf, end)?;
        ensure!(&buf[size..end] == b"\r\n", "bad chunk terminator");
        body.extend_from_slice(&buf[..size]);
        buf.drain(..end);
    }
}

fn read_response(stream: &mut impl Read, head_only: bool) -> Result<Resp> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 8192];
    let (status, head, mut rest) = loop {
        let header_end = loop {
            if let Some(i) = find(&buf, b"\r\n\r\n") {
                break i;
            }
            let n = stream.read(&mut chunk)?;
            ensure!(n > 0, "the connection closed before a full response");
            buf.extend_from_slice(&chunk[..n]);
        };
        let head = String::from_utf8_lossy(&buf[..header_end]).into_owned();
        let status_line = head.split("\r\n").next().unwrap_or_default();
        let status: u16 = status_line
            .split_whitespace()
            .nth(1)
            .and_then(|s| s.parse().ok())
            .with_context(|| format!("not an HTTP response: `{status_line}`"))?;
        let rest = buf.split_off(header_end + 4);
        if status == 101 || !(100..200).contains(&status) {
            break (status, head, rest);
        }
        buf = rest;
    };
    let lines = head.split("\r\n").skip(1);
    let mut headers: BTreeMap<String, String> = BTreeMap::new();
    for line in lines {
        if let Some((k, v)) = line.split_once(':') {
            let v = v.trim().to_string();
            headers
                .entry(k.trim().to_ascii_lowercase())
                .and_modify(|e| {
                    e.push_str(", ");
                    e.push_str(&v);
                })
                .or_insert(v);
        }
    }
    let chunked = headers
        .get("transfer-encoding")
        .is_some_and(|t| t.to_ascii_lowercase().contains("chunked"));
    let body = if head_only || status == 204 || status == 304 || (100..200).contains(&status) {
        Vec::new()
    } else if chunked {
        read_chunked(stream, rest)?
    } else if let Some(lengths) = headers.get("content-length") {
        let parse = |value: &str| -> Result<usize> {
            let value = value.trim();
            ensure!(
                !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()),
                "invalid Content-Length"
            );
            value.parse().context("invalid Content-Length")
        };
        let mut lengths = lengths.split(',');
        let len = parse(lengths.next().unwrap_or_default())?;
        for other in lengths {
            ensure!(parse(other)? == len, "conflicting Content-Length headers");
        }
        fill(stream, &mut rest, len)?;
        rest.truncate(len);
        rest
    } else {
        stream
            .read_to_end(&mut rest)
            .context("reading close-delimited HTTP body")?;
        rest
    };
    Ok(Resp {
        status,
        headers,
        body,
    })
}

fn send(addr: SocketAddr, req: &Req, timeout: Duration) -> Result<Resp> {
    let mut stream = TcpStream::connect_timeout(&addr, timeout)?;
    stream.set_read_timeout(Some(timeout))?;
    stream.set_write_timeout(Some(timeout))?;
    let body_opt = req.body_bytes()?;
    let body = body_opt.as_deref().unwrap_or(&[]);
    let mut head = format!(
        "{} {} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n",
        req.method, req.path
    );
    for (k, v) in &req.headers {
        if !matches!(
            k.to_ascii_lowercase().as_str(),
            "host" | "connection" | "content-length"
        ) {
            head += &format!("{k}: {v}\r\n");
        }
    }
    if body_opt.is_some() || matches!(req.method.as_str(), "POST" | "PUT" | "PATCH") {
        head += &format!("Content-Length: {}\r\n", body.len());
    }
    head += "\r\n";
    stream.write_all(head.as_bytes())?;
    stream.write_all(body)?;
    read_response(&mut stream, req.method == "HEAD")
}

/// A server process; stopped (SIGTERM, then SIGKILL) when dropped.
struct Server {
    child: Child,
    /// The loopback address that answered while waiting for the server; requests go there.
    addr: SocketAddr,
    dir: PathBuf,
    cwd: PathBuf,
    log: PathBuf,
    stopped: bool,
    sinks: Option<crate::effects::Sinks>,
}

impl Server {
    fn start(command: &[String], launch: &Launch, repo: &Path, port: u16) -> Result<Server> {
        if listening(port, Duration::from_millis(100)).is_some() {
            bail!("port {port} is already in use");
        }
        let dir = app::temp_dir("server")?;
        let words: Vec<String> = command
            .iter()
            .map(|w| app::resolve_word(repo, &w.replace("{port}", &port.to_string())))
            .collect();
        let (program, rest) = words.split_first().context("empty command")?;
        let log = dir.join("fallback.log");
        let cwd = match launch.files {
            Some(files) => {
                let cwd = dir.join("cwd");
                copy_tree(files, &cwd)?;
                cwd
            }
            None => repo.to_path_buf(),
        };
        let sinks = launch
            .effects
            .map(|config| -> Result<_> {
                let sinks = crate::effects::Sinks::start(config)?;
                sinks.reset(&cwd, repo)?;
                Ok(sinks)
            })
            .transpose()?;
        let mut cmd = Command::new(program);
        cmd.args(rest)
            .args(
                launch
                    .args
                    .iter()
                    .map(|a| a.replace("{port}", &port.to_string())),
            )
            .current_dir(&cwd)
            .envs(sinks.iter().flat_map(crate::effects::Sinks::env))
            .env("PORT", port.to_string())
            .env(app::FALLBACK_ENV, &log)
            .stdin(Stdio::null())
            .stdout(std::fs::File::create(dir.join("stdout"))?)
            .stderr(std::fs::File::create(dir.join("stderr"))?);
        let mut child = cmd
            .spawn()
            .with_context(|| format!("starting `{}`", command.join(" ")))?;
        let deadline = Instant::now() + READY;
        let addr = loop {
            let failure = if let Some(status) = child.try_wait()? {
                Some(format!("exited ({status}) before listening"))
            } else if let Some(addr) = listening(port, Duration::from_millis(200)) {
                if let Some(status) = child.try_wait()? {
                    Some(format!("exited ({status}) before listening"))
                } else {
                    break addr;
                }
            } else if Instant::now() > deadline {
                Some(format!("did not listen within {}s", READY.as_secs()))
            } else {
                None
            };
            if let Some(failure) = failure {
                let _ = child.kill();
                let _ = child.wait();
                let stderr = std::fs::read_to_string(dir.join("stderr")).unwrap_or_default();
                let tail: Vec<&str> = stderr.lines().rev().take(15).collect();
                let tail: Vec<&str> = tail.into_iter().rev().collect();
                let _ = std::fs::remove_dir_all(&dir);
                bail!(
                    "`{}` {failure} on port {port}{}",
                    command.join(" "),
                    if tail.is_empty() {
                        String::new()
                    } else {
                        format!(":\n{}", tail.join("\n"))
                    }
                );
            }
            std::thread::sleep(Duration::from_millis(50));
        };
        Ok(Server {
            child,
            addr,
            dir,
            cwd,
            log,
            stopped: false,
            sinks,
        })
    }

    fn fallbacks(&self) -> Option<usize> {
        std::fs::read_to_string(&self.log)
            .ok()
            .map(|t| t.lines().count())
    }

    fn stop(&mut self) {
        if self.stopped {
            return;
        }
        self.stopped = true;
        // The whole process tree: the real server can be a grandchild (`npm start`, `sh -c`).
        // The server stays in our process group, so a Ctrl-C in the terminal still reaches it.
        #[cfg(unix)]
        {
            // The one we started is reaped only below, so its pid cannot go to another process
            // while `end_tree` may still signal it.
            end_tree(
                self.child.id(),
                Duration::from_secs(5),
                processes,
                |signal, pids| {
                    let _ = Command::new("kill")
                        .arg(signal)
                        .arg("--")
                        .args(pids.iter().map(u32::to_string))
                        .stderr(Stdio::null())
                        .status();
                },
            );
        }
        #[cfg(windows)]
        {
            let _ = Command::new("taskkill")
                .args(["/T", "/F", "/PID", &self.child.id().to_string()])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_dir_all(&self.dir);
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.stop();
    }
}

/// Every process as `pid ppid stat start` lines, from `ps` (the start time is several words).
#[cfg(unix)]
fn processes() -> Option<String> {
    let out = Command::new("ps")
        .args([
            "-A", "-o", "pid=", "-o", "ppid=", "-o", "stat=", "-o", "lstart=",
        ])
        .stderr(Stdio::null())
        .output()
        .ok()?;
    out.status
        .success()
        .then(|| String::from_utf8_lossy(&out.stdout).into_owned())
}

/// Ends `root` and every process below it: TERM, a wait of up to `grace` until a listing shows
/// none of them running (so nothing still holds the port), then KILL and the same wait. Every
/// listing adds the processes forked since the last one below a process it tracks, and sends them
/// the current signal too. A failed listing proves nothing: the wait goes on and KILL still
/// follows, to `root` alone if the listing before it failed too, and to the rest of the tree
/// once a listing shows them running as they were. A process whose parent exited
/// before any listing saw it is reparented and can no longer be traced to the server. Returns
/// whether a listing showed the whole tree gone.
///
/// A process is tracked with its start time, and is signalled only while the latest listing shows
/// that pid with that start time, so a pid that went to another process after its own exited is
/// left alone (two processes started in the same second on the same pid look alike, and a pid
/// reused between a listing and the signal is not caught). `root` is the caller's child, not yet
/// reaped, so its pid stays its own and it needs no start time.
#[cfg(any(unix, test))]
fn end_tree(
    root: u32,
    grace: Duration,
    mut list: impl FnMut() -> Option<String>,
    mut signal: impl FnMut(&str, &[u32]),
) -> bool {
    let mut tree: Vec<(u32, Option<String>)> = vec![(root, None)];
    for sig in ["-TERM", "-KILL"] {
        // Each stage's targets come from a listing taken now: an older one may name a pid that
        // went to another process since. Without a listing, only `root` is surely ours, and the
        // others get the signal once a listing shows them again.
        let mut sent: Vec<(u32, Option<String>)> = Vec::new();
        let fresh = list();
        if let Some(ps) = &fresh {
            grow(ps, &mut tree);
        }
        signal_due(sig, fresh.as_deref(), &tree, &mut sent, &mut signal);
        let deadline = Instant::now() + grace;
        loop {
            if let Some(ps) = list() {
                grow(&ps, &mut tree);
                signal_due(sig, Some(&ps), &tree, &mut sent, &mut signal);
                if running(Some(&ps), &tree).is_empty() {
                    return true;
                }
            }
            if Instant::now() > deadline {
                break;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    false
}

/// Sends `sig` to the processes in `tree` that `ps` shows running (see `running`) and that have
/// not had it yet, and records them in `sent`.
#[cfg(any(unix, test))]
fn signal_due(
    sig: &str,
    ps: Option<&str>,
    tree: &[(u32, Option<String>)],
    sent: &mut Vec<(u32, Option<String>)>,
    signal: &mut impl FnMut(&str, &[u32]),
) {
    let due: Vec<_> = tree.iter().filter(|t| !sent.contains(t)).cloned().collect();
    let pids = running(ps, &due);
    if !pids.is_empty() {
        signal(sig, &pids);
        sent.extend(due.into_iter().filter(|(pid, _)| pids.contains(pid)));
    }
}

/// A process in a `ps` line of `pid ppid stat start`.
#[cfg(any(unix, test))]
struct Listed<'a> {
    pid: u32,
    ppid: u32,
    stat: &'a str,
    start: String,
}

#[cfg(any(unix, test))]
fn listed(ps: &str) -> Vec<Listed<'_>> {
    ps.lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            Some(Listed {
                pid: fields.next()?.parse().ok()?,
                ppid: fields.next()?.parse().ok()?,
                stat: fields.next()?,
                start: fields.collect::<Vec<_>>().join(" "),
            })
        })
        .collect()
}

/// Whether a listed process is the tracked one: the same pid, and the same start time if one was
/// recorded.
#[cfg(any(unix, test))]
fn same(p: &Listed, pid: u32, start: Option<&str>) -> bool {
    p.pid == pid && start.is_none_or(|s| s == p.start)
}

/// Adds to `tree` every process listed below one in it, with its start time.
#[cfg(any(unix, test))]
fn grow(ps: &str, tree: &mut Vec<(u32, Option<String>)>) {
    let procs = listed(ps);
    let mut i = 0;
    while let Some((pid, start)) = tree.get(i).cloned() {
        // A pid that went to another process has no children of ours.
        if procs.iter().any(|p| same(p, pid, start.as_deref())) {
            for p in procs.iter().filter(|p| p.ppid == pid) {
                if !tree.iter().any(|(t, s)| same(p, *t, s.as_deref())) {
                    tree.push((p.pid, Some(p.start.clone())));
                }
            }
        }
        i += 1;
    }
}

/// The tracked processes that still run as the listing shows them: the same process (see `same`)
/// and not a zombie, which holds no port. With no listing, only `root` (no start time) is known.
#[cfg(any(unix, test))]
fn running(ps: Option<&str>, tree: &[(u32, Option<String>)]) -> Vec<u32> {
    let Some(ps) = ps else {
        return tree
            .iter()
            .filter(|(_, start)| start.is_none())
            .map(|(pid, _)| *pid)
            .collect();
    };
    let procs = listed(ps);
    tree.iter()
        .filter(|(pid, start)| {
            procs
                .iter()
                .any(|p| same(p, *pid, start.as_deref()) && !p.stat.starts_with('Z'))
        })
        .map(|(pid, _)| *pid)
        .collect()
}

type Answer = (
    std::result::Result<Resp, String>,
    Option<bool>,
    Option<Vec<u8>>,
);

/// What every start gets besides its command: the server's arguments and its fresh files.
struct Launch<'a> {
    args: &'a [String],
    files: Option<&'a Path>,
    pace: Option<Duration>,
    effects: Option<&'a crate::effects::EffectsConfig>,
}

/// Starts the server once and sends every request in order.
fn run_all(
    command: &[String],
    launch: &Launch,
    repo: &Path,
    port: u16,
    reqs: &[&Req],
    timeout: Duration,
) -> Result<Vec<Answer>> {
    let mut server = Server::start(command, launch, repo, port)?;
    let mut out = Vec::new();
    for req in reqs {
        let before = server.fallbacks().unwrap_or(0);
        let resp = send(server.addr, req, timeout).map_err(|e| format!("{e:#}"));
        if let Some(pace) = launch.pace {
            std::thread::sleep(pace);
        }
        let fell_back = server.fallbacks().map(|after| after > before);
        let effects = server.sinks.as_ref().map(|s| s.take(&server.cwd, repo));
        out.push((resp, fell_back, effects));
    }
    server.stop();
    Ok(out)
}

fn percent_decode(text: &str) -> String {
    let bytes = text.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%'
            && let Some(hex) = text.get(i + 1..i + 3)
            // Two hex digits only: `from_str_radix` also takes a leading `+`, which would turn
            // `%+1` into an escape for byte 1.
            && hex.len() == 2
            && hex.bytes().all(|b| b.is_ascii_hexdigit())
        {
            out.push(u8::from_str_radix(hex, 16).unwrap_or(0));
            i += 3;
            continue;
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn percent_encode(text: &str) -> String {
    let mut out = String::new();
    for b in text.bytes() {
        if b.is_ascii_alphanumeric() || b"-._~".contains(&b) {
            out.push(char::from(b));
        } else {
            out += &format!("%{b:02X}");
        }
    }
    out
}

struct FormPart {
    name: String,
    filename: Option<String>,
    content_type: Option<String>,
    data: Vec<u8>,
}

fn infer_mime(path: &str) -> &'static str {
    match Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("gif") => "image/gif",
        Some("webp") => "image/webp",
        Some("pdf") => "application/pdf",
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("html" | "htm") => "text/html",
        Some("txt" | "csv") => "text/plain",
        _ => "application/octet-stream",
    }
}

fn escape_param(s: &str) -> String {
    s.replace(['\r', '\n'], "")
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
}

fn split_form_segments(s: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let bytes = s.as_bytes();
    let mut start = 0usize;
    let mut in_quotes = false;
    let mut i = 0usize;
    while i < bytes.len() {
        match bytes[i] {
            b'\\' if in_quotes && i + 1 < bytes.len() => i += 2,
            b'"' => {
                in_quotes = !in_quotes;
                i += 1;
            }
            b';' if !in_quotes => {
                out.push(&s[start..i]);
                i += 1;
                start = i;
            }
            _ => i += 1,
        }
    }
    out.push(&s[start..]);
    out
}

fn unquote_form_value(s: &str) -> String {
    let Some(rest) = s.strip_prefix('"') else {
        return s.to_string();
    };
    let mut out = String::with_capacity(rest.len());
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '\\' => match chars.next() {
                Some(next @ ('"' | '\\')) => out.push(next),
                Some(other) => {
                    out.push('\\');
                    out.push(other);
                }
                None => return s.to_string(),
            },
            '"' => {
                return if chars.next().is_none() {
                    out
                } else {
                    s.to_string()
                };
            }
            _ => out.push(c),
        }
    }
    s.to_string()
}

fn with_multipart_boundary(existing: &str, boundary: &str) -> String {
    let mut parts = split_form_segments(existing).into_iter();
    let media = parts
        .next()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("multipart/form-data");
    let mut out = String::from(media);
    for param in parts {
        let trimmed = param.trim();
        if trimmed.is_empty() {
            continue;
        }
        let is_boundary = trimmed
            .split_once('=')
            .is_some_and(|(k, _)| k.trim().eq_ignore_ascii_case("boundary"));
        if !is_boundary {
            out.push_str("; ");
            out.push_str(trimmed);
        }
    }
    out.push_str("; boundary=");
    out.push_str(boundary);
    out
}

fn parse_form_part(
    spec: &str,
    literal: bool,
    read_bytes: &impl Fn(&str) -> Option<Vec<u8>>,
) -> Option<FormPart> {
    let (name, rest) = spec.split_once('=')?;
    if name.is_empty() {
        return None;
    }
    if literal {
        return Some(FormPart {
            name: name.to_string(),
            filename: None,
            content_type: None,
            data: rest.as_bytes().to_vec(),
        });
    }
    // Split `;type=...` and `;filename=...` parameters outside quoted strings.
    let mut segments = split_form_segments(rest).into_iter();
    let first = segments.next()?;
    let mut filename_override: Option<String> = None;
    let mut type_override: Option<String> = None;
    let mut trailing_literal = String::new();
    for seg in segments {
        let trimmed = seg.trim();
        if let Some(v) = trimmed.strip_prefix("type=") {
            type_override = Some(unquote_form_value(v.trim()));
        } else if let Some(v) = trimmed.strip_prefix("filename=") {
            filename_override = Some(unquote_form_value(v.trim()));
        } else if trimmed.starts_with("headers=") || trimmed.starts_with("encoder=") {
            return None;
        } else if let Some(ct) = type_override.as_mut() {
            // Curl treats parameters after `;type=` (e.g. `;charset=UTF-8`) as part of the media type.
            ct.push(';');
            ct.push_str(trimmed);
        } else {
            trailing_literal.push(';');
            trailing_literal.push_str(seg);
        }
    }
    if let Some(path) = first.strip_prefix('@') {
        let raw_path = if trailing_literal.is_empty() {
            unquote_form_value(path)
        } else {
            format!("{}{trailing_literal}", unquote_form_value(path))
        };
        let data = read_bytes(&raw_path)?;
        let default_name = Path::new(&raw_path)
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| raw_path.clone());
        let ct = type_override.unwrap_or_else(|| infer_mime(&raw_path).to_string());
        Some(FormPart {
            name: name.to_string(),
            filename: Some(filename_override.unwrap_or(default_name)),
            content_type: Some(ct),
            data,
        })
    } else if let Some(path) = first.strip_prefix('<') {
        let raw_path = if trailing_literal.is_empty() {
            unquote_form_value(path)
        } else {
            format!("{}{trailing_literal}", unquote_form_value(path))
        };
        let data = read_bytes(&raw_path)?;
        Some(FormPart {
            name: name.to_string(),
            filename: None,
            content_type: type_override,
            data,
        })
    } else {
        let val = if trailing_literal.is_empty() {
            unquote_form_value(first)
        } else {
            format!("{}{trailing_literal}", unquote_form_value(first))
        };
        Some(FormPart {
            name: name.to_string(),
            filename: filename_override,
            content_type: type_override,
            data: val.into_bytes(),
        })
    }
}

fn build_multipart(parts: &[FormPart]) -> (String, Vec<u8>) {
    let mut boundary = String::from("----RustAnythingFormBoundary7MA4YWxkTrZu0gW");
    let mut suffix = 0u32;
    while parts
        .iter()
        .any(|p| find(&p.data, boundary.as_bytes()).is_some())
    {
        suffix += 1;
        boundary = format!("----RustAnythingFormBoundary{suffix}");
    }
    let mut out = Vec::new();
    for part in parts {
        out.extend_from_slice(b"--");
        out.extend_from_slice(boundary.as_bytes());
        out.extend_from_slice(b"\r\nContent-Disposition: form-data; name=\"");
        out.extend_from_slice(escape_param(&part.name).as_bytes());
        out.push(b'"');
        if let Some(file) = &part.filename {
            out.extend_from_slice(b"; filename=\"");
            out.extend_from_slice(escape_param(file).as_bytes());
            out.push(b'"');
        }
        out.extend_from_slice(b"\r\n");
        if let Some(ct) = &part.content_type {
            out.extend_from_slice(b"Content-Type: ");
            out.extend_from_slice(ct.as_bytes());
            out.extend_from_slice(b"\r\n");
        }
        out.extend_from_slice(b"\r\n");
        out.extend_from_slice(&part.data);
        out.extend_from_slice(b"\r\n");
    }
    out.extend_from_slice(b"--");
    out.extend_from_slice(boundary.as_bytes());
    out.extend_from_slice(b"--\r\n");
    (boundary, out)
}

fn parse_curl(line: &str, repo: &Path, doc_dir: &Path) -> Option<Req> {
    let toks = app::shell_tokens(line)?;
    if toks.iter().any(|t| t.op) {
        return None;
    }
    let words: Vec<String> = toks.into_iter().map(|t| t.text).collect();
    let (first, args) = words.split_first()?;
    if first != "curl" {
        return None;
    }
    let mut method: Option<String> = None;
    let mut headers = BTreeMap::new();
    let mut data: Vec<Vec<u8>> = Vec::new();
    let mut form_parts: Vec<FormPart> = Vec::new();
    let mut url: Option<String> = None;
    let mut i = 0;
    let read_file_bytes = |name: &str| -> Option<Vec<u8>> {
        [repo.join(name), doc_dir.join(name)]
            .iter()
            .find(|p| p.is_file())
            .and_then(|p| std::fs::read(p).ok())
    };
    while i < args.len() {
        let arg = &args[i];
        let mut next = || -> Option<String> {
            i += 1;
            args.get(i).cloned()
        };
        match arg.as_str() {
            "-X" | "--request" => method = Some(next()?),
            "-H" | "--header" => {
                let h = next()?;
                let (k, v) = h.split_once(':')?;
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
            "-d" | "--data" | "--data-raw" | "--data-binary" | "--data-ascii" => {
                let d = next()?;
                let bytes = if let Some(path) = d.strip_prefix('@')
                    && arg != "--data-raw"
                {
                    let mut raw = read_file_bytes(path)?;
                    if arg != "--data-binary" {
                        raw.retain(|b| !matches!(b, b'\r' | b'\n' | b'\0'));
                    }
                    raw
                } else {
                    d.into_bytes()
                };
                data.push(bytes);
            }
            "--json" => {
                let d = next()?;
                let bytes = if let Some(path) = d.strip_prefix('@') {
                    read_file_bytes(path)?
                } else {
                    d.into_bytes()
                };
                data.push(bytes);
                headers
                    .entry("Content-Type".to_string())
                    .or_insert_with(|| "application/json".to_string());
                headers
                    .entry("Accept".to_string())
                    .or_insert_with(|| "application/json".to_string());
            }
            "-F" | "--form" => {
                let spec = next()?;
                form_parts.push(parse_form_part(&spec, false, &read_file_bytes)?);
            }
            "--form-string" => {
                let spec = next()?;
                form_parts.push(parse_form_part(&spec, true, &read_file_bytes)?);
            }
            "-A" | "--user-agent" => {
                headers.insert("User-Agent".to_string(), next()?);
            }
            "-b" | "--cookie" => {
                headers.insert("Cookie".to_string(), next()?);
            }
            "-e" | "--referer" => {
                headers.insert("Referer".to_string(), next()?);
            }
            "-o" | "--output" | "-w" | "--write-out" | "-m" | "--max-time"
            | "--connect-timeout" => {
                next()?;
            }
            "--silent"
            | "--show-error"
            | "--include"
            | "--verbose"
            | "--location"
            | "--fail"
            | "--insecure"
            | "--no-progress-meter" => {}
            other if other.starts_with("-X") && other.len() > 2 => {
                method = Some(other[2..].to_string());
            }
            other if other.starts_with("-H") && other.len() > 2 => {
                let (k, v) = other[2..].split_once(':')?;
                headers.insert(k.trim().to_string(), v.trim().to_string());
            }
            other if other.starts_with("-F") && other.len() > 2 => {
                form_parts.push(parse_form_part(&other[2..], false, &read_file_bytes)?);
            }
            other
                if other.starts_with('-')
                    && other.len() > 1
                    && !other.starts_with("--")
                    && other[1..].chars().all(|c| "sSivLfk".contains(c)) => {}
            other if other.starts_with('-') => return None,
            other => url = Some(other.to_string()),
        }
        i += 1;
    }
    if !data.is_empty() && !form_parts.is_empty() {
        return None;
    }
    let url = url?;
    let (scheme, rest) = url.split_once("://").unwrap_or(("http", &url));
    if !matches!(scheme, "http" | "https") {
        return None;
    }
    let rest = rest.split('#').next()?;
    let (host, target) = rest.split_at(rest.find(['/', '?']).unwrap_or(rest.len()));
    let path = if target.starts_with('/') {
        target.to_string()
    } else {
        format!("/{target}")
    };
    let host = if host.starts_with('[') {
        host.split_inclusive(']').next().unwrap_or(host)
    } else {
        host.split(':').next().unwrap_or(host)
    };
    if !LOCAL_HOSTS.contains(&host) {
        return None;
    }
    let (body, body_base64) = if !form_parts.is_empty() {
        let (boundary, raw) = build_multipart(&form_parts);
        if let Some(existing_key) = headers
            .keys()
            .find(|k| k.eq_ignore_ascii_case("content-type"))
            .cloned()
        {
            let updated = with_multipart_boundary(&headers[&existing_key], &boundary);
            headers.insert(existing_key, updated);
        } else {
            headers.insert(
                "Content-Type".to_string(),
                format!("multipart/form-data; boundary={boundary}"),
            );
        }
        Req::from_body_bytes(raw)
    } else if !data.is_empty() {
        let mut joined = Vec::new();
        for (idx, chunk) in data.into_iter().enumerate() {
            if idx > 0 {
                joined.push(b'&');
            }
            joined.extend_from_slice(&chunk);
        }
        if !headers
            .keys()
            .any(|k| k.eq_ignore_ascii_case("content-type"))
        {
            headers.insert(
                "Content-Type".to_string(),
                "application/x-www-form-urlencoded".to_string(),
            );
        }
        Req::from_body_bytes(joined)
    } else {
        (None, None)
    };
    let has_body = body.is_some() || body_base64.is_some();
    let method = method.unwrap_or_else(|| if has_body { "POST" } else { "GET" }.to_string());
    Some(Req {
        method,
        path,
        headers,
        body,
        body_base64,
        body_file: None,
    })
}

fn read_inputs(path: &Path, repo: &Path) -> Result<Vec<(Option<String>, Req)>> {
    let text =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let inputs_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut out = Vec::new();
    for (n, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let mut input: HttpInput = serde_json::from_str(line).with_context(|| {
            format!(
                "{}:{}: expected {{\"method\", \"path\", \"headers\": {{...}}, \"body\"|\"body_base64\"|\"body_file\"}}",
                path.display(),
                n + 1
            )
        })?;
        ensure!(
            input.req.path.starts_with('/'),
            "{}:{}: `path` must start with /",
            path.display(),
            n + 1
        );
        let body_forms = usize::from(input.req.body.is_some())
            + usize::from(input.req.body_base64.is_some())
            + usize::from(input.req.body_file.is_some());
        ensure!(
            body_forms <= 1,
            "{}:{}: specify at most one of `body`, `body_base64`, or `body_file`",
            path.display(),
            n + 1
        );
        if let Some(b64) = &input.req.body_base64 {
            decode_base64_strict(b64)
                .with_context(|| format!("{}:{}: invalid `body_base64`", path.display(), n + 1))?;
        }
        if let Some(rel) = input.req.body_file.take() {
            let file = if Path::new(&rel).is_absolute() {
                PathBuf::from(&rel)
            } else {
                [inputs_dir.join(&rel), repo.join(&rel)]
                    .into_iter()
                    .find(|p| p.is_file())
                    .unwrap_or_else(|| repo.join(&rel))
            };
            let bytes = std::fs::read(&file).with_context(|| {
                format!(
                    "{}:{}: reading `body_file` `{}`",
                    path.display(),
                    n + 1,
                    file.display()
                )
            })?;
            let (body, body_base64) = Req::from_body_bytes(bytes);
            input.req.body = body;
            input.req.body_base64 = body_base64;
        }
        out.push((input.name, input.req));
    }
    Ok(out)
}

fn trap_values(raw: &str) -> Vec<&'static str> {
    if raw.parse::<i128>().is_ok() {
        app::INT_TRAPS.to_vec()
    } else if raw.parse::<f64>().is_ok() && raw.contains(['.', 'e', 'E']) {
        app::FLOAT_TRAPS.to_vec()
    } else {
        app::STRING_TRAPS.to_vec()
    }
}

fn req_key(req: &Req) -> String {
    serde_json::to_string(req).unwrap_or_default()
}

fn traps(bases: &[Req]) -> (Vec<Req>, usize) {
    let mut per_base = Vec::with_capacity(bases.len());
    for base in bases {
        let mut slots: Vec<Vec<Req>> = Vec::new();
        let (path, query) = match base.path.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (base.path.as_str(), None),
        };
        let segments: Vec<&str> = path.split('/').collect();
        for i in 2..segments.len() {
            let raw = percent_decode(segments[i]);
            let slot: Vec<Req> = trap_values(&raw)
                .into_iter()
                .filter(|v| *v != raw)
                .map(|v| {
                    let mut parts: Vec<String> = segments.iter().map(|s| s.to_string()).collect();
                    parts[i] = percent_encode(v);
                    let mut req = base.clone();
                    req.path =
                        parts.join("/") + &query.map(|q| format!("?{q}")).unwrap_or_default();
                    req
                })
                .collect();
            if !slot.is_empty() {
                slots.push(slot);
            }
        }
        if let Some(query) = query {
            let pairs: Vec<&str> = query.split('&').collect();
            for (i, pair) in pairs.iter().enumerate() {
                let Some((key, raw)) = pair.split_once('=') else {
                    continue;
                };
                let raw = percent_decode(raw);
                let slot: Vec<Req> = trap_values(&raw)
                    .into_iter()
                    .filter(|v| *v != raw)
                    .map(|v| {
                        let mut new: Vec<String> = pairs.iter().map(|p| p.to_string()).collect();
                        new[i] = format!("{key}={}", percent_encode(v));
                        let mut req = base.clone();
                        req.path = format!("{path}?{}", new.join("&"));
                        req
                    })
                    .collect();
                if !slot.is_empty() {
                    slots.push(slot);
                }
            }
        }
        if base.body.is_some() || base.body_base64.is_some() {
            let json = base.body.as_deref().is_some_and(|body| {
                serde_json::from_str::<Value>(body).is_ok_and(|v| v.is_object() || v.is_array())
            });
            let values: &[&str] = if json {
                &app::JSON_TRAPS
            } else {
                &app::STDIN_TRAPS
            };
            slots.push(
                values
                    .iter()
                    .map(|v| {
                        let mut req = base.clone();
                        req.body = Some(v.to_string());
                        req.body_base64 = None;
                        req.body_file = None;
                        req
                    })
                    .collect(),
            );
        }
        per_base.push(app::interleave(slots));
    }
    let unknown_route = Req {
        method: "GET".to_string(),
        path: "/rust-anything-missing".to_string(),
        headers: BTreeMap::new(),
        body: None,
        body_base64: None,
        body_file: None,
    };
    let existing: BTreeSet<_> = bases.iter().map(req_key).collect();
    app::spread_traps(
        per_base,
        vec![unknown_route],
        &existing,
        req_key,
        app::MAX_TRAPS,
    )
}

fn case_slug(req: &Req) -> String {
    let first = req
        .path
        .split(['/', '?'])
        .find(|s| !s.is_empty())
        .unwrap_or("root");
    app::slug(&format!("{} {first}", req.method))
}

fn settle_headers(
    norm: &Normalizers,
    a: &BTreeMap<String, String>,
    b: &BTreeMap<String, String>,
) -> (BTreeMap<String, String>, bool) {
    let mut out = BTreeMap::new();
    let mut noisy = false;
    for (k, va) in a {
        if SKIP_HEADERS.contains(&k.as_str()) {
            continue;
        }
        let Some(vb) = b.get(k) else {
            noisy = true;
            continue;
        };
        let (va, vb) = (norm.apply(va), norm.apply(vb));
        if va == vb {
            out.insert(k.clone(), va);
        } else {
            noisy = true;
            out.insert(k.clone(), app::merge_line(&va, &vb));
        }
    }
    noisy |= b
        .keys()
        .any(|k| !a.contains_key(k) && !SKIP_HEADERS.contains(&k.as_str()));
    (out, noisy)
}

fn table(entries: &BTreeMap<String, String>) -> Table {
    let mut t = Table::new();
    for (k, v) in entries {
        t[k.as_str()] = value(v.as_str());
    }
    t
}

fn write_case(
    dir: &Path,
    name: &str,
    req: &Req,
    status: u16,
    headers: &BTreeMap<String, String>,
    body: &[u8],
    binary: bool,
) -> Result<()> {
    let mut doc = DocumentMut::new();
    let mut request = Table::new();
    request["method"] = value(req.method.as_str());
    request["path"] = value(req.path.as_str());
    if let Some(body) = &req.body {
        request["body"] = value(body.as_str());
    } else if let Some(b64) = &req.body_base64 {
        request["body_base64"] = value(b64.as_str());
    }
    if !req.headers.is_empty() {
        request.insert("headers", Item::Table(table(&req.headers)));
    }
    doc.insert("request", Item::Table(request));
    let mut response = Table::new();
    response["status"] = value(i64::from(status));
    if binary {
        response["binary"] = value(true);
    }
    if !headers.is_empty() {
        response.insert("headers", Item::Table(table(headers)));
    }
    doc.insert("response", Item::Table(response));
    std::fs::write(dir.join(format!("{name}.toml")), doc.to_string())?;
    std::fs::write(dir.join(format!("{name}.body")), body)?;
    Ok(())
}

pub(crate) fn record(
    repo: &Path,
    config: &Config,
    norm: &Normalizers,
    opts: &RecordOptions,
) -> Result<RecordOutcome> {
    let examples = app::scan_examples(repo, |line, doc_dir| parse_curl(line, repo, doc_dir));
    let inputs = match opts.inputs {
        Some(path) => read_inputs(path, repo)?,
        None => Vec::new(),
    };
    let mut planned: Vec<(String, Req)> = Vec::new();
    let mut seen = BTreeSet::new();
    let mut taken = BTreeSet::new();
    let mut counts = [0usize; 3];
    let mut bases = Vec::new();
    let mut push = |kind: usize, given: Option<String>, req: Req| -> bool {
        if !seen.insert(req_key(&req)) {
            return false;
        }
        counts[kind] += 1;
        let prefix = ["example", "input", "trap"][kind];
        let name = match given.map(|g| app::given_slug(&g)).filter(|g| !g.is_empty()) {
            Some(g) => g,
            None => format!("{prefix}-{:03}-{}", counts[kind], case_slug(&req)),
        };
        let name = app::reserve_case_name(&mut taken, name, counts[kind]);
        planned.push((name, req));
        true
    };
    for req in examples {
        if push(0, None, req.clone()) {
            bases.push(req);
        }
    }
    for (given, req) in inputs {
        if push(1, given, req.clone()) {
            bases.push(req);
        }
    }
    if bases.is_empty() {
        let root = Req {
            method: "GET".to_string(),
            path: "/".to_string(),
            headers: BTreeMap::new(),
            body: None,
            body_base64: None,
            body_file: None,
        };
        push(0, None, root.clone());
        bases.push(root);
    }
    let mut traps_dropped = 0usize;
    if opts.traps {
        let (generated, dropped) = traps(&bases);
        traps_dropped = dropped;
        for req in generated {
            push(2, None, req);
        }
    }
    planned.sort_by(|a, b| a.0.cmp(&b.0));

    let timeout = Duration::from_secs(config.timeout_secs);
    let fixed = config.http.as_ref().and_then(|h| h.port);
    let reqs: Vec<&Req> = planned.iter().map(|(_, r)| r).collect();
    let http = config.http.clone().unwrap_or_default();
    let files = snapshot(repo, &http.fresh)?;
    let launch = Launch {
        args: &http.args,
        files: files.as_deref(),
        pace: http.pace_ms.map(Duration::from_millis),
        effects: config.effects.as_ref(),
    };
    let one = run_all(
        &config.run,
        &launch,
        repo,
        fixed.map_or_else(free_port, Ok)?,
        &reqs,
        timeout,
    )?;
    let two = run_all(
        &config.run,
        &launch,
        repo,
        fixed.map_or_else(free_port, Ok)?,
        &reqs,
        timeout,
    )?;

    let dir = app::cases_dir(repo).join(HTTP_DIR);
    std::fs::create_dir_all(&dir).with_context(|| format!("creating {}", dir.display()))?;
    let mut outcome = RecordOutcome {
        dir: dir.clone(),
        run: config.run.clone(),
        cases: 0,
        examples: counts[0],
        inputs: counts[1],
        traps: counts[2],
        traps_dropped,
        noisy: Vec::new(),
        noise: Vec::new(),
        unstable: Vec::new(),
        timed_out: Vec::new(),
        recorded: Vec::new(),
        stale: Vec::new(),
        kept: Vec::new(),
    };
    for (((name, req), (a, _, fx_a)), (b, _, fx_b)) in planned.iter().zip(one).zip(two) {
        let (Ok(a), Ok(b)) = (a, b) else {
            outcome.timed_out.push(name.clone());
            continue;
        };
        let (body, body_noise, binary) = app::settle_text(norm, &a.body, &b.body);
        let (headers, header_noisy) = settle_headers(norm, &a.headers, &b.headers);
        write_case(&dir, name, req, a.status, &headers, &body, binary)?;
        let effects_file = dir.join(format!("{name}.{}", crate::effects::EXT));
        let _ = std::fs::remove_file(&effects_file);
        let mut fx_noise = None;
        if let (Some(fa), Some(fb)) = (&fx_a, &fx_b) {
            let (fx, noise, _) = app::settle_text(norm, fa, fb);
            std::fs::write(&effects_file, fx)?;
            fx_noise = noise;
        }
        outcome.cases += 1;
        outcome.recorded.push(name.clone());
        if body_noise.is_some() || header_noisy || fx_noise.is_some() {
            outcome.noisy.push(name.clone());
        }
        for (field, kind) in [
            ("body", body_noise),
            ("headers", header_noisy.then_some(app::NoiseKind::Values)),
            (crate::effects::EXT, fx_noise),
        ] {
            if let Some(kind) = kind {
                outcome.noise.push(app::Noise {
                    case: name.clone(),
                    field: field.to_string(),
                    kind,
                });
            }
        }
        if a.status != b.status || (binary && body_noise.is_some()) {
            outcome.unstable.push(name.clone());
        }
    }
    Ok(outcome)
}

struct Spec {
    name: String,
    req: Req,
    status: u16,
    headers: BTreeMap<String, String>,
    body: Option<Vec<u8>>,
    binary: bool,
    effects: Option<Vec<u8>>,
}

fn strings(item: Option<&Item>) -> BTreeMap<String, String> {
    item.and_then(Item::as_table_like)
        .map(|t| {
            t.iter()
                .map(|(k, v)| (k.to_string(), v.as_str().unwrap_or_default().to_string()))
                .collect()
        })
        .unwrap_or_default()
}

fn load_spec(dir: &Path, path: &Path) -> Result<Spec> {
    let name = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let doc: DocumentMut = std::fs::read_to_string(path)
        .with_context(|| format!("reading {}", path.display()))?
        .parse()
        .with_context(|| format!("parsing {}", path.display()))?;
    let request = doc.get("request").context("no [request]")?;
    let response = doc.get("response").context("no [response]")?;
    let body_file = dir.join(format!("{name}.body"));
    let effects_file = dir.join(format!("{name}.{}", crate::effects::EXT));
    let raw_body = request.get("body");
    let raw_body_base64 = request.get("body_base64");
    let raw_body_file = request.get("body_file");
    let body_forms = usize::from(raw_body.is_some())
        + usize::from(raw_body_base64.is_some())
        + usize::from(raw_body_file.is_some());
    ensure!(
        body_forms <= 1,
        "{}: [request] may specify at most one of body, body_base64, or body_file",
        path.display()
    );
    let req_body = match raw_body {
        Some(item) => Some(
            item.as_str()
                .with_context(|| format!("{}: request.body must be a string", path.display()))?
                .to_string(),
        ),
        None => None,
    };
    let req_body_base64 = match raw_body_base64 {
        Some(item) => Some(
            item.as_str()
                .with_context(|| {
                    format!("{}: request.body_base64 must be a string", path.display())
                })?
                .to_string(),
        ),
        None => None,
    };
    let req_body_file = match raw_body_file {
        Some(item) => Some(
            item.as_str()
                .with_context(|| format!("{}: request.body_file must be a string", path.display()))?
                .to_string(),
        ),
        None => None,
    };
    let (req_body, req_body_base64) = if let Some(b64) = req_body_base64 {
        decode_base64_strict(&b64)
            .with_context(|| format!("{}: invalid request.body_base64", path.display()))?;
        (None, Some(b64))
    } else if let Some(rel) = req_body_file {
        let rel_path = Path::new(&rel);
        ensure!(
            !rel_path.is_absolute()
                && rel_path.components().all(|c| matches!(
                    c,
                    std::path::Component::Normal(_) | std::path::Component::CurDir
                )),
            "{}: request.body_file `{rel}` must be a relative path under http/",
            path.display()
        );
        let root = std::fs::canonicalize(dir)
            .with_context(|| format!("{}: resolving http case directory", path.display()))?;
        let candidate = std::fs::canonicalize(dir.join(rel_path))
            .with_context(|| format!("{}: reading request.body_file `{rel}`", path.display()))?;
        ensure!(
            candidate.starts_with(&root),
            "{}: request.body_file `{rel}` escapes http/",
            path.display()
        );
        let meta = std::fs::metadata(&candidate)
            .with_context(|| format!("{}: reading request.body_file `{rel}`", path.display()))?;
        ensure!(
            meta.is_file(),
            "{}: request.body_file `{rel}` is not a regular file",
            path.display()
        );
        let bytes = std::fs::read(&candidate)
            .with_context(|| format!("{}: reading request.body_file `{rel}`", path.display()))?;
        Req::from_body_bytes(bytes)
    } else {
        (req_body, None)
    };
    Ok(Spec {
        effects: if effects_file.exists() {
            Some(std::fs::read(&effects_file)?)
        } else {
            None
        },
        req: Req {
            method: request
                .get("method")
                .and_then(Item::as_str)
                .unwrap_or("GET")
                .to_string(),
            path: request
                .get("path")
                .and_then(Item::as_str)
                .with_context(|| format!("{}: no request.path", path.display()))?
                .to_string(),
            headers: strings(request.get("headers")),
            body: req_body,
            body_base64: req_body_base64,
            body_file: None,
        },
        status: response
            .get("status")
            .and_then(Item::as_integer)
            .and_then(|s| u16::try_from(s).ok())
            .with_context(|| format!("{}: no response.status", path.display()))?,
        headers: strings(response.get("headers")),
        body: if body_file.exists() {
            Some(std::fs::read(&body_file)?)
        } else {
            None
        },
        binary: response
            .get("binary")
            .and_then(Item::as_bool)
            .unwrap_or(false),
        name,
    })
}

/// The recorded requests, as `METHOD /path` without the query, in case order.
pub(crate) fn recorded_routes(repo: &Path) -> Result<Vec<String>> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for spec in load_specs(&app::cases_dir(repo).join(HTTP_DIR))? {
        let path = spec.req.path.split('?').next().unwrap_or("/").to_string();
        let route = format!("{} {path}", spec.req.method);
        if seen.insert(route.clone()) {
            out.push(route);
        }
    }
    Ok(out)
}

fn load_specs(dir: &Path) -> Result<Vec<Spec>> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .with_context(|| {
            format!(
                "no HTTP cases in {}: run `rust-anything app record` first",
                dir.display()
            )
        })?
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "toml"))
        .collect();
    paths.sort();
    let specs = paths
        .iter()
        .map(|p| load_spec(dir, p))
        .collect::<Result<Vec<_>>>()?;
    ensure!(!specs.is_empty(), "no HTTP cases in {}", dir.display());
    Ok(specs)
}

pub(crate) fn replay(
    repo: &Path,
    config: &Config,
    norm: &Normalizers,
    command: &[String],
    port: Option<u16>,
    case: Option<&str>,
    timeout: Duration,
) -> Result<ReplayOutcome> {
    let dir = app::cases_dir(repo);
    let specs = app::only_case(load_specs(&dir.join(HTTP_DIR))?, case, |s| &s.name)?;
    let port = match port.or(config.http.as_ref().and_then(|h| h.port)) {
        Some(p) => p,
        None => free_port()?,
    };
    let reqs: Vec<&Req> = specs.iter().map(|s| &s.req).collect();
    let files = saved_snapshot(repo, config)?;
    let http = config.http.clone().unwrap_or_default();
    let launch = Launch {
        args: &http.args,
        files: files.as_deref(),
        pace: http.pace_ms.map(Duration::from_millis),
        effects: config.effects.as_ref(),
    };
    let answers = run_all(command, &launch, repo, port, &reqs, timeout)?;
    let mut outcome = ReplayOutcome {
        build: command.to_vec(),
        passed: 0,
        total: 0,
        skipped: 0,
        failures: Vec::new(),
        fallback: None,
        report: dir.join(app::REPORT_FILE),
    };
    let (mut reported, mut fell_back) = (false, Vec::new());
    for (spec, (answer, fallback, effects)) in specs.iter().zip(answers) {
        outcome.total += 1;
        reported |= fallback.is_some();
        if fallback == Some(true) {
            fell_back.push(spec.name.clone());
        }
        let mut problems = Vec::new();
        match answer {
            Err(e) => problems.push(Problem {
                field: "response".to_string(),
                detail: format!("no response: {e}\n"),
            }),
            Ok(resp) => {
                if resp.status != spec.status {
                    problems.push(Problem {
                        field: "status".to_string(),
                        detail: format!("expected {}, got {}\n", spec.status, resp.status),
                    });
                }
                for (k, expected) in &spec.headers {
                    let field = format!("header {k}");
                    let key = k.to_ascii_lowercase();
                    match resp.headers.get(&key) {
                        None => problems.push(Problem {
                            field,
                            detail: format!("- {expected}\n(missing)\n"),
                        }),
                        Some(actual) => {
                            let (e, a) = (norm.apply(expected), norm.apply(actual));
                            if !app::line_matches(&e, &a) {
                                problems.push(Problem {
                                    field,
                                    detail: format!("- {e}\n+ {a}\n"),
                                });
                            }
                        }
                    }
                }
                if let Some(expected) = &spec.body
                    && let Some(detail) = app::compare(
                        norm,
                        expected,
                        &resp.body,
                        spec.binary,
                        norm.unordered(&spec.name, "body"),
                    )
                {
                    problems.push(Problem {
                        field: "body".to_string(),
                        detail,
                    });
                }
            }
        }
        if let Some(expected) = &spec.effects
            && let Some(detail) = app::compare(
                norm,
                expected,
                effects.as_deref().unwrap_or_default(),
                false,
                norm.unordered(&spec.name, crate::effects::EXT),
            )
        {
            problems.push(Problem {
                field: crate::effects::EXT.to_string(),
                detail,
            });
        }
        if problems.is_empty() {
            outcome.passed += 1;
        } else {
            outcome.failures.push(Failure {
                case: spec.name.clone(),
                args: vec![spec.req.method.clone(), spec.req.path.clone()],
                problems,
            });
        }
    }
    if reported {
        outcome.fallback = Some(Fallback {
            cases: fell_back.len(),
            total: outcome.total,
            rate: fell_back.len() as f64 / outcome.total.max(1) as f64,
            names: fell_back,
        });
    }
    std::fs::write(&outcome.report, app::report_markdown(&outcome))
        .with_context(|| format!("writing {}", outcome.report.display()))?;
    Ok(outcome)
}

fn encode_base64(input: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(input.len().div_ceil(3) * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let n = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((n >> 18) & 0x3f) as usize] as char);
        out.push(TABLE[((n >> 12) & 0x3f) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((n >> 6) & 0x3f) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(n & 0x3f) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn decode_base64_strict(input: &str) -> Result<Vec<u8>> {
    let cleaned: Vec<u8> = input.bytes().filter(|b| !b.is_ascii_whitespace()).collect();
    if cleaned.is_empty() {
        return Ok(Vec::new());
    }
    ensure!(
        cleaned.len().is_multiple_of(4),
        "base64 length must be a multiple of 4"
    );
    let mut out = Vec::with_capacity(cleaned.len() / 4 * 3);
    let (chunks, _) = cleaned.as_chunks::<4>();
    for (chunk_idx, chunk) in chunks.iter().enumerate() {
        let is_last = (chunk_idx + 1) * 4 == cleaned.len();
        let decode = |b: u8| -> Option<u8> {
            match b {
                b'A'..=b'Z' => Some(b - b'A'),
                b'a'..=b'z' => Some(b - b'a' + 26),
                b'0'..=b'9' => Some(b - b'0' + 52),
                b'+' => Some(62),
                b'/' => Some(63),
                _ => None,
            }
        };
        let a = decode(chunk[0]).context("invalid base64 character")?;
        let b = decode(chunk[1]).context("invalid base64 character")?;
        match (chunk[2], chunk[3]) {
            (b'=', b'=') => {
                ensure!(is_last, "unexpected base64 padding before end");
                out.push((a << 2) | (b >> 4));
            }
            (c, b'=') => {
                ensure!(is_last, "unexpected base64 padding before end");
                let c = decode(c).context("invalid base64 character")?;
                out.push((a << 2) | (b >> 4));
                out.push((b << 4) | (c >> 2));
            }
            (c, d) => {
                let c = decode(c).context("invalid base64 character")?;
                let d = decode(d).context("invalid base64 character")?;
                out.push((a << 2) | (b >> 4));
                out.push((b << 4) | (c >> 2));
                out.push((c << 6) | d);
            }
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tree_of_a_server_is_found_and_zombies_are_not_alive() {
        // pid ppid stat start, as `ps -A -o pid= -o ppid= -o stat= -o lstart=` prints them.
        let ps = "    1     0 Ss   Tue Nov 24 13:00:00 2026\n  100     1 S    Tue Nov 24 14:00:00 2026\n  200   100 Z    Tue Nov 24 14:00:01 2026\n  300   100 S    Tue Nov 24 14:00:01 2026\n  400   300 R    Tue Nov 24 14:00:02 2026\n  500     1 S    Tue Nov 24 14:00:00 2026\n  600   500 S    Tue Nov 24 14:00:01 2026\n";
        let tree = |root| {
            let mut tree = vec![(root, None)];
            grow(ps, &mut tree);
            tree
        };
        let pids =
            |tree: &[(u32, Option<String>)]| tree.iter().map(|(p, _)| *p).collect::<Vec<_>>();
        assert_eq!(pids(&tree(100)), [100, 200, 300, 400]);
        assert_eq!(tree(100)[2].1.as_deref(), Some("Tue Nov 24 14:00:01 2026"));
        assert_eq!(pids(&tree(400)), [400]);
        assert_eq!(pids(&tree(999)), [999]);
        assert_eq!(
            running(Some(ps), &tree(100)),
            [100, 300, 400],
            "a zombie holds no port"
        );
        assert!(running(Some(ps), &[(999, None)]).is_empty());
        assert_eq!(running(None, &tree(100)), [100]);
    }

    /// Runs `end_tree` on these listings (`None` a failed `ps`, and failing once they run out)
    /// and returns the signals it sent and whether it saw the tree gone.
    fn ending(listings: &[Option<&str>], grace: Duration) -> (Vec<(String, Vec<u32>)>, bool) {
        let mut listings = listings.iter().map(|l| l.map(str::to_string));
        let mut sent = Vec::new();
        let gone = end_tree(
            100,
            grace,
            || listings.next().flatten(),
            |signal, pids| sent.push((signal.to_string(), pids.to_vec())),
        );
        (sent, gone)
    }

    #[test]
    fn a_failed_listing_after_term_still_ends_in_kill() {
        // `ps` works for TERM, then fails: the wait goes on to the deadline, and KILL goes to
        // the started child only, since 300's pid may belong to another process by then.
        let (sent, gone) = ending(
            &[Some("  100 1 S\n  300 100 S\n")],
            Duration::from_millis(60),
        );
        assert!(!gone);
        assert_eq!(
            sent,
            [
                ("-TERM".to_string(), vec![100, 300]),
                ("-KILL".to_string(), vec![100])
            ]
        );
    }

    #[test]
    fn a_process_forked_while_stopping_gets_the_signal_too() {
        let (sent, gone) = ending(
            &[
                Some("  100 1 S\n  300 100 S\n"),
                // 300 forked 700 after the first listing.
                Some("  100 1 S\n  300 100 S\n  700 300 S\n"),
                // 300 is gone and 700 reparented, still running: still waited for.
                Some("  100 1 Z\n  700 1 S\n"),
                Some("  100 1 Z\n"),
            ],
            Duration::from_secs(5),
        );
        assert!(gone);
        assert_eq!(
            sent,
            [
                ("-TERM".to_string(), vec![100, 300]),
                ("-TERM".to_string(), vec![700])
            ]
        );
    }

    #[test]
    fn a_pid_reused_while_stopping_gets_no_kill() {
        // 300 exited and its pid went to another process; 100 ignores TERM. Listed so through
        // the TERM wait (60 ms, about 3 polls) and in the listing taken before KILL.
        const REUSED: &str =
            "  100 1 S Tue Nov 24 14:00:00 2026\n  300 1 S Tue Nov 24 14:00:05 2026\n";
        let (sent, gone) = ending(
            &[
                Some("  100 1 S Tue Nov 24 14:00:00 2026\n  300 100 S Tue Nov 24 14:00:01 2026\n"),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
                Some(REUSED),
            ],
            Duration::from_millis(60),
        );
        assert!(!gone);
        assert_eq!(
            sent,
            [
                ("-TERM".to_string(), vec![100, 300]),
                ("-KILL".to_string(), vec![100])
            ]
        );
    }

    #[test]
    fn a_process_the_listing_before_kill_missed_gets_kill_once_a_listing_shows_it() {
        // 100 and 300 ignore TERM. `ps` fails through the TERM wait and right before KILL, so
        // KILL first goes to 100 alone; the next listing shows 300 still running as it was.
        let tree = "  100 1 S Tue Nov 24 14:00:00 2026\n  300 100 S Tue Nov 24 14:00:01 2026\n";
        let sent = std::cell::RefCell::new(Vec::<(String, Vec<u32>)>::new());
        let calls = std::cell::Cell::new(0);
        let gone = end_tree(
            100,
            Duration::from_millis(60),
            || {
                calls.set(calls.get() + 1);
                let sent = sent.borrow();
                let killed = |pid| sent.iter().any(|(s, p)| s == "-KILL" && p.contains(&pid));
                if calls.get() == 1 {
                    Some(tree.to_string())
                } else if !killed(100) {
                    None
                } else if !killed(300) {
                    Some(tree.to_string())
                } else {
                    Some(String::new())
                }
            },
            |signal, pids| sent.borrow_mut().push((signal.to_string(), pids.to_vec())),
        );
        assert!(gone);
        assert_eq!(
            sent.into_inner(),
            [
                ("-TERM".to_string(), vec![100, 300]),
                ("-KILL".to_string(), vec![100]),
                ("-KILL".to_string(), vec![300])
            ]
        );
    }

    #[test]
    fn stopping_waits_after_kill_until_the_tree_is_gone() {
        let killed = std::cell::Cell::new(false);
        let after_kill = std::cell::Cell::new(0);
        let gone = end_tree(
            100,
            Duration::from_millis(60),
            || {
                let alive = "  100 1 S Tue Nov 24 14:00:00 2026\n".to_string();
                if !killed.get() {
                    return Some(alive);
                }
                after_kill.set(after_kill.get() + 1);
                // Still running once after KILL, then gone.
                Some(if after_kill.get() == 1 {
                    alive
                } else {
                    String::new()
                })
            },
            |signal, _| killed.set(killed.get() || signal == "-KILL"),
        );
        assert!(gone);
        assert_eq!(after_kill.get(), 2, "both listings after KILL were read");
    }

    #[test]
    fn curl_lines_become_requests() {
        let here = Path::new(".");
        let r = parse_curl(
            "curl -sS -X POST -H 'Content-Type: text/plain' -d 'hello world' http://localhost:8080/upper?x=1",
            here,
            here,
        )
        .unwrap();
        assert_eq!(r.method, "POST");
        assert_eq!(r.path, "/upper?x=1");
        assert_eq!(r.body.as_deref(), Some("hello world"));
        assert_eq!(r.headers["Content-Type"], "text/plain");
        let g = parse_curl("curl 127.0.0.1:3000", here, here).unwrap();
        assert_eq!((g.method.as_str(), g.path.as_str()), ("GET", "/"));
        let form = parse_curl("curl -d a=1 http://[::1]:80/f", here, here).unwrap();
        assert_eq!(form.method, "POST");
        assert_eq!(
            form.headers["Content-Type"],
            "application/x-www-form-urlencoded"
        );
        assert!(parse_curl("curl https://example.com/x", here, here).is_none());
        let query = parse_curl("curl 'http://localhost?x=y#section'", here, here).unwrap();
        assert_eq!(query.path, "/?x=y");
        let fragment = parse_curl("curl 'http://[::1]:8080/path#section'", here, here).unwrap();
        assert_eq!(fragment.path, "/path");
        assert!(parse_curl("curl ftp://localhost/file", here, here).is_none());
        assert!(parse_curl("curl --unknown x http://localhost/", here, here).is_none());
        assert!(parse_curl("curl http://localhost/ | jq .", here, here).is_none());
    }

    #[test]
    fn a_curl_example_wrapped_over_lines_is_one_request() {
        let repo = std::env::temp_dir().join(format!("ra-curl-wrapped-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&repo);
        std::fs::create_dir_all(&repo).unwrap();
        std::fs::write(
            repo.join("README.md"),
            "```sh\ncurl -X POST http://localhost:8080/users \\\n  -H 'Content-Type: application/json' \\\n  -d '{\"name\":\"ada\"}'\n```\n",
        )
        .unwrap();
        let reqs = app::scan_examples(&repo, |line, doc_dir| parse_curl(line, &repo, doc_dir));
        std::fs::remove_dir_all(&repo).unwrap();
        assert_eq!(reqs.len(), 1, "{reqs:?}");
        assert_eq!(
            (reqs[0].method.as_str(), reqs[0].path.as_str()),
            ("POST", "/users")
        );
        assert_eq!(reqs[0].body.as_deref(), Some("{\"name\":\"ada\"}"));
        assert_eq!(reqs[0].headers["Content-Type"], "application/json");
    }

    #[test]
    fn curl_data_and_binary_file_inputs_match_curl_file_semantics() {
        let root = app::temp_dir("curl-data").unwrap();
        std::fs::write(root.join("body.txt"), b"one\r\ntwo\0\nthree").unwrap();
        let body = |flag| {
            parse_curl(
                &format!("curl {flag} @body.txt http://localhost/upper"),
                &root,
                &root,
            )
            .unwrap()
            .body
            .unwrap()
        };
        let data = body("--data");
        let binary = body("--data-binary");
        std::fs::remove_dir_all(root).unwrap();
        assert_eq!(data, "onetwothree");
        assert_eq!(binary, "one\r\ntwo\0\nthree");
    }

    #[test]
    fn informational_responses_do_not_replace_the_final_response() {
        let raw = b"HTTP/1.1 100 Continue\r\n\r\nHTTP/1.1 103 Early Hints\r\nLink: </app.js>\r\n\r\nHTTP/1.1 201 Created\r\nContent-Length: 2\r\n\r\nok";
        let response = read_response(&mut &raw[..], false).unwrap();
        assert_eq!(response.status, 201);
        assert_eq!(response.body, b"ok");
        assert!(!response.headers.contains_key("link"));

        let switching = b"HTTP/1.1 101 Switching Protocols\r\n\r\n";
        assert_eq!(
            read_response(&mut &switching[..], false).unwrap().status,
            101
        );
    }

    #[test]
    fn responses_parse_with_length_chunks_or_close() {
        let fixed =
            b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\n\r\nhelloEXTRA";
        let r = read_response(&mut &fixed[..], false).unwrap();
        assert_eq!((r.status, r.body.as_slice()), (200, &b"hello"[..]));
        assert_eq!(r.headers["content-type"], "text/plain");
        let chunked = b"HTTP/1.1 404 Not Found\r\nTransfer-Encoding: chunked\r\n\r\n4\r\nnot \r\n5;x=y\r\nfound\r\n0\r\n\r\n";
        let r = read_response(&mut &chunked[..], false).unwrap();
        assert_eq!((r.status, r.body.as_slice()), (404, &b"not found"[..]));
        let close = b"HTTP/1.0 500 Oops\r\n\r\nbye";
        assert_eq!(read_response(&mut &close[..], false).unwrap().body, b"bye");
    }

    #[test]
    fn malformed_http_chunks_are_errors() {
        let mut bad_ending = &b"1\r\naXX0\r\n\r\n"[..];
        assert!(read_chunked(&mut bad_ending, Vec::new()).is_err());
        let max = format!("{:x}\r\n", usize::MAX);
        assert!(read_chunked(&mut max.as_bytes(), Vec::new()).is_err());
        let mut truncated = &b"1\r\na\r\n0\r\n"[..];
        assert!(read_chunked(&mut truncated, Vec::new()).is_err());
        let mut trailers = &b"1\r\na\r\n0\r\nX-Trailer: present\r\n\r\n"[..];
        assert_eq!(read_chunked(&mut trailers, Vec::new()).unwrap(), b"a");
    }

    #[test]
    fn fixed_body_rejects_malformed_lengths_and_accepts_equal_duplicates() {
        let response = |lengths| {
            let msg = format!("HTTP/1.1 200 OK\r\n{lengths}\r\n\r\nabc trailing");
            read_response(&mut msg.as_bytes(), false)
        };
        assert!(response("Content-Length: invalid").is_err());
        assert!(response("Content-Length: 3\r\nContent-Length: 4").is_err());
        let matching = response("Content-Length: 3\r\nContent-Length: 03").unwrap();
        assert_eq!(matching.body, b"abc");
    }

    #[test]
    fn interrupted_close_delimited_body_is_an_error() {
        struct BrokenBody;

        impl Read for BrokenBody {
            fn read(&mut self, _: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::ErrorKind::TimedOut.into())
            }
        }

        let response = &b"HTTP/1.0 200 OK\r\n\r\npartial"[..];
        assert!(read_response(&mut response.chain(BrokenBody), false).is_err());
    }

    #[test]
    fn traps_change_path_segments_queries_and_bodies() {
        let base = Req {
            method: "POST".into(),
            path: "/div/7/2?name=bob".into(),
            headers: BTreeMap::new(),
            body: Some("{\"a\":1}".into()),
            body_base64: None,
            body_file: None,
        };
        let (t, dropped) = traps(&[base]);
        assert_eq!(dropped, 0);
        assert!(t.iter().any(|r| r.path == "/div/-7/2?name=bob"));
        assert!(t.iter().any(|r| r.path == "/div/7/0?name=bob"));
        assert!(
            t.iter()
                .any(|r| r.path == "/div/7/2?name=%F0%9F%91%8D%F0%9F%8F%BD%20thumbs")
        );
        assert!(
            t.iter()
                .all(|r| r.path.starts_with("/div/") || r.path == "/rust-anything-missing")
        );
        assert!(
            t.iter()
                .any(|r| r.body.as_deref() == Some(app::JSON_TRAPS[0]))
        );
        assert_eq!(percent_decode("a%20b%zz"), "a b%zz");
        // `from_str_radix` accepts a sign, so `%+1` and `%-1` used to decode as an escape.
        assert_eq!(percent_decode("%+1"), "%+1");
        assert_eq!(percent_decode("%-1"), "%-1");
        assert_eq!(percent_decode("%2b"), "+");
        assert_eq!(percent_decode("%2B"), "+");
        assert_eq!(percent_decode("%4"), "%4");
    }

    #[test]
    fn http_traps_spread_across_all_endpoints_keep_unknown_route_and_deduplicate() {
        // 40 endpoints each with a numeric path segment and a query param -> > 300 unique traps.
        let bases: Vec<Req> = (0..40)
            .map(|i| Req {
                method: "GET".into(),
                path: format!("/ep{i:02}/7?name=bob"),
                headers: BTreeMap::new(),
                body: None,
                body_base64: None,
                body_file: None,
            })
            .collect();
        let (t, dropped) = traps(&bases);
        assert_eq!(t.len(), app::MAX_TRAPS);
        assert!(dropped > 0);
        // Unknown-route probe is always kept even when traps exceed MAX_TRAPS.
        assert!(
            t.iter()
                .any(|r| r.method == "GET" && r.path == "/rust-anything-missing"),
            "unknown-route trap was dropped"
        );
        // Every single endpoint gets traps round-robin (not just the first few).
        for i in 0..40 {
            let prefix = format!("/ep{i:02}/");
            let count = t.iter().filter(|r| r.path.starts_with(&prefix)).count();
            assert!(
                count >= 7,
                "{prefix} only got {count} traps out of {}",
                app::MAX_TRAPS
            );
        }
        // All emitted traps are unique and distinct from bases.
        let keys: BTreeSet<String> = t.iter().map(req_key).collect();
        assert_eq!(keys.len(), t.len());
        for b in &bases {
            assert!(!keys.contains(&req_key(b)));
        }

        // Colliding traps across many bases (> 300 raw, < 300 unique) deduplicate before
        // truncating so nothing is dropped and later endpoints still get traps.
        let mut colliding: Vec<Req> = (1..=40)
            .map(|i| Req {
                method: "GET".into(),
                path: format!("/items/{i}"),
                headers: BTreeMap::new(),
                body: None,
                body_base64: None,
                body_file: None,
            })
            .collect();
        colliding.push(Req {
            method: "GET".into(),
            path: "/tail/hello".into(),
            headers: BTreeMap::new(),
            body: None,
            body_base64: None,
            body_file: None,
        });
        let (ct, cdropped) = traps(&colliding);
        assert_eq!(cdropped, 0);
        assert!(ct.iter().any(|r| r.path.starts_with("/tail/")));
        assert!(ct.iter().any(|r| r.path == "/rust-anything-missing"));
        let ckeys: BTreeSet<String> = ct.iter().map(req_key).collect();
        assert_eq!(ckeys.len(), ct.len());
    }

    #[test]
    fn parse_curl_supports_binary_files_and_multipart_forms() {
        let root = app::temp_dir("curl-binary-multipart").unwrap();
        let png = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0xff];
        std::fs::write(root.join("photo.png"), &png).unwrap();
        std::fs::write(root.join("note.txt"), "hello form").unwrap();

        let bin = parse_curl(
            "curl --data-binary @photo.png -H 'Content-Type: image/png' http://localhost:8080/upload",
            &root,
            &root,
        )
        .unwrap();
        assert_eq!(bin.method, "POST");
        assert!(bin.body.is_none());
        assert!(bin.body_base64.is_some());
        assert_eq!(bin.body_bytes().unwrap().as_deref(), Some(png.as_slice()));

        let multi = parse_curl(
            "curl -F title=avatar -F 'file=@photo.png;type=image/png;filename=pic.png' -F 'note=<note.txt' http://localhost:8080/multipart",
            &root,
            &root,
        )
        .unwrap();
        assert_eq!(multi.method, "POST");
        let ct = multi.headers.get("Content-Type").unwrap();
        assert!(ct.starts_with("multipart/form-data; boundary="), "{ct}");
        let raw = multi.body_bytes().unwrap().unwrap();
        assert!(find(&raw, b"name=\"title\"\r\n\r\navatar\r\n").is_some());
        assert!(
            find(
                &raw,
                b"name=\"file\"; filename=\"pic.png\"\r\nContent-Type: image/png\r\n\r\n"
            )
            .is_some()
        );
        assert!(find(&raw, &png).is_some());
        assert!(find(&raw, b"name=\"note\"\r\n\r\nhello form\r\n").is_some());

        // Quoted -F values and filenames containing `;`, plus --form-string literal preservation.
        std::fs::write(root.join("photo;final.png"), &png).unwrap();
        let quoted = parse_curl(
            "curl -H 'Content-Type: multipart/mixed; charset=utf-8' \
             -F 'colors=\"red; green; blue\";type=text/plain' \
             -F 'file=@\"photo;final.png\"' \
             --form-string 'raw=\"a; b\"' \
             http://localhost:8080/quoted",
            &root,
            &root,
        )
        .unwrap();
        let qct = quoted.headers.get("Content-Type").unwrap();
        assert!(
            qct.starts_with("multipart/mixed; charset=utf-8; boundary="),
            "{qct}"
        );
        let qraw = quoted.body_bytes().unwrap().unwrap();
        assert!(
            find(
                &qraw,
                b"name=\"colors\"\r\nContent-Type: text/plain\r\n\r\nred; green; blue\r\n"
            )
            .is_some()
        );
        assert!(
            find(
                &qraw,
                b"name=\"file\"; filename=\"photo;final.png\"\r\nContent-Type: image/png\r\n\r\n"
            )
            .is_some()
        );
        assert!(find(&qraw, b"name=\"raw\"\r\n\r\n\"a; b\"\r\n").is_some());

        // Caller-supplied `Content-Type: multipart/form-data` still gets the generated boundary.
        let hdr_ct = parse_curl(
            "curl -H 'Content-Type: multipart/form-data' -F a=1 http://localhost:8080/m",
            &root,
            &root,
        )
        .unwrap();
        let hct = hdr_ct.headers.get("Content-Type").unwrap();
        assert!(hct.starts_with("multipart/form-data; boundary="), "{hct}");

        // Mixing -d and -F is refused, as curl does.
        assert!(parse_curl("curl -d a=1 -F b=2 http://localhost:8080/mix", &root, &root).is_none());

        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn binary_request_bodies_round_trip_through_inputs_and_toml() {
        let root = app::temp_dir("http-binary-spec").unwrap();
        let bytes = vec![0x00, 0xff, 0x80, 0xfe, b'A', b'B'];
        let b64 = encode_base64(&bytes);
        std::fs::write(root.join("blob.bin"), &bytes).unwrap();
        std::fs::write(
            root.join("inputs.jsonl"),
            format!(
                "{{\"name\":\"b64\",\"method\":\"POST\",\"path\":\"/raw\",\"body_base64\":\"{b64}\"}}\n\
                 {{\"name\":\"file\",\"method\":\"POST\",\"path\":\"/raw\",\"body_file\":\"blob.bin\"}}\n"
            ),
        )
        .unwrap();

        let inputs = read_inputs(&root.join("inputs.jsonl"), &root).unwrap();
        assert_eq!(inputs.len(), 2);
        assert_eq!(
            inputs[0].1.body_bytes().unwrap().as_deref(),
            Some(bytes.as_slice())
        );
        assert_eq!(
            inputs[1].1.body_bytes().unwrap().as_deref(),
            Some(bytes.as_slice())
        );

        // Round-trip through write_case + load_spec (body_base64) and hand-written body_file.
        let http_dir = root.join("http");
        std::fs::create_dir_all(&http_dir).unwrap();
        write_case(
            &http_dir,
            "b64",
            &inputs[0].1,
            200,
            &BTreeMap::new(),
            b"ok",
            false,
        )
        .unwrap();
        let loaded = load_spec(&http_dir, &http_dir.join("b64.toml")).unwrap();
        assert_eq!(
            loaded.req.body_bytes().unwrap().as_deref(),
            Some(bytes.as_slice())
        );

        std::fs::write(http_dir.join("payload.bin"), &bytes).unwrap();
        std::fs::write(
            http_dir.join("from-file.toml"),
            "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody_file = \"payload.bin\"\n\n[response]\nstatus = 200\n",
        )
        .unwrap();
        let from_file = load_spec(&http_dir, &http_dir.join("from-file.toml")).unwrap();
        assert_eq!(
            from_file.req.body_bytes().unwrap().as_deref(),
            Some(bytes.as_slice())
        );

        // Reject traversal in body_file and invalid base64.
        std::fs::write(
            http_dir.join("escape.toml"),
            "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody_file = \"../blob.bin\"\n\n[response]\nstatus = 200\n",
        )
        .unwrap();
        assert!(load_spec(&http_dir, &http_dir.join("escape.toml")).is_err());

        // Reject symlink escaping http/.
        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(root.join("blob.bin"), http_dir.join("link.bin")).unwrap();
            std::fs::write(
                http_dir.join("symlink-escape.toml"),
                "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody_file = \"link.bin\"\n\n[response]\nstatus = 200\n",
            )
            .unwrap();
            assert!(load_spec(&http_dir, &http_dir.join("symlink-escape.toml")).is_err());
        }

        assert!(decode_base64_strict("not!valid!b64!").is_err());

        // Quoted Content-Type parameters containing `;` are preserved when replacing boundary.
        assert_eq!(
            with_multipart_boundary("multipart/form-data; profile=\"a;b\"; boundary=old", "new"),
            "multipart/form-data; profile=\"a;b\"; boundary=new"
        );
        // Unsupported curl `-F '...;headers=...'` and `encoder=` are declined cleanly.
        assert!(
            parse_curl(
                "curl -F 'submit=OK;headers=\"X-Submit-Type: OK\"' http://localhost:8080/h",
                &root,
                &root,
            )
            .is_none()
        );
        assert!(
            parse_curl(
                "curl -F 'note=hello;type=text/plain;encoder=base64' http://localhost:8080/h",
                &root,
                &root,
            )
            .is_none()
        );
        // Parameters following `;type=` belong to the part Content-Type, not the field value.
        let typed = parse_curl(
            "curl -F 'note=hello;type=text/plain;charset=UTF-8' http://localhost:8080/typed",
            &root,
            &root,
        )
        .unwrap();
        let typed_raw = typed.body_bytes().unwrap().unwrap();
        assert!(
            find(
                &typed_raw,
                b"name=\"note\"\r\nContent-Type: text/plain;charset=UTF-8\r\n\r\nhello\r\n"
            )
            .is_some()
        );
        // Non-string TOML body declarations are rejected before exclusivity checks.
        std::fs::write(
            http_dir.join("bad-type.toml"),
            "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody = \"fallback\"\nbody_base64 = 42\n\n[response]\nstatus = 200\n",
        )
        .unwrap();
        assert!(load_spec(&http_dir, &http_dir.join("bad-type.toml")).is_err());
        std::fs::write(
            http_dir.join("bad-file-type.toml"),
            "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody_file = 42\n\n[response]\nstatus = 200\n",
        )
        .unwrap();
        assert!(load_spec(&http_dir, &http_dir.join("bad-file-type.toml")).is_err());
        // Non-regular body_file (e.g. directory) is rejected.
        std::fs::create_dir_all(http_dir.join("subdir")).unwrap();
        std::fs::write(
            http_dir.join("dir-body.toml"),
            "[request]\nmethod = \"POST\"\npath = \"/raw\"\nbody_file = \"subdir\"\n\n[response]\nstatus = 200\n",
        )
        .unwrap();
        assert!(load_spec(&http_dir, &http_dir.join("dir-body.toml")).is_err());

        std::fs::remove_dir_all(root).unwrap();
    }
}
