use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use serde::{Deserialize, Serialize};

pub const EXT: &str = "effects";
const IO_TIMEOUT: Duration = Duration::from_secs(5);
const SETTLE: Duration = Duration::from_secs(2);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HttpResponseRule {
    /// HTTP method token (case-sensitive per RFC 9110), or `*` to match any method.
    pub method: String,
    /// Path pattern (leading `/`), supporting `*` wildcards.
    pub path: String,
    pub status: u16,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub body: String,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub body_base64: bool,
}

fn is_http_token(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
}

impl HttpResponseRule {
    /// Parses `[METHOD ]/path=STATUS[ BODY|@FILE]`, loading `@FILE` relative to `repo`.
    pub fn parse(spec: &str, repo: &Path) -> Result<Self> {
        let trimmed = spec.trim_start();
        let (first, after_first) = match trimmed.find(char::is_whitespace) {
            Some(i) => (&trimmed[..i], trimmed[i..].trim_start()),
            None => (trimmed, ""),
        };
        let (method, path_and_status, rest) = if first.starts_with('/') {
            ("*".to_string(), first, after_first)
        } else {
            let (p_and_s, rest) = match after_first.find(char::is_whitespace) {
                Some(i) => (&after_first[..i], after_first[i..].trim_start()),
                None => (after_first, ""),
            };
            (first.to_string(), p_and_s, rest)
        };
        if method != "*" && !is_http_token(&method) {
            bail!("--effect-response `{spec}`: invalid HTTP method `{method}`");
        }
        let (path, status_str) = path_and_status.rsplit_once('=').with_context(|| {
            format!("--effect-response `{spec}`: give `METHOD /path=STATUS [BODY|@FILE]`")
        })?;
        if !path.starts_with('/') {
            bail!("--effect-response `{spec}`: path `{path}` must start with `/`");
        }
        let status: u16 = status_str
            .parse()
            .ok()
            .filter(|&s| (200..=599).contains(&s))
            .with_context(|| {
                format!(
                    "--effect-response `{spec}`: invalid HTTP status `{status_str}` (expected 200-599)"
                )
            })?;

        let (body, body_base64, content_type) = if rest.is_empty() {
            (String::new(), false, None)
        } else if let Some(file_spec) = rest.strip_prefix('@') {
            let rel = file_spec.trim();
            if rel.is_empty() {
                bail!("--effect-response `{spec}`: missing file path after `@`");
            }
            let file_path = if Path::new(rel).is_absolute() {
                std::path::PathBuf::from(rel)
            } else {
                repo.join(rel)
            };
            let bytes = std::fs::read(&file_path).with_context(|| {
                format!(
                    "--effect-response `{spec}`: cannot read `{}`",
                    file_path.display()
                )
            })?;
            let ext = file_path
                .extension()
                .and_then(|e| e.to_str())
                .map(str::to_ascii_lowercase);
            match String::from_utf8(bytes) {
                Ok(text) => {
                    let ct = infer_content_type(ext.as_deref(), &text);
                    (text, false, Some(ct.to_string()))
                }
                Err(err) => {
                    let raw = err.into_bytes();
                    let ct = match ext.as_deref() {
                        Some("json") => "application/json",
                        Some("xml") => "application/xml",
                        Some("html" | "htm") => "text/html; charset=utf-8",
                        _ => "application/octet-stream",
                    };
                    (encode_base64(&raw), true, Some(ct.to_string()))
                }
            }
        } else {
            let ct = infer_content_type(None, rest);
            (rest.to_string(), false, Some(ct.to_string()))
        };

        Ok(Self {
            method,
            path: path.to_string(),
            status,
            content_type,
            body,
            body_base64,
        })
    }

    fn matches(&self, method: &str, target: &str) -> bool {
        if self.method != "*" && self.method != method {
            return false;
        }
        let req_path = request_path(target);
        let candidate = if self.path.contains('?') {
            req_path.as_ref()
        } else {
            req_path.split('?').next().unwrap_or(req_path.as_ref())
        };
        glob_match(&self.path, candidate)
    }

    fn response_bytes(&self, request_method: &str) -> Vec<u8> {
        let body = if self.body_base64 {
            decode_base64(self.body.as_bytes())
        } else {
            self.body.as_bytes().to_vec()
        };
        let mut head = format!(
            "HTTP/1.1 {} {}\r\n",
            self.status,
            status_reason(self.status)
        );
        let no_body_status = matches!(self.status, 204 | 205 | 304);
        let is_head = request_method == "HEAD";
        if !no_body_status && let Some(ct) = &self.content_type {
            head.push_str(&format!("Content-Type: {ct}\r\n"));
        }
        if no_body_status || is_head {
            head.push_str("Connection: close\r\n\r\n");
            return head.into_bytes();
        }
        head.push_str(&format!(
            "Content-Length: {}\r\nConnection: close\r\n\r\n",
            body.len()
        ));
        let mut out = head.into_bytes();
        out.extend_from_slice(&body);
        out
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EffectsConfig {
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub http: bool,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub smtp: bool,
    /// Mail is recorded as decoded headers and MIME parts. Configs saved before it existed leave
    /// it out and keep recording the raw DATA, so their cases still replay.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub smtp_parts: bool,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: BTreeMap<String, String>,
    /// Canned responses for the HTTP effect sink, matched in order; unmatched requests get `200 {}`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub http_responses: Vec<HttpResponseRule>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_reset: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub db_dump: Option<String>,
    /// Whether `--effects` named the kinds. `--effect-env` turns a sink on too, but that is inferred
    /// from a variable, not a request to replace the saved kinds.
    #[serde(skip)]
    pub named_kinds: bool,
}

const ENV_KINDS: [&str; 4] = ["http-url", "smtp-host", "smtp-port", "smtp-url"];

impl EffectsConfig {
    pub fn is_empty(&self) -> bool {
        !self.http && !self.smtp && self.db_dump.is_none()
    }

    pub fn serial(&self) -> bool {
        self.db_reset.is_some() || self.db_dump.is_some()
    }

    pub fn from_args(
        kinds: &[String],
        env: &[String],
        db_reset: Option<&str>,
        db_dump: Option<&str>,
    ) -> Result<Option<Self>> {
        Self::from_args_with_responses(kinds, env, &[], Path::new("."), db_reset, db_dump)
    }

    pub fn from_args_with_responses(
        kinds: &[String],
        env: &[String],
        responses: &[String],
        repo: &Path,
        db_reset: Option<&str>,
        db_dump: Option<&str>,
    ) -> Result<Option<Self>> {
        let mut config = EffectsConfig::default();
        let mut raw = false;
        for kind in kinds.iter().flat_map(|k| k.split(',')).map(str::trim) {
            config.named_kinds |= !kind.is_empty();
            match kind {
                "http" => config.http = true,
                "smtp" => config.smtp = true,
                "smtp-raw" => {
                    config.smtp = true;
                    raw = true;
                }
                "" => {}
                other => bail!("unknown effect `{other}`: use http, smtp or smtp-raw"),
            }
        }
        for pair in env {
            let (var, kind) = pair
                .split_once('=')
                .with_context(|| format!("--effect-env `{pair}`: give VAR=KIND"))?;
            if !ENV_KINDS.contains(&kind) {
                bail!(
                    "--effect-env `{pair}`: KIND is one of {}",
                    ENV_KINDS.join(", ")
                );
            }
            if kind.starts_with("http") {
                config.http = true;
            } else {
                config.smtp = true;
            }
            config.env.insert(var.to_string(), kind.to_string());
        }
        for spec in responses {
            config
                .http_responses
                .push(HttpResponseRule::parse(spec, repo)?);
            config.http = true;
        }
        config.smtp_parts = config.smtp && !raw;
        config.db_reset = db_reset.map(str::to_string);
        config.db_dump = db_dump.map(str::to_string);
        Ok((!config.is_empty() || config.db_reset.is_some()).then_some(config))
    }

    /// Merges field by field, as `http::merge_config` does for `--port` and `--arg`: a flag replaces
    /// only what it names, so re-recording with one effect flag keeps the others it saved before.
    pub fn merge(saved: Option<Self>, given: Option<Self>) -> Option<Self> {
        let (mut saved, given) = match (saved, given) {
            (Some(saved), Some(given)) => (saved, given),
            (saved, given) => return saved.or(given),
        };
        // `--effects` names the kinds, so it replaces them. `--effect-env` also turns a sink on, but
        // that is inferred: it must not switch the other sink off or change the saved mail mode.
        if given.named_kinds {
            saved.http = given.http;
            saved.smtp = given.smtp;
            saved.smtp_parts = given.smtp_parts;
        } else {
            if given.smtp && !saved.smtp {
                saved.smtp_parts = given.smtp_parts;
            }
            saved.http |= given.http;
            saved.smtp |= given.smtp;
        }
        // `--effect-env` adds or replaces one variable at a time.
        for (var, kind) in given.env {
            saved.env.insert(var, kind);
        }
        // The rules are tried in order, so the ones given replace the saved list as a whole.
        if !given.http_responses.is_empty() {
            saved.http_responses = given.http_responses;
        }
        if given.db_reset.is_some() {
            saved.db_reset = given.db_reset;
        }
        if given.db_dump.is_some() {
            saved.db_dump = given.db_dump;
        }
        (!saved.is_empty() || saved.db_reset.is_some()).then_some(saved)
    }
}

/// Two configs are equal when they save the same effects: `named_kinds` records how this run was
/// called, not what is kept, and it is not serialized.
impl PartialEq for EffectsConfig {
    fn eq(&self, other: &Self) -> bool {
        self.http == other.http
            && self.smtp == other.smtp
            && self.smtp_parts == other.smtp_parts
            && self.env == other.env
            && self.http_responses == other.http_responses
            && self.db_reset == other.db_reset
            && self.db_dump == other.db_dump
    }
}

struct Sink {
    port: u16,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

pub struct Sinks {
    config: EffectsConfig,
    events: Arc<Mutex<Vec<String>>>,
    active: Arc<AtomicUsize>,
    http: Option<Sink>,
    smtp: Option<Sink>,
}

impl Sinks {
    pub fn start(config: &EffectsConfig) -> Result<Sinks> {
        if config.http
            && let Some(rule) = config
                .http_responses
                .iter()
                .find(|r| !(200..=599).contains(&r.status))
        {
            bail!(
                "saved effect response `{} {}` has invalid HTTP status {} (expected 200-599)",
                rule.method,
                rule.path,
                rule.status
            );
        }
        let events = Arc::new(Mutex::new(Vec::new()));
        let active = Arc::new(AtomicUsize::new(0));
        let http = config
            .http
            .then(|| {
                let rules = Arc::new(config.http_responses.clone());
                listen(&events, &active, move |stream, ev| {
                    serve_http(stream, ev, &rules);
                })
            })
            .transpose()?;
        let smtp_parts = config.smtp_parts;
        let smtp = config
            .smtp
            .then(|| {
                listen(&events, &active, move |stream, ev| {
                    if smtp_parts {
                        serve_smtp(stream, ev);
                    } else {
                        serve_smtp_raw(stream, ev);
                    }
                })
            })
            .transpose()?;
        Ok(Sinks {
            config: config.clone(),
            events,
            active,
            http,
            smtp,
        })
    }

    pub fn env(&self) -> Vec<(String, String)> {
        let mut out = Vec::new();
        if let Some(h) = &self.http {
            let url = format!("http://127.0.0.1:{}", h.port);
            for var in ["RUST_ANYTHING_HTTP_SINK", "HTTP_PROXY", "http_proxy"] {
                out.push((var.to_string(), url.clone()));
            }
        }
        if let Some(s) = &self.smtp {
            out.push(("SMTP_HOST".into(), "127.0.0.1".into()));
            out.push(("SMTP_PORT".into(), s.port.to_string()));
            out.push((
                "RUST_ANYTHING_SMTP_SINK".into(),
                format!("smtp://127.0.0.1:{}", s.port),
            ));
        }
        for (var, kind) in &self.config.env {
            let value = match (kind.as_str(), &self.http, &self.smtp) {
                ("http-url", Some(h), _) => format!("http://127.0.0.1:{}", h.port),
                ("smtp-host", _, Some(_)) => "127.0.0.1".to_string(),
                ("smtp-port", _, Some(s)) => s.port.to_string(),
                ("smtp-url", _, Some(s)) => format!("smtp://127.0.0.1:{}", s.port),
                _ => continue,
            };
            out.push((var.clone(), value));
        }
        out
    }

    /// Runs `--db-reset` in `cwd`, the folder the case or server runs in.
    pub fn reset(&self, cwd: &Path, repo: &Path) -> Result<()> {
        if let Some(cmd) = &self.config.db_reset {
            let out = shell(cmd, cwd, repo, &self.env())?;
            if !out.status.success() {
                bail!(
                    "--db-reset `{cmd}` failed ({}): {}",
                    out.status,
                    String::from_utf8_lossy(&out.stderr).trim()
                );
            }
        }
        Ok(())
    }

    /// Collects the effects and runs `--db-dump` in `cwd`, the folder the case or server ran in.
    pub fn take(&self, cwd: &Path, repo: &Path) -> Vec<u8> {
        let deadline = Instant::now() + SETTLE;
        while self.active.load(Ordering::SeqCst) > 0 && Instant::now() < deadline {
            std::thread::sleep(Duration::from_millis(10));
        }
        let mut text: String = self
            .events
            .lock()
            .map(|mut e| e.drain(..).collect())
            .unwrap_or_default();
        if let Some(cmd) = &self.config.db_dump {
            text.push_str("db\n");
            match shell(cmd, cwd, repo, &self.env()) {
                Ok(out) => {
                    text.push_str(&String::from_utf8_lossy(&out.stdout));
                    if !out.status.success() {
                        text.push_str(&format!("(db dump failed: {})\n", out.status));
                    }
                }
                Err(e) => text.push_str(&format!("(db dump failed: {e:#})\n")),
            }
        }
        text.into_bytes()
    }
}

impl Drop for Sinks {
    fn drop(&mut self) {
        for sink in [self.http.as_mut(), self.smtp.as_mut()]
            .into_iter()
            .flatten()
        {
            sink.stop.store(true, Ordering::SeqCst);
            if let Some(t) = sink.thread.take() {
                let _ = t.join();
            }
        }
    }
}

/// Extensions of the files `with_repo_scripts` points at the repository.
const SCRIPTS: [&str; 14] = [
    "py", "sh", "bash", "js", "mjs", "cjs", "ts", "rb", "php", "pl", "ps1", "bat", "cmd", "sql",
];

/// The command runs where the case runs, so a script it names by a relative path (`reset.py`,
/// `"my scripts/reset.py"`, `< schema.sql`) is pointed at the repository's copy. Other words, such
/// as `app.db`, the target of an output redirect (`> snapshot.sql`), a here-string and the body of a
/// here-document stay as written: they name the case's own files or are input. Everything else in
/// the command is kept as written.
fn with_repo_scripts(cmd: &str, repo: &Path) -> String {
    let mut out = String::with_capacity(cmd.len());
    let mut kept = 0;
    let mut skip_next = false;
    for word in shell_words(cmd) {
        if word.op {
            // The word after `>`, `>>` or `2>` is an output, after `<<` a here-document's
            // delimiter, after `<<<` a here-string: none is a file to read.
            skip_next = word.text.contains('>') || word.text.starts_with("<<");
            continue;
        }
        if std::mem::take(&mut skip_next) {
            continue;
        }
        let path = word.text.as_str();
        let script = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .is_some_and(|e| SCRIPTS.contains(&e.to_ascii_lowercase().as_str()));
        let in_repo = repo.join(path);
        if script && Path::new(path).is_relative() && in_repo.is_file() {
            out.push_str(&cmd[kept..word.start]);
            out.push_str(&quoted(&in_repo));
            kept = word.end;
        }
    }
    out.push_str(&cmd[kept..]);
    out
}

/// A word of a shell command: where it sits in the command, its text without quotes, and whether
/// it is a redirect operator (`<`, `>`, `>>`, `<<`, `<<-`, `<<<`) rather than a word.
struct ShellWord {
    start: usize,
    end: usize,
    text: String,
    op: bool,
}

/// Splits a command into words as the shell does: whitespace, `;`, `&` and `|` separate words
/// outside quotes, an unquoted `<` or `>` starts a redirect operator of its own, and quotes group
/// without being part of the text. The body of a here-document is input, so it gives no words.
/// cmd quotes with `"` and escapes with `^`; sh also quotes with `'`, escapes with a backslash, and
/// skips a comment (`#` at the start of a word) to the end of the line.
fn shell_words(cmd: &str) -> Vec<ShellWord> {
    shell_words_as(cmd, cfg!(not(windows)))
}

/// `shell_words` as sh (`unix`) or cmd reads the command.
fn shell_words_as(cmd: &str, unix: bool) -> Vec<ShellWord> {
    let mut words = Vec::new();
    let mut word: Option<ShellWord> = None;
    let mut quote = None;
    // Here-documents whose bodies start on the next line: the delimiter, and whether it is `<<-`,
    // which strips leading tabs.
    let mut heredocs = Vec::new();
    // cmd blocks open, `(` at the start of a word, and not yet closed.
    let mut blocks = 0usize;
    let mut i = 0;
    while let Some(c) = cmd[i..].chars().next() {
        let len = c.len_utf8();
        // A line continuation (`\` in sh, `^` in cmd, then a newline) joins the lines before the
        // command is split into words, so it is part of no word and makes none of its own.
        let escape = if unix { '\\' } else { '^' };
        if quote.is_none() && c == escape {
            let rest = &cmd[i + len..];
            if let Some(newline) = ["\r\n", "\n"].into_iter().find(|n| rest.starts_with(n)) {
                i += len + newline.len();
                continue;
            }
        }
        let in_op = word.as_ref().is_some_and(|w| w.op);
        let redirect = quote.is_none() && matches!(c, '<' | '>');
        // sh groups commands with `(` and `)`. cmd does with a `(` that starts a word and the `)`
        // that closes it; elsewhere a parenthesis is part of a name (`my(1).py`).
        let opens = !unix && quote.is_none() && c == '(' && word.is_none();
        let closes = !unix && quote.is_none() && c == ')' && blocks > 0;
        let separator = quote.is_none()
            && (c.is_whitespace()
                || matches!(c, ';' | '&' | '|')
                || (unix && matches!(c, '(' | ')'))
                || opens
                || closes);
        if opens {
            blocks += 1;
        } else if closes {
            blocks -= 1;
        }
        let op_goes_on = redirect || (c == '-' && word.as_ref().is_some_and(|w| w.text == "<<"));
        if separator || (redirect && !in_op) || (in_op && !op_goes_on) {
            finish(&mut words, &mut heredocs, word.take());
        }
        // sh: an unquoted `#` that starts a word comments out the rest of the line.
        if unix && c == '#' && quote.is_none() && word.is_none() {
            i = cmd[i..].find('\n').map_or(cmd.len(), |n| i + n);
            continue;
        }
        if separator {
            i += len;
            if c == '\n' && !heredocs.is_empty() {
                i = past_heredocs(cmd, i, &mut heredocs);
            }
            continue;
        }
        let w = word.get_or_insert_with(|| ShellWord {
            start: i,
            end: i,
            text: String::new(),
            op: redirect,
        });
        match (quote, c) {
            (Some(q), c) if c == q => quote = None,
            // sh: in double quotes a backslash escapes only these, and drops a newline.
            (Some('"'), '\\') if unix => match cmd[i + len..].chars().next() {
                Some(next @ ('$' | '`' | '"' | '\\' | '\n')) => {
                    if next != '\n' {
                        w.text.push(next);
                    }
                    i += len + next.len_utf8();
                    w.end = i;
                    continue;
                }
                _ => w.text.push('\\'),
            },
            (Some(_), c) => w.text.push(c),
            (None, '"') => quote = Some('"'),
            (None, '\'') if unix => quote = Some('\''),
            // A backslash (sh) or caret (cmd) escapes the next character.
            (None, c) if c == escape => {
                if let Some(next) = cmd[i + len..].chars().next() {
                    w.text.push(next);
                    i += len + next.len_utf8();
                    w.end = i;
                    continue;
                }
            }
            (None, c) => w.text.push(c),
        }
        i += len;
        w.end = i;
    }
    finish(&mut words, &mut heredocs, word);
    words
}

/// Ends a word; a word right after `<<` or `<<-` is the delimiter of a here-document.
fn finish(words: &mut Vec<ShellWord>, heredocs: &mut Vec<(String, bool)>, word: Option<ShellWord>) {
    let Some(w) = word.filter(|w| w.end > w.start) else {
        return;
    };
    let after = words
        .last()
        .filter(|p| p.op && !w.op)
        .map(|p| p.text.as_str());
    if let Some(op @ ("<<" | "<<-")) = after {
        heredocs.push((w.text.clone(), op == "<<-"));
    }
    words.push(w);
}

/// Skips the bodies of the pending here-documents, which start at `from`, through each closing
/// delimiter line, and returns where the command goes on.
fn past_heredocs(cmd: &str, mut from: usize, heredocs: &mut Vec<(String, bool)>) -> usize {
    for (delimiter, tabs) in heredocs.drain(..) {
        while from < cmd.len() {
            let end = cmd[from..].find('\n').map_or(cmd.len(), |n| from + n);
            let line = cmd[from..end].trim_end_matches('\r');
            let line = if tabs {
                line.trim_start_matches('\t')
            } else {
                line
            };
            from = (end + 1).min(cmd.len());
            if line == delimiter {
                break;
            }
        }
    }
    from
}

#[cfg(windows)]
fn quoted(path: &Path) -> String {
    format!("\"{}\"", path.display())
}

#[cfg(not(windows))]
fn quoted(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', r"'\''"))
}

fn shell(
    cmd: &str,
    cwd: &Path,
    repo: &Path,
    env: &[(String, String)],
) -> Result<crate::process::Output> {
    let cmd = with_repo_scripts(cmd, repo);
    #[cfg(windows)]
    let mut c = {
        use std::os::windows::process::CommandExt as _;
        // `/S` strips exactly the outer quotes, so the command keeps the quotes `quoted` added;
        // `arg` would escape them as `\"`, which cmd does not read.
        let mut c = Command::new("cmd");
        c.args(["/S", "/C"]).raw_arg(format!("\"{cmd}\""));
        c
    };
    #[cfg(not(windows))]
    let mut c = {
        let mut c = Command::new("sh");
        c.args(["-c", &cmd]);
        c
    };
    c.current_dir(cwd)
        .envs(env.iter().map(|(k, v)| (k, v)))
        .stdin(std::process::Stdio::null());
    crate::process::output(&mut c, crate::process::QUICK)
}

fn listen<F>(
    events: &Arc<Mutex<Vec<String>>>,
    active: &Arc<AtomicUsize>,
    handler: F,
) -> Result<Sink>
where
    F: Fn(TcpStream, &Mutex<Vec<String>>) + Send + Sync + 'static,
{
    let listener = TcpListener::bind("127.0.0.1:0").context("starting an effect sink")?;
    listener.set_nonblocking(true)?;
    let port = listener.local_addr()?.port();
    let stop = Arc::new(AtomicBool::new(false));
    let (events, active, flag) = (events.clone(), active.clone(), stop.clone());
    let handler = Arc::new(handler);
    let thread = std::thread::spawn(move || {
        while !flag.load(Ordering::SeqCst) {
            match listener.accept() {
                Ok((stream, _)) => {
                    let _ = stream.set_nonblocking(false);
                    let _ = stream.set_read_timeout(Some(IO_TIMEOUT));
                    let (events, active, handler) =
                        (events.clone(), active.clone(), handler.clone());
                    active.fetch_add(1, Ordering::SeqCst);
                    std::thread::spawn(move || {
                        handler(stream, &events);
                        active.fetch_sub(1, Ordering::SeqCst);
                    });
                }
                Err(_) => std::thread::sleep(Duration::from_millis(5)),
            }
        }
    });
    Ok(Sink {
        port,
        stop,
        thread: Some(thread),
    })
}

fn push(events: &Mutex<Vec<String>>, event: String) {
    if let Ok(mut e) = events.lock() {
        e.push(event);
    }
}

fn body_text(body: &[u8]) -> String {
    match std::str::from_utf8(body) {
        Ok(t) => t.to_string(),
        Err(_) => format!("({} bytes)", body.len()),
    }
}

fn serve_http(mut stream: TcpStream, events: &Mutex<Vec<String>>, rules: &[HttpResponseRule]) {
    let mut reader = BufReader::new(match stream.try_clone() {
        Ok(s) => s,
        Err(_) => return,
    });
    let mut line = String::new();
    if reader.read_line(&mut line).unwrap_or(0) == 0 {
        return;
    }
    let mut parts = line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let target = parts.next().unwrap_or_default().to_string();
    let mut headers: Vec<(String, String)> = Vec::new();
    loop {
        let mut h = String::new();
        if reader.read_line(&mut h).unwrap_or(0) == 0 {
            break;
        }
        let h = h.trim_end();
        if h.is_empty() {
            break;
        }
        if let Some((k, v)) = h.split_once(':') {
            headers.push((k.trim().to_ascii_lowercase(), v.trim().to_string()));
        }
    }
    let header = |name: &str| {
        headers
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.clone())
    };
    if method.eq_ignore_ascii_case("CONNECT") {
        push(events, format!("https CONNECT {target}\n"));
        let _ = stream.write_all(
            b"HTTP/1.1 502 Bad Gateway\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        );
        return;
    }
    let mut body = Vec::new();
    if header("transfer-encoding").is_some_and(|t| t.eq_ignore_ascii_case("chunked")) {
        loop {
            let mut size = String::new();
            if reader.read_line(&mut size).unwrap_or(0) == 0 {
                break;
            }
            let n = usize::from_str_radix(size.trim().split(';').next().unwrap_or("0"), 16)
                .unwrap_or(0);
            if n == 0 {
                break;
            }
            let mut chunk = vec![0; n + 2];
            if reader.read_exact(&mut chunk).is_err() {
                break;
            }
            chunk.truncate(n);
            body.extend(chunk);
        }
    } else if let Some(n) = header("content-length").and_then(|v| v.parse::<usize>().ok()) {
        body.resize(n, 0);
        if reader.read_exact(&mut body).is_err() {
            body.clear();
        }
    }
    let matched = rules.iter().find(|r| r.matches(&method, &target));
    let url = if strip_http_scheme(&target).is_some() {
        target
    } else {
        format!("http://{}{target}", header("host").unwrap_or_default())
    };
    let url = match stream.local_addr() {
        Ok(local) => url
            .replace(&format!("127.0.0.1:{}", local.port()), "sink")
            .replace(&format!("localhost:{}", local.port()), "sink"),
        Err(_) => url,
    };
    let mut event = format!("http {method} {url}\n");
    if let Some(ct) = header("content-type") {
        event.push_str(&format!("content-type: {ct}\n"));
    }
    if !body.is_empty() {
        event.push('\n');
        event.push_str(&body_text(&body));
        if !event.ends_with('\n') {
            event.push('\n');
        }
    }
    push(events, event);
    let _ = stream.set_write_timeout(Some(IO_TIMEOUT));
    if let Some(rule) = matched {
        let _ = stream.write_all(&rule.response_bytes(&method));
    } else if method == "HEAD" {
        let _ = stream.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n",
        );
    } else {
        let _ = stream.write_all(
            b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 2\r\nConnection: close\r\n\r\n{}",
        );
    }
}

fn strip_http_scheme(target: &str) -> Option<&str> {
    let bytes = target.as_bytes();
    if bytes.len() >= 7 && bytes[..7].eq_ignore_ascii_case(b"http://") {
        Some(&target[7..])
    } else if bytes.len() >= 8 && bytes[..8].eq_ignore_ascii_case(b"https://") {
        Some(&target[8..])
    } else {
        None
    }
}

fn request_path(target: &str) -> std::borrow::Cow<'_, str> {
    if let Some(rest) = strip_http_scheme(target) {
        match rest.find(['/', '?']) {
            Some(i) if rest.as_bytes()[i] == b'/' => std::borrow::Cow::Borrowed(&rest[i..]),
            Some(i) => std::borrow::Cow::Owned(format!("/{}", &rest[i..])),
            None => std::borrow::Cow::Borrowed("/"),
        }
    } else if target.starts_with('?') {
        std::borrow::Cow::Owned(format!("/{target}"))
    } else {
        std::borrow::Cow::Borrowed(target)
    }
}

fn glob_match(pattern: &str, text: &str) -> bool {
    let (p, t) = (pattern.as_bytes(), text.as_bytes());
    let (mut pi, mut ti) = (0usize, 0usize);
    let mut star = None;
    let mut match_ti = 0usize;
    while ti < t.len() {
        if pi < p.len() && p[pi] == b'*' {
            star = Some(pi);
            match_ti = ti;
            pi += 1;
        } else if pi < p.len() && p[pi] == t[ti] {
            pi += 1;
            ti += 1;
        } else if let Some(s) = star {
            pi = s + 1;
            match_ti += 1;
            ti = match_ti;
        } else {
            return false;
        }
    }
    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }
    pi == p.len()
}

fn infer_content_type(ext: Option<&str>, text: &str) -> &'static str {
    match ext {
        Some("json") => "application/json",
        Some("xml") => "application/xml",
        Some("html" | "htm") => "text/html; charset=utf-8",
        Some("txt") => "text/plain; charset=utf-8",
        _ => {
            let trimmed = text.trim_start();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                "application/json"
            } else {
                "text/plain; charset=utf-8"
            }
        }
    }
}

fn status_reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        202 => "Accepted",
        204 => "No Content",
        301 => "Moved Permanently",
        302 => "Found",
        304 => "Not Modified",
        400 => "Bad Request",
        401 => "Unauthorized",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        410 => "Gone",
        422 => "Unprocessable Entity",
        429 => "Too Many Requests",
        500 => "Internal Server Error",
        502 => "Bad Gateway",
        503 => "Service Unavailable",
        504 => "Gateway Timeout",
        _ => "Status",
    }
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

fn decode_base64(input: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(input.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u8;
    for &b in input {
        let val = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            b'=' => break,
            _ => continue,
        };
        buf = (buf << 6) | u32::from(val);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buf >> bits) as u8);
        }
    }
    out
}

fn serve_smtp(stream: TcpStream, events: &Mutex<Vec<String>>) {
    serve_smtp_mode(stream, events, false);
}

fn serve_smtp_raw(stream: TcpStream, events: &Mutex<Vec<String>>) {
    serve_smtp_mode(stream, events, true);
}

fn serve_smtp_mode(stream: TcpStream, events: &Mutex<Vec<String>>, raw: bool) {
    let Ok(mut out) = stream.try_clone() else {
        return;
    };
    let mut reader = BufReader::new(stream);
    let mut say = |line: &str| out.write_all(format!("{line}\r\n").as_bytes()).is_ok();
    if !say("220 rust-anything ESMTP") {
        return;
    }
    let (mut from, mut to) = (String::new(), Vec::<String>::new());
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line).unwrap_or(0) == 0 {
            return;
        }
        let cmd = line.trim_end();
        let upper = cmd.to_ascii_uppercase();
        let ok = if upper.starts_with("EHLO") {
            say("250-rust-anything") && say("250 AUTH PLAIN LOGIN")
        } else if upper.starts_with("HELO") {
            say("250 rust-anything")
        } else if upper.starts_with("MAIL FROM:") {
            from = cmd[10..].trim().to_string();
            say("250 OK")
        } else if upper.starts_with("RCPT TO:") {
            to.push(cmd[8..].trim().to_string());
            say("250 OK")
        } else if upper == "DATA" {
            if !say("354 End data with <CR><LF>.<CR><LF>") {
                return;
            }
            let mut data = String::new();
            loop {
                let mut l = String::new();
                if reader.read_line(&mut l).unwrap_or(0) == 0 {
                    return;
                }
                let t = l.trim_end_matches(['\r', '\n']);
                if t == "." {
                    break;
                }
                data.push_str(
                    t.strip_prefix('.')
                        .filter(|_| t.starts_with(".."))
                        .unwrap_or(t),
                );
                data.push('\n');
            }
            let body = if raw {
                data
            } else {
                crate::mime::format_email(&data)
            };
            push(
                events,
                format!("smtp from {from} to {}\n{body}", to.join(", ")),
            );
            from.clear();
            to.clear();
            say("250 OK queued")
        } else if upper.starts_with("AUTH") {
            say("235 Authentication succeeded")
        } else if upper == "RSET" {
            from.clear();
            to.clear();
            say("250 OK")
        } else if upper == "QUIT" {
            let _ = say("221 Bye");
            return;
        } else if upper.starts_with("STARTTLS") {
            say("454 TLS not available")
        } else {
            say("250 OK")
        };
        if !ok {
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_scripts_of_the_repository_are_pointed_at_it() {
        let repo = std::env::temp_dir().join(format!("ra-effects-scripts-{}", std::process::id()));
        std::fs::create_dir_all(repo.join("scripts")).unwrap();
        for file in ["reset.py", "scripts/seed.sh", "app.db"] {
            std::fs::write(repo.join(file), "").unwrap();
        }
        let at = |file: &str| quoted(&repo.join(file));
        assert_eq!(
            with_repo_scripts("python3 reset.py --all", &repo),
            format!("python3 {} --all", at("reset.py"))
        );
        assert_eq!(
            with_repo_scripts("sh scripts/seed.sh && sqlite3 app.db .dump", &repo),
            format!("sh {} && sqlite3 app.db .dump", at("scripts/seed.sh"))
        );
        // Not in the repository, or not a relative path: left as written.
        assert_eq!(
            with_repo_scripts("python3 missing.py", &repo),
            "python3 missing.py"
        );
        let absolute = repo.join("reset.py").display().to_string();
        assert_eq!(
            with_repo_scripts(&format!("python3 {absolute}"), &repo),
            format!("python3 {absolute}")
        );
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn script_words_are_read_as_the_shell_reads_them() {
        let repo = std::env::temp_dir().join(format!("ra-effects-words-{}", std::process::id()));
        std::fs::create_dir_all(repo.join("my scripts")).unwrap();
        for file in [
            "reset.py",
            "my scripts/seed.py",
            "schema.sql",
            "snapshot.sql",
        ] {
            std::fs::write(repo.join(file), "").unwrap();
        }
        let at = |file: &str| quoted(&repo.join(file));
        // Quoted, and quoted with a space.
        assert_eq!(
            with_repo_scripts(r#"python3 "reset.py""#, &repo),
            format!("python3 {}", at("reset.py"))
        );
        assert_eq!(
            with_repo_scripts(r#"python3 "my scripts/seed.py" --all"#, &repo),
            format!("python3 {} --all", at("my scripts/seed.py"))
        );
        // An input is the repository's; an output stays in the case's folder.
        assert_eq!(
            with_repo_scripts("sqlite3 app.db < schema.sql", &repo),
            format!("sqlite3 app.db < {}", at("schema.sql"))
        );
        assert_eq!(
            with_repo_scripts("sqlite3 app.db <schema.sql", &repo),
            format!("sqlite3 app.db <{}", at("schema.sql"))
        );
        for cmd in [
            "sqlite3 app.db .dump > snapshot.sql",
            "sqlite3 app.db .dump >> snapshot.sql",
            "sqlite3 app.db .dump >snapshot.sql",
            "sqlite3 app.db .dump 2> snapshot.sql",
        ] {
            assert_eq!(with_repo_scripts(cmd, &repo), cmd);
        }
        // Spacing and operators around the words are kept as written.
        assert_eq!(
            with_repo_scripts("python3  reset.py;  echo  ok", &repo),
            format!("python3  {};  echo  ok", at("reset.py"))
        );
        std::fs::remove_dir_all(&repo).unwrap();
    }

    #[test]
    fn sh_continuations_escapes_and_parentheses_are_read_as_sh_reads_them() {
        let words = |cmd: &str| -> Vec<String> {
            shell_words_as(cmd, true)
                .into_iter()
                .map(|w| w.text)
                .collect()
        };
        // A line continuation is no part of any word, and makes none of its own.
        assert_eq!(words("python3 \\\nreset.py"), ["python3", "reset.py"]);
        assert_eq!(words("python3 \\\n  reset.py"), ["python3", "reset.py"]);
        assert_eq!(words("echo '' x"), ["echo", "", "x"]);
        let ops: Vec<(String, bool)> = shell_words_as("echo result > \\\n snapshot.sql", true)
            .into_iter()
            .map(|w| (w.text, w.op))
            .collect();
        assert_eq!(
            ops,
            [
                ("echo".to_string(), false),
                ("result".to_string(), false),
                (">".to_string(), true),
                ("snapshot.sql".to_string(), false)
            ]
        );
        assert_eq!(words("python3 re\\\nset.py"), ["python3", "reset.py"]);
        assert_eq!(words("python3 \"re\\\nset.py\""), ["python3", "reset.py"]);
        // Unquoted parentheses group commands; quoted ones are part of a name.
        assert_eq!(words("(python3 reset.py)"), ["python3", "reset.py"]);
        assert_eq!(words("python3 '(x).py'"), ["python3", "(x).py"]);
        // In double quotes a backslash escapes only `$`, `` ` ``, `"`, `\` and a newline.
        assert_eq!(words(r#"echo "a\"b" "c\d""#), ["echo", "a\"b", "c\\d"]);
        assert_eq!(words(r"echo 'a\b'"), ["echo", "a\\b"]);
    }

    #[test]
    fn cmd_blocks_and_carets_are_read_as_cmd_reads_them() {
        let words = |cmd: &str| -> Vec<String> {
            shell_words_as(cmd, false)
                .into_iter()
                .map(|w| w.text)
                .collect()
        };
        // A `(` that starts a word opens a block, and a `)` in a block closes it.
        assert_eq!(
            words("if exist app.db (python reset.py)"),
            ["if", "exist", "app.db", "python", "reset.py"]
        );
        assert_eq!(words("(python reset.py)"), ["python", "reset.py"]);
        // Outside a block, and quoted or escaped with `^`, parentheses are part of a name.
        assert_eq!(words("python my(1).py"), ["python", "my(1).py"]);
        assert_eq!(words(r#"python "a (1).py""#), ["python", "a (1).py"]);
        assert_eq!(words("(echo ^(x^))"), ["echo", "(x)"]);
    }

    #[test]
    fn sh_comments_give_no_words() {
        let words = |cmd: &str| -> Vec<String> {
            shell_words_as(cmd, true)
                .into_iter()
                .map(|w| w.text)
                .collect()
        };
        // A comment's `<<EOF` starts no here-document, so the next line is still read.
        assert_eq!(words("# <<EOF\npython3 reset.py"), ["python3", "reset.py"]);
        assert_eq!(
            words("python3 reset.py # then <<EOF\necho ok"),
            ["python3", "reset.py", "echo", "ok"]
        );
        // `#` inside a word or in quotes is text.
        assert_eq!(words("echo a#b '#c' \"#d\""), ["echo", "a#b", "#c", "#d"]);
        // cmd has no `#` comments.
        assert_eq!(
            shell_words_as("echo # x", false)
                .into_iter()
                .map(|w| w.text)
                .collect::<Vec<_>>(),
            ["echo", "#", "x"]
        );
    }

    #[test]
    fn redirects_and_here_documents_are_read_as_the_shell_reads_them() {
        let repo = std::env::temp_dir().join(format!("ra-effects-heredoc-{}", std::process::id()));
        std::fs::create_dir_all(&repo).unwrap();
        for file in ["reset.py", "schema.sql", "snapshot.sql"] {
            std::fs::write(repo.join(file), "").unwrap();
        }
        let at = |file: &str| quoted(&repo.join(file));
        // A redirect needs no space before it.
        assert_eq!(
            with_repo_scripts("sqlite3 app.db<schema.sql", &repo),
            format!("sqlite3 app.db<{}", at("schema.sql"))
        );
        for cmd in [
            "echo result> snapshot.sql",
            "echo result>snapshot.sql",
            "echo result>>snapshot.sql",
            "python3 check.py 2>&1 >snapshot.sql",
        ] {
            assert_eq!(with_repo_scripts(cmd, &repo), cmd);
        }
        // A here-document's body and a here-string are input, not words; the command goes on
        // after the delimiter.
        let heredoc = "cat <<EOF > notes.txt\nschema.sql\nEOF\npython3 reset.py";
        assert_eq!(
            with_repo_scripts(heredoc, &repo),
            format!(
                "cat <<EOF > notes.txt\nschema.sql\nEOF\npython3 {}",
                at("reset.py")
            )
        );
        let tabs = "cat <<-END\n\tschema.sql\n\tEND\npython3 reset.py";
        assert_eq!(
            with_repo_scripts(tabs, &repo),
            format!(
                "cat <<-END\n\tschema.sql\n\tEND\npython3 {}",
                at("reset.py")
            )
        );
        assert_eq!(
            with_repo_scripts("sqlite3 app.db <<< schema.sql", &repo),
            "sqlite3 app.db <<< schema.sql"
        );
        if cfg!(not(windows)) {
            let quoted_delim = "cat <<'EOF'\nschema.sql\nEOF\npython3 reset.py";
            assert_eq!(
                with_repo_scripts(quoted_delim, &repo),
                format!("cat <<'EOF'\nschema.sql\nEOF\npython3 {}", at("reset.py"))
            );
        }
        std::fs::remove_dir_all(&repo).unwrap();
    }

    fn send(port: u16, text: &str) -> String {
        let mut s = TcpStream::connect(("127.0.0.1", port)).unwrap();
        s.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        s.write_all(text.as_bytes()).unwrap();
        let mut out = String::new();
        let _ = s.read_to_string(&mut out);
        out
    }

    #[test]
    fn http_requests_and_mail_become_effects() {
        let config = EffectsConfig::from_args(
            &["http".into(), "smtp".into()],
            &["HOOK=http-url".into()],
            None,
            None,
        )
        .unwrap()
        .unwrap();
        let sinks = Sinks::start(&config).unwrap();
        let env: BTreeMap<String, String> = sinks.env().into_iter().collect();
        let http: u16 = env["HOOK"].rsplit(':').next().unwrap().parse().unwrap();
        let smtp: u16 = env["SMTP_PORT"].parse().unwrap();
        let reply = send(
            http,
            "POST http://hooks.example/new HTTP/1.1\r\nHost: hooks.example\r\nContent-Type: application/json\r\nContent-Length: 7\r\n\r\n{\"a\":1}",
        );
        assert!(reply.starts_with("HTTP/1.1 200"), "{reply}");
        let mut mail = TcpStream::connect(("127.0.0.1", smtp)).unwrap();
        mail.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        mail.write_all(
            b"EHLO me\r\nMAIL FROM:<a@x.test>\r\nRCPT TO:<b@y.test>\r\nDATA\r\nSubject: hi\r\n\r\n..dot\r\n.\r\nQUIT\r\n",
        )
        .unwrap();
        let mut out = String::new();
        let _ = mail.read_to_string(&mut out);
        assert!(out.contains("221"), "{out}");
        let text = String::from_utf8(sinks.take(Path::new("."), Path::new("."))).unwrap();
        assert_eq!(
            text,
            "http POST http://hooks.example/new\ncontent-type: application/json\n\n{\"a\":1}\n\
             smtp from <a@x.test> to <b@y.test>\nSubject: hi\npart text/plain\n.dot\n"
        );
        assert!(sinks.take(Path::new("."), Path::new(".")).is_empty());
    }

    #[test]
    fn smtp_raw_records_unparsed_mime_data() {
        let config = EffectsConfig::from_args(&["smtp-raw".into()], &[], None, None)
            .unwrap()
            .unwrap();
        assert!(config.smtp && !config.smtp_parts);
        let sinks = Sinks::start(&config).unwrap();
        let env: BTreeMap<String, String> = sinks.env().into_iter().collect();
        let smtp: u16 = env["SMTP_PORT"].parse().unwrap();
        let mut mail = TcpStream::connect(("127.0.0.1", smtp)).unwrap();
        mail.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        mail.write_all(
            b"EHLO me\r\nMAIL FROM:<a@x.test>\r\nRCPT TO:<b@y.test>\r\nDATA\r\nX-Custom: keep\r\nSubject: hi\r\n\r\n..dot\r\n.\r\nQUIT\r\n",
        )
        .unwrap();
        let mut out = String::new();
        let _ = mail.read_to_string(&mut out);
        assert!(out.contains("221"), "{out}");
        let text = String::from_utf8(sinks.take(Path::new("."), Path::new("."))).unwrap();
        assert_eq!(
            text,
            "smtp from <a@x.test> to <b@y.test>\nX-Custom: keep\nSubject: hi\n\n.dot\n"
        );
    }

    #[test]
    fn a_config_saved_before_smtp_parts_keeps_recording_raw_mail() {
        let old: EffectsConfig = serde_json::from_str(r#"{"smtp":true}"#).unwrap();
        assert!(old.smtp && !old.smtp_parts);
        let parts = EffectsConfig::from_args(&["smtp".into()], &[], None, None)
            .unwrap()
            .unwrap();
        assert!(parts.smtp_parts);
        let saved: EffectsConfig =
            serde_json::from_str(&serde_json::to_string(&parts).unwrap()).unwrap();
        assert_eq!(saved, parts);
        let env_only = EffectsConfig::from_args(&[], &["MAIL=smtp-url".into()], None, None)
            .unwrap()
            .unwrap();
        assert!(env_only.smtp_parts);
        let raw = EffectsConfig::from_args(&["smtp,smtp-raw".into()], &[], None, None)
            .unwrap()
            .unwrap();
        assert!(raw.smtp && !raw.smtp_parts);
    }

    #[test]
    fn effect_arguments_are_checked() {
        assert!(
            EffectsConfig::from_args(&[], &[], None, None)
                .unwrap()
                .is_none()
        );
        assert!(EffectsConfig::from_args(&["ftp".into()], &[], None, None).is_err());
        assert!(EffectsConfig::from_args(&[], &["X".into()], None, None).is_err());
        let c = EffectsConfig::from_args(&[], &["URL=smtp-url".into()], None, Some("cat db"))
            .unwrap()
            .unwrap();
        assert!(c.smtp && !c.http && c.serial());
    }

    #[test]
    fn http_effect_responses_match_method_wildcards_fixtures_and_fallback() {
        let dir =
            std::env::temp_dir().join(format!("rust-anything-http-resp-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("fixtures")).unwrap();
        std::fs::write(
            dir.join("fixtures/charge.json"),
            r#"{"id":"ch_123","ok":true}"#,
        )
        .unwrap();

        let config = EffectsConfig::from_args_with_responses(
            &[],
            &["API_URL=http-url".into()],
            &[
                "POST /v1/charges=201 @fixtures/charge.json".into(),
                "POST /hooks/ok=202 accepted".into(),
                "POST /hooks/*=500".into(),
                "GET /token=200 {\"token\":\"abc\"}".into(),
            ],
            &dir,
            None,
            None,
        )
        .unwrap()
        .unwrap();
        assert!(config.http);
        assert_eq!(config.http_responses.len(), 4);
        // Round-trip through JSON (as saved in rust-anything.json for replay).
        let saved: EffectsConfig =
            serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap();
        assert_eq!(saved, config);

        let sinks = Sinks::start(&saved).unwrap();
        let env: BTreeMap<String, String> = sinks.env().into_iter().collect();
        let port: u16 = env["API_URL"].rsplit(':').next().unwrap().parse().unwrap();

        // Exact match with @fixture via proxy-style absolute URL + query string.
        let r1 = send(
            port,
            "POST http://pay.example/v1/charges?dry=1 HTTP/1.1\r\nHost: pay.example\r\nContent-Length: 2\r\n\r\n{}",
        );
        assert!(r1.starts_with("HTTP/1.1 201 Created\r\n"), "{r1}");
        assert!(r1.contains("Content-Type: application/json\r\n"), "{r1}");
        assert!(r1.ends_with(r#"{"id":"ch_123","ok":true}"#), "{r1}");

        // First matching rule wins (`POST /hooks/ok` before `POST /hooks/*`).
        let r2 = send(
            port,
            "POST /hooks/ok HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
        );
        assert!(r2.starts_with("HTTP/1.1 202 Accepted\r\n"), "{r2}");
        assert!(r2.ends_with("accepted"), "{r2}");

        // Wildcard match with status only (empty body).
        let r3 = send(
            port,
            "POST /hooks/v2/fail HTTP/1.1\r\nHost: localhost\r\nContent-Length: 0\r\n\r\n",
        );
        assert!(
            r3.starts_with("HTTP/1.1 500 Internal Server Error\r\n"),
            "{r3}"
        );
        assert!(r3.contains("Content-Length: 0\r\n"), "{r3}");

        // Method mismatch falls back to default `200 {}`.
        let r4 = send(
            port,
            "GET /hooks/v2/fail HTTP/1.1\r\nHost: localhost\r\n\r\n",
        );
        assert!(r4.starts_with("HTTP/1.1 200 OK\r\n"), "{r4}");
        assert!(r4.ends_with("{}"), "{r4}");

        // Inline JSON body.
        let r5 = send(port, "GET /token HTTP/1.1\r\nHost: localhost\r\n\r\n");
        assert!(r5.starts_with("HTTP/1.1 200 OK\r\n"), "{r5}");
        assert!(r5.ends_with(r#"{"token":"abc"}"#), "{r5}");

        // Validation errors for malformed specs (including 1xx interim statuses).
        assert!(HttpResponseRule::parse("no-equals", &dir).is_err());
        assert!(HttpResponseRule::parse("POST relative=200", &dir).is_err());
        assert!(HttpResponseRule::parse("POST /x=103", &dir).is_err());
        assert!(HttpResponseRule::parse("POST /x=999", &dir).is_err());
        assert!(HttpResponseRule::parse("POST /x=200 @fixtures/missing.json", &dir).is_err());

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn http_effect_responses_follow_rfc_matching_and_body_rules() {
        let dir = Path::new(".");
        // Query patterns with `=` inside the query string.
        let q_rule =
            HttpResponseRule::parse("GET /items?mode=full=201 {\"ok\":true}", dir).unwrap();
        assert_eq!(q_rule.method, "GET");
        assert_eq!(q_rule.path, "/items?mode=full");
        assert_eq!(q_rule.status, 201);
        assert_eq!(q_rule.body, "{\"ok\":true}");
        assert!(q_rule.matches("GET", "/items?mode=full"));
        assert!(!q_rule.matches("GET", "/items?mode=short"));

        // Case-sensitive HTTP methods and RFC 9110 method tokens with punctuation.
        let upper = HttpResponseRule::parse("POST /x=201", dir).unwrap();
        let lower = HttpResponseRule::parse("post /x=410", dir).unwrap();
        assert!(upper.matches("POST", "/x"));
        assert!(!upper.matches("post", "/x"));
        assert!(lower.matches("post", "/x"));
        assert!(!lower.matches("POST", "/x"));

        let msearch = HttpResponseRule::parse("M-SEARCH /devices=201 ok", dir).unwrap();
        assert_eq!(msearch.method, "M-SEARCH");
        assert!(msearch.matches("M-SEARCH", "/devices"));

        // Wildcard `*` when input contains a literal `*`, and absolute URI with query before `/`.
        let star = HttpResponseRule::parse("GET /a*b=201", dir).unwrap();
        assert!(star.matches("GET", "/axb"));
        assert!(star.matches("GET", "/a*xb"));
        let abs_q = HttpResponseRule::parse("GET /?next/*=201", dir).unwrap();
        assert!(abs_q.matches("GET", "/?next/foo"));
        assert!(abs_q.matches("GET", "http://example.test?next/foo"));

        // RFC 9110 body & Content-Length suppression for 204, 304, and HEAD.
        let r204 = String::from_utf8(
            HttpResponseRule::parse("GET /x=204", dir)
                .unwrap()
                .response_bytes("GET"),
        )
        .unwrap();
        assert!(r204.starts_with("HTTP/1.1 204 No Content\r\n"), "{r204}");
        assert!(!r204.contains("Content-Length"), "{r204}");
        assert!(r204.ends_with("\r\n\r\n"), "{r204}");

        let r304 = String::from_utf8(
            HttpResponseRule::parse("GET /x=304 stale", dir)
                .unwrap()
                .response_bytes("GET"),
        )
        .unwrap();
        assert!(r304.starts_with("HTTP/1.1 304 Not Modified\r\n"), "{r304}");
        assert!(!r304.contains("Content-Length"), "{r304}");
        assert!(r304.ends_with("\r\n\r\n"), "{r304}");

        let r205 = String::from_utf8(
            HttpResponseRule::parse("GET /x=205 reset", dir)
                .unwrap()
                .response_bytes("GET"),
        )
        .unwrap();
        assert!(r205.starts_with("HTTP/1.1 205 "), "{r205}");
        assert!(!r205.contains("Content-Length"), "{r205}");
        assert!(r205.ends_with("\r\n\r\n"), "{r205}");

        let head_body = String::from_utf8(
            HttpResponseRule::parse("HEAD /x=200 sample", dir)
                .unwrap()
                .response_bytes("HEAD"),
        )
        .unwrap();
        assert!(!head_body.contains("Content-Length"), "{head_body}");
        assert!(head_body.ends_with("\r\n\r\n"), "{head_body}");

        let head_empty = String::from_utf8(
            HttpResponseRule::parse("HEAD /x=200", dir)
                .unwrap()
                .response_bytes("HEAD"),
        )
        .unwrap();
        assert!(!head_empty.contains("Content-Length"), "{head_empty}");
        assert!(head_empty.ends_with("\r\n\r\n"), "{head_empty}");

        // Trailing whitespace preserved in inline body, case-insensitive HTTP:// scheme,
        // and saved-config status validation in Sinks::start.
        let nl = HttpResponseRule::parse("GET /x=200 ready\n", dir).unwrap();
        assert_eq!(nl.body, "ready\n");
        assert!(nl.matches("GET", "HTTP://example.test/x"));
        assert!(nl.matches("GET", "HTTPS://example.test/x"));
        assert!(!nl.matches("GET", "/aaaaaé"));

        let mut bad_cfg = EffectsConfig {
            http: true,
            ..Default::default()
        };
        bad_cfg.http_responses.push(HttpResponseRule {
            method: "GET".into(),
            path: "/x".into(),
            status: 103,
            content_type: None,
            body: String::new(),
            body_base64: false,
        });
        assert!(Sinks::start(&bad_cfg).is_err());
    }
}

#[cfg(test)]
mod merge_tests {
    use super::*;

    #[test]
    fn a_second_record_with_one_flag_keeps_the_saved_effects() {
        // `--effects http --effect-env HOOK=http-url`, then only `--db-dump "echo x"`: the second run
        // must keep the sink and the variable, or the webhook stops being recorded (issue #331).
        let saved =
            EffectsConfig::from_args(&["http".into()], &["HOOK=http-url".into()], None, None);
        let given = EffectsConfig::from_args(&[], &[], None, Some("echo x"));
        let merged = EffectsConfig::merge(saved.unwrap(), given.unwrap()).unwrap();
        assert!(merged.http, "the HTTP sink survived: {merged:?}");
        assert_eq!(merged.env.get("HOOK").map(String::as_str), Some("http-url"));
        assert_eq!(merged.db_dump.as_deref(), Some("echo x"));
    }

    #[test]
    fn a_given_kind_list_replaces_the_saved_kinds() {
        let saved = EffectsConfig::from_args(&["http".into(), "smtp".into()], &[], None, None);
        let given = EffectsConfig::from_args(&["smtp".into()], &[], None, None);
        let merged = EffectsConfig::merge(saved.unwrap(), given.unwrap()).unwrap();
        assert!(
            !merged.http,
            "a named kind list replaces the kinds: {merged:?}"
        );
        assert!(merged.smtp);
    }

    #[test]
    fn an_environment_only_update_keeps_both_sinks_and_the_mail_mode() {
        // Saved: http and smtp, recorded as raw MIME. A run that only sets `HOOK` infers `http`, but
        // that is not a request to drop SMTP or to switch to decoded parts.
        let mut saved =
            EffectsConfig::from_args(&["http".into(), "smtp-raw".into()], &[], None, None)
                .unwrap()
                .unwrap();
        saved.env.insert("MAIL".into(), "smtp-host".into());
        let given = EffectsConfig::from_args(&[], &["HOOK=http-url".into()], None, None)
            .unwrap()
            .unwrap();
        let merged = EffectsConfig::merge(Some(saved), Some(given)).unwrap();
        assert!(merged.http && merged.smtp, "both sinks stay: {merged:?}");
        assert!(
            !merged.smtp_parts,
            "the saved raw mail mode stays: {merged:?}"
        );
        assert_eq!(merged.env.len(), 2, "{merged:?}");
        assert_eq!(merged.env.get("HOOK").map(String::as_str), Some("http-url"));
    }

    #[test]
    fn effect_responses_replace_the_saved_rules_and_survive_other_flags() {
        let repo = Path::new(".");
        let saved = EffectsConfig::from_args_with_responses(
            &["http".into()],
            &[],
            &["GET /a=404".into()],
            repo,
            None,
            None,
        )
        .unwrap();
        let env_only =
            EffectsConfig::from_args(&[], &["HOOK=http-url".into()], None, None).unwrap();
        let kept = EffectsConfig::merge(saved.clone(), env_only).unwrap();
        assert_eq!(kept.http_responses, saved.as_ref().unwrap().http_responses);
        let given = EffectsConfig::from_args_with_responses(
            &[],
            &[],
            &["POST /b=500".into()],
            repo,
            None,
            None,
        )
        .unwrap();
        let replaced = EffectsConfig::merge(saved, given).unwrap();
        assert_eq!(replaced.http_responses.len(), 1);
        assert_eq!(replaced.http_responses[0].path, "/b");
    }

    #[test]
    fn an_environment_update_turns_on_the_sink_it_needs() {
        let saved = EffectsConfig::from_args(&["http".into()], &[], None, None).unwrap();
        let given = EffectsConfig::from_args(&[], &["MAIL=smtp-host".into()], None, None).unwrap();
        let merged = EffectsConfig::merge(saved, given).unwrap();
        assert!(
            merged.http && merged.smtp,
            "the new mapping's sink is added: {merged:?}"
        );
        assert!(merged.smtp_parts, "{merged:?}");
    }

    #[test]
    fn an_effect_env_replaces_one_variable_and_keeps_the_others() {
        let saved = EffectsConfig::from_args(
            &["http".into()],
            &["HOOK=http-url".into(), "MAIL=smtp-host".into()],
            None,
            None,
        );
        let given = EffectsConfig::from_args(&[], &["HOOK=http-url".into()], None, None);
        let merged = EffectsConfig::merge(saved.unwrap(), given.unwrap()).unwrap();
        assert_eq!(merged.env.len(), 2, "{merged:?}");
        assert!(
            merged.http && merged.smtp,
            "an env update keeps both sinks: {merged:?}"
        );
    }

    #[test]
    fn nothing_given_keeps_everything_saved() {
        let saved =
            EffectsConfig::from_args(&["http".into()], &[], Some("rm x"), Some("echo y")).unwrap();
        let kept = saved.clone().unwrap();
        let merged = EffectsConfig::merge(saved, None).unwrap();
        assert_eq!(merged.http, kept.http);
        assert_eq!(merged.db_reset.as_deref(), Some("rm x"));
        assert_eq!(merged.db_dump.as_deref(), Some("echo y"));
        assert_eq!(EffectsConfig::merge(None, None), None);
    }
}
