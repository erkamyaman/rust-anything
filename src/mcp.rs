//! MCP server over stdio: every step as a tool, for any agent.
//!
//! Newline-delimited JSON-RPC 2.0, as the MCP stdio transport specifies. The protocol surface
//! is small (initialize, tools/list, tools/call, prompts/list, prompts/get, ping, progress and
//! cancellation), so it is implemented directly on serde_json instead of pulling in an SDK.
//!
//! Each tools/call runs on its own thread, so ping and notifications/cancelled are answered
//! while a long call runs; one writer thread owns stdout and writes whole lines.

use std::collections::HashMap;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex, PoisonError};
use std::thread::JoinHandle;

use anyhow::{Context, Result};
use serde_json::{Value, json};

use crate::{analyze, app, bridge, front_door, gate, harness, lift, shadow};

const PROTOCOL_VERSION: &str = "2025-06-18";

pub fn serve() -> Result<()> {
    serve_io(std::io::stdin().lock(), std::io::stdout())
}

/// In-flight tools/call requests by id, each with its cancelled flag.
type Running = Arc<Mutex<HashMap<String, Arc<AtomicBool>>>>;

fn serve_io(input: impl BufRead, output: impl Write + Send + 'static) -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<Value>();
    let writer = std::thread::spawn(move || -> std::io::Result<()> {
        let mut output = output;
        for message in rx {
            if message.is_null() {
                break;
            }
            let mut line = message.to_string();
            line.push('\n');
            output.write_all(line.as_bytes())?;
            output.flush()?;
        }
        Ok(())
    });
    let running = Running::default();
    let mut workers: Vec<(Arc<AtomicBool>, JoinHandle<()>)> = Vec::new();
    for line in input.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let request = match serde_json::from_str::<Value>(&line) {
            Ok(request) => request,
            Err(e) => {
                let _ = tx.send(error(Value::Null, -32700, &format!("parse error: {e}")));
                continue;
            }
        };
        match request["method"].as_str() {
            Some("tools/call") if request.get("id").is_some() => {
                workers.retain(|(_, worker)| !worker.is_finished());
                workers.push(spawn_call(request, tx.clone(), running.clone()));
            }
            Some("notifications/cancelled") => {
                let key = request["params"]["requestId"].to_string();
                if let Some(flag) = lock(&running).get(&key) {
                    flag.store(true, Ordering::SeqCst);
                }
            }
            _ => {
                if let Some(response) = handle(&request) {
                    let _ = tx.send(response);
                }
            }
        }
    }
    for (cancelled, worker) in workers {
        if !cancelled.load(Ordering::SeqCst) {
            let _ = worker.join();
        }
    }
    let _ = tx.send(Value::Null);
    writer
        .join()
        .map_err(|_| anyhow::anyhow!("the MCP writer thread panicked"))??;
    Ok(())
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    m.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Runs one tools/call on its own thread. A cancelled request gets no response and no further
/// progress; the check and the send happen under the `running` lock, so a cancellation lands
/// either before the response (nothing is sent) or after it (the request is already gone).
fn spawn_call(
    request: Value,
    tx: Sender<Value>,
    running: Running,
) -> (Arc<AtomicBool>, JoinHandle<()>) {
    let key = request["id"].to_string();
    let cancelled = Arc::new(AtomicBool::new(false));
    lock(&running).insert(key.clone(), cancelled.clone());
    let flag = cancelled.clone();
    let worker = std::thread::spawn(move || {
        let token = progress_token(&request);
        let name = request["params"]["name"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        if let Some(token) = &token {
            let _ = tx.send(progress(token, 0, &format!("running {name}")));
        }
        let response = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| handle(&request)))
            .unwrap_or_else(|_| {
                Some(error(
                    request["id"].clone(),
                    -32603,
                    &format!("tool `{name}` panicked"),
                ))
            });
        let mut running = lock(&running);
        if running.get(&key).is_some_and(|f| Arc::ptr_eq(f, &flag)) {
            running.remove(&key);
        }
        if flag.load(Ordering::SeqCst) {
            return;
        }
        if let Some(token) = &token {
            let failed = response
                .as_ref()
                .is_none_or(|r| r["result"]["isError"] != false);
            let done = if failed { "failed" } else { "finished" };
            let _ = tx.send(progress(token, 1, &format!("{name} {done}")));
        }
        if let Some(response) = response {
            let _ = tx.send(response);
        }
    });
    (cancelled, worker)
}

/// `params._meta.progressToken` when it is a string or an integer, as the spec requires.
fn progress_token(request: &Value) -> Option<Value> {
    let token = &request["params"]["_meta"]["progressToken"];
    (token.is_string() || token.is_i64() || token.is_u64()).then(|| token.clone())
}

fn progress(token: &Value, done: u64, message: &str) -> Value {
    json!({
        "jsonrpc": "2.0",
        "method": "notifications/progress",
        "params": { "progressToken": token, "progress": done, "total": 1, "message": message },
    })
}

/// The response to one JSON-RPC message; `None` for notifications.
pub fn handle(request: &Value) -> Option<Value> {
    let id = request.get("id").cloned();
    let method = request["method"].as_str().unwrap_or_default();
    let id = id?;
    let result = match method {
        "initialize" => json!({
            "protocolVersion": request["params"]["protocolVersion"].as_str().unwrap_or(PROTOCOL_VERSION),
            "capabilities": {
                "tools": {},
                "prompts": {},
                "resources": {},
                "extensions": { "io.modelcontextprotocol/skills": {} },
            },
            "serverInfo": { "name": "rust-anything", "version": env!("CARGO_PKG_VERSION") },
            "instructions": "Incremental TS/JS to Rust migration. Loop per unit: analyze -> bridge -> \
                record -> (port src/port.rs) -> build -> diff -> shadow -> gate. Never mark a port done \
                before gate passes. Read rust/RULES.md (`rules`) before porting; when the user corrects \
                behavior, add it with `rules`.",
        }),
        "ping" => json!({}),
        "tools/list" => json!({ "tools": tools() }),
        "tools/call" => {
            let name = request["params"]["name"].as_str().unwrap_or_default();
            let args = &request["params"]["arguments"];
            match call(name, args) {
                Ok(value) => json!({
                    "content": [{ "type": "text", "text": serde_json::to_string_pretty(&value).unwrap_or_default() }],
                    "structuredContent": value,
                    "isError": false,
                }),
                Err(e) => {
                    let text = format!("{e:#}");
                    json!({
                        "content": [{ "type": "text", "text": text }],
                        "structuredContent": failure(&text, args),
                        "isError": true,
                    })
                }
            }
        }
        "prompts/list" => crate::mcp_prompts::list(),
        "prompts/get" => {
            let params = &request["params"];
            match crate::mcp_prompts::get(
                params["name"].as_str().unwrap_or_default(),
                &params["arguments"],
            ) {
                Ok(prompt) => prompt,
                Err(message) => return Some(error(id, -32602, &message)),
            }
        }
        "skills/list" => fresh(json!({ "skills": [skill_entry()] })),
        "skills/get" => {
            let uri = request["params"]["uri"].as_str().unwrap_or_default();
            if uri != format!("{SKILL_ROOT}SKILL.md") {
                return Some(error(id, -32602, &format!("no skill at `{uri}`")));
            }
            fresh(json!({ "skill": skill_entry() }))
        }
        "resources/list" => {
            json!({ "resources": crate::setup::SKILL_FILES.iter().map(|(file, _)| json!({
            "uri": format!("{SKILL_ROOT}{file}"),
            "name": format!("rust-anything/{file}"),
            "mimeType": "text/markdown",
        })).collect::<Vec<_>>() })
        }
        "resources/read" => {
            let uri = request["params"]["uri"].as_str().unwrap_or_default();
            let Some((_, text)) = uri
                .strip_prefix(SKILL_ROOT)
                .and_then(|file| crate::setup::SKILL_FILES.iter().find(|(f, _)| *f == file))
            else {
                return Some(error(id, -32002, &format!("resource not found: `{uri}`")));
            };
            fresh(
                json!({ "contents": [{ "uri": uri, "mimeType": "text/markdown", "text": text }] }),
            )
        }
        _ => return Some(error(id, -32601, &format!("unknown method `{method}`"))),
    };
    Some(json!({ "jsonrpc": "2.0", "id": id, "result": result }))
}

/// Where the workflow skill's files live for the MCP skills extension.
const SKILL_ROOT: &str = "skill://rust-anything/";

/// A skills-extension result with the cache fields it requires.
fn fresh(mut result: Value) -> Value {
    result["resultType"] = json!("complete");
    result["ttlMs"] = json!(300_000);
    result["cacheScope"] = json!("public");
    result
}

fn skill_digest(text: &str) -> String {
    use sha2::{Digest, Sha256};
    let hash = Sha256::digest(text.as_bytes());
    let hex: String = hash.iter().map(|b| format!("{b:02x}")).collect();
    format!("sha256:{hex}")
}

/// The skill's `skills/list` entry: its frontmatter and every file with digest and size.
fn skill_entry() -> Value {
    let mut frontmatter = serde_json::Map::new();
    let header = crate::setup::SKILL
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---").map(|(h, _)| h))
        .unwrap_or_default();
    for line in header.lines() {
        if let Some((key, value)) = line.split_once(':') {
            frontmatter.insert(key.trim().into(), json!(value.trim()));
        }
    }
    json!({
        "uri": format!("{SKILL_ROOT}SKILL.md"),
        "frontmatter": frontmatter,
        "resources": crate::setup::SKILL_FILES.iter().map(|(file, text)| json!({
            "uri": format!("{SKILL_ROOT}{file}"),
            "digest": skill_digest(text),
            "size": text.len(),
        })).collect::<Vec<_>>(),
    })
}

fn error(id: Value, code: i64, message: &str) -> Value {
    json!({ "jsonrpc": "2.0", "id": id, "error": { "code": code, "message": message } })
}

/// One kind of tool failure: a stable code, whether the same call may succeed when retried,
/// and the next step, where `{path}` is the call's crate, repo or root.
struct FailureKind {
    code: &'static str,
    retryable: bool,
    matches: fn(&str) -> bool,
    next: Option<&'static str>,
}

/// Checked in order; the first match wins, `failed` otherwise.
const FAILURES: &[FailureKind] = &[
    // An argument of the wrong type (`opt_string`, `req_string`, `opt_u64`): checked first, since the
    // value it quotes may contain text another rule matches, such as "still running after".
    FailureKind {
        code: "invalid_arguments",
        retryable: false,
        matches: |t| t.starts_with('`') && t.contains("` must be ") && !t.contains('\n'),
        next: None,
    },
    FailureKind {
        code: "timed_out",
        retryable: true,
        matches: |t| t.contains("still running after"),
        next: Some(
            "retry; if it times out again, restart the MCP server with RUST_ANYTHING_COMMAND_TIMEOUT set higher",
        ),
    },
    FailureKind {
        code: "no_recording",
        retryable: false,
        matches: |t| {
            (t.contains("no recorded cases") || t.contains("no cases in"))
                && t.contains("app record` first")
        },
        next: Some("rust-anything app record {path} -- <command that runs the original>"),
    },
    FailureKind {
        code: "no_recording",
        retryable: false,
        matches: |t| t.contains("no recorded cases"),
        next: Some("rust-anything record {path}"),
    },
    FailureKind {
        code: "stale_recording",
        retryable: false,
        matches: |t| t.contains("changed since it was recorded"),
        next: Some("rust-anything record {path}"),
    },
    FailureKind {
        code: "stale_build",
        retryable: false,
        matches: |t| t.contains("is older than the crate's sources"),
        next: Some("rust-anything build {path}"),
    },
    FailureKind {
        code: "missing_tool",
        retryable: false,
        matches: |t| t.contains("` not found") && t.contains("`rust-anything doctor`"),
        next: Some("rust-anything doctor"),
    },
    FailureKind {
        code: "build_failed",
        retryable: false,
        matches: |t| t.contains("cargo build") && t.contains("failed"),
        next: Some("fix the compiler errors in the message, then rust-anything build {path}"),
    },
    FailureKind {
        code: "invalid_arguments",
        retryable: false,
        matches: |t| t.starts_with("missing `") && !t.contains('\n') && !t.contains(": "),
        next: None,
    },
    FailureKind {
        code: "unknown_tool",
        retryable: false,
        matches: |t| t.starts_with("unknown tool `"),
        next: Some("call tools/list for the tool names"),
    },
];

/// The structuredContent of a failed tool call.
fn failure(text: &str, args: &Value) -> Value {
    let kind = FAILURES.iter().find(|k| (k.matches)(text));
    let path = ["crate", "repo", "root"]
        .iter()
        .find_map(|key| args[key].as_str())
        .unwrap_or("<crate>");
    json!({
        "code": kind.map_or("failed", |k| k.code),
        "message": text,
        "retryable": kind.is_some_and(|k| k.retryable),
        "next": kind.and_then(|k| k.next).map(|n| n.replace("{path}", path)),
    })
}

/// How a tool behaves, for the MCP tool annotations.
struct ToolMeta {
    name: &'static str,
    title: &'static str,
    read_only: bool,
    destructive: bool,
    idempotent: bool,
    open_world: bool,
}

const fn meta(
    name: &'static str,
    title: &'static str,
    [read_only, destructive, idempotent, open_world]: [bool; 4],
) -> ToolMeta {
    ToolMeta {
        name,
        title,
        read_only,
        destructive,
        idempotent,
        open_world,
    }
}

/// `[readOnly, destructive, idempotent, openWorld]` per tool. Tools that run the original's
/// code, test commands or cargo are open-world; only app_lift rewrites files the user wrote.
const TOOL_META: &[ToolMeta] = &[
    meta("analyze", "Analyze repository", [true, false, true, false]),
    meta(
        "bridge",
        "Scaffold unit crate",
        [false, false, false, false],
    ),
    meta(
        "refresh",
        "Regenerate bindings",
        [false, false, true, false],
    ),
    meta(
        "record",
        "Record original behavior",
        [false, false, false, true],
    ),
    meta("build", "Build crate", [false, false, true, true]),
    meta(
        "diff",
        "Diff port against recording",
        [false, false, true, true],
    ),
    meta("profile", "Profile workload", [false, false, true, true]),
    meta(
        "go",
        "Port the hot path end to end",
        [false, false, false, true],
    ),
    meta("enable", "Switch a unit on", [false, false, true, false]),
    meta("disable", "Switch a unit off", [false, false, true, false]),
    meta("report", "Write a report card", [false, false, true, true]),
    meta("bench", "Benchmark port", [false, false, true, true]),
    meta(
        "fuzz",
        "Fuzz port against original",
        [false, false, false, true],
    ),
    meta(
        "shadow",
        "Generate shadow module",
        [false, false, false, true],
    ),
    meta(
        "gate",
        "Check definition of done",
        [false, false, true, true],
    ),
    meta("map", "Map the port", [true, false, true, false]),
    meta("rules", "Port rules", [false, false, true, false]),
    meta("package", "Lay out packages", [false, false, true, false]),
    meta(
        "app_record",
        "Record whole app",
        [false, false, false, true],
    ),
    meta("app_replay", "Replay app cases", [false, false, true, true]),
    meta(
        "app_front_door",
        "Scaffold Rust front door",
        [false, false, false, true],
    ),
    meta("app_wire", "Wire unit into app", [false, false, true, true]),
    meta(
        "app_lift",
        "Lift to pure Rust app",
        [false, true, false, true],
    ),
    meta("app_drift", "Check app drift", [false, false, true, true]),
];

fn tools() -> Value {
    let mut tools = tool_schemas();
    for tool in tools.as_array_mut().into_iter().flatten() {
        let name = tool["name"].as_str().unwrap_or_default();
        if let Some(m) = TOOL_META.iter().find(|m| m.name == name) {
            tool["title"] = json!(m.title);
            tool["annotations"] = json!({
                "title": m.title,
                "readOnlyHint": m.read_only,
                "destructiveHint": m.destructive,
                "idempotentHint": m.idempotent,
                "openWorldHint": m.open_world,
            });
        }
    }
    tools
}

fn tool_schemas() -> Value {
    let crate_dir = json!({ "type": "string", "description": "Folder of the generated crate (holds rust-anything.json)." });
    json!([
        {
            "name": "analyze",
            "description": "Map a repository (TypeScript/JavaScript, Python, C, Go, Ruby, Java, Kotlin, PHP, C# or PowerShell): units, port order (waves), risks with file:line, bridge per unit, dependency-to-crate fit, and `start_here`: the best first ports.",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Repository root." },
                "lang": { "type": "string", "enum": ["ts", "python", "c", "go", "ruby", "java", "kotlin", "php", "csharp", "powershell"], "description": "Source language; detected from the repository when omitted." },
                "whole_app": { "type": "boolean", "description": "Plan a whole-app port: entry points and shape (cli, server, library), every dependency imported or declared with its crate and fit, pieces in port order (units that import each other together), and numbered decisions with a recommended answer for the grilling round." },
            }, "required": ["root"] },
        },
        {
            "name": "bridge",
            "description": "Scaffold a Rust crate for one unit: napi-rs or wasm-bindgen bindings in src/lib.rs, stubs to port in src/port.rs, UTF-16 helpers when the unit uses string offsets. Never overwrites an existing folder. Read the repository's port rules (`rules`) before writing the port.",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Repository root." },
                "unit": { "type": "string", "description": "Unit id from analyze." },
                "kind": { "type": "string", "enum": ["napi", "wasm"], "description": "TS/JS units only; defaults to the plan's bridge. Other languages have their own bridge." },
                "out": { "type": "string", "description": "Crate folder; defaults to <root>/rust/<name>." },
                "name": { "type": "string", "description": "Crate name; defaults to `<unit>-rs`." },
                "force": { "type": "boolean", "description": "Bridge a unit the plan marks blocked or keep." },
                "keep": { "type": "array", "items": { "type": "string" }, "description": "Exports to keep in the source language: `name` or `name: reason`." },
                "batch": { "type": "array", "items": { "type": "string" }, "description": "Batch classes, `function:param`: the function runs over many `param` values kept in Rust, on all cores, in one call. A `<Function>Index` struct in port.rs replaces the default per-item loop." },
                "as_json": { "type": "array", "items": { "type": "string" }, "description": "Type parameters the app only fills with data (`T` in `paginate<T>(items: T[])`): bridged as JSON values instead of keeping the export in TS." },
                "allocator": { "type": "string", "enum": ["mimalloc", "rusty_alloc"], "description": "Global allocator for crates with batches. mimalloc (default) is C and leaves wasm alone; rusty_alloc is pure Rust (no C compiler) and also serves wasm." },
                "end_state": { "type": "string", "enum": ["replace", "bindings", "scaffold"], "description": "What the bindings are for once the port is done; ask the user before bridging. replace: the original is deleted after cut-over. bindings: callers keep importing it for good, so the bindings are public API. scaffold: the bindings are deleted at cut-over." },
            }, "required": ["root", "unit"] },
        },
        {
            "name": "refresh",
            "description": "Regenerate a crate's bindings and its language wrapper (the JS loader, the Python package or the generated wrapper file) from the signatures in src/port.rs, after changing them, adding any new batch classes. Never touches port.rs.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "batch": { "type": "array", "items": { "type": "string" }, "description": "Batch classes to add, `function:param`: the function runs over many `param` values kept in Rust, on all cores, in one call. A `<Function>Index` struct in port.rs replaces the default per-item loop." },
            }, "required": ["crate"] },
        },
        {
            "name": "record",
            "description": "Run the original functions on generated edge-case inputs (non-ASCII, emoji, -0, NaN, ...) plus optional extra inputs, and save the results as cases.jsonl: the behavior the port must match.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "count": { "type": "integer", "description": "Generated cases per function (default 200)." },
                "inputs": { "type": "string", "description": "JSONL file of extra cases: {\"fn\": name, \"args\": [...]}." },
                "module": { "type": "string", "description": "JS only: import the original from this JS module instead of the TS source." },
                "resolve_from": { "type": "string", "description": "Resolve npm packages from this folder's node_modules." },
                "float_tolerance": { "type": "number", "description": "Numeric code: floats match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9. Saved with the crate for diff and gate; 0 removes it." },
                "ignore_key_order": { "type": "boolean", "description": "Compare objects regardless of key order, now and in every later diff, gate, enable and report; false saves the opposite." },
                "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." },
                "test_command": { "type": "array", "items": { "type": "string" }, "description": "TS/JS or Python: also record every call the original's own test suite makes to the unit's functions, e.g. [\"npx\", \"jest\"] or [\"pytest\"]. Runs in the original's folder." },
                "profile": { "type": "string", "enum": ["host", "android"], "description": "Record on this machine (default) or on a connected Android device (JVM crates)." },
                "device": { "type": "string", "description": "Android device serial; picked automatically when only one is connected." },
                "pin_clock": { "type": "boolean", "description": "TS/JS (napi) and Python: true pins the clock and random numbers while recording and diffing (`Date.now()`, `new Date()`, `Math.random`; `time.time`, `datetime.now`, `random`) to the values in pinned.json, and writes src/port/pinned.rs for the port to read them (`mod pinned;` in port.rs). Kept for later runs; false stops pinning." },
            }, "required": ["crate"] },
        },
        {
            "name": "build",
            "description": "Build the crate: the napi-rs .node, wasm-bindgen output, Python extension or shared library. With `profile: \"android\"`, build the JNI library for Android ABIs into `jniLibs/` (needs the NDK and the Rust Android targets; returned as {ndk, api, libraries} instead of {artifact}).",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "profile": { "type": "string", "enum": ["host", "android"], "description": "Build for this machine (default) or for Android." },
                "abi": { "type": "array", "items": { "type": "string", "enum": ["arm64-v8a", "armeabi-v7a", "x86_64", "x86"] }, "description": "Android ABIs; defaults to arm64-v8a, armeabi-v7a and x86_64." },
                "api": { "type": "integer", "description": "Android API level (at least 26; default 26)." },
                "ndk": { "type": "string", "description": "Android NDK root; found from ANDROID_NDK_HOME or the SDK when omitted." },
                "jni_libs": { "type": "string", "description": "Where to install the Android libraries; defaults to <crate>/jniLibs." },
            }, "required": ["crate"] },
        },
        {
            "name": "diff",
            "description": "Replay cases.jsonl against the built Rust module and list mismatches. With `profile: \"android\"`, replay on the device against the .so built for its ABI.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "profile": { "type": "string", "enum": ["host", "android"], "description": "Diff on this machine (default) or on a connected Android device (JVM crates)." },
                "device": { "type": "string", "description": "Android device serial; picked automatically when only one is connected." },
                "jni_libs": { "type": "string", "description": "Android libraries folder to diff; defaults to <crate>/jniLibs." },
                "ignore_key_order": { "type": "boolean", "description": "Compare objects regardless of key order." },
                "module": { "type": "string", "description": "Test this JS module instead of the crate's `index.cjs`." },
                "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." },
                "float_tolerance": { "type": "number", "description": "Floats match within this tolerance for this run (the one saved by record otherwise)." },
                "redact": { "type": "boolean", "description": "Mismatches as shapes only: where they differ, types, lengths and hashes, never values or arguments; safe to paste into an issue or hand to another agent." },
            }, "required": ["crate"] },
        },
        {
            "name": "profile",
            "description": "Run a real workload (a command: node or python3 script, `go test -bench`, java, ruby) under the language's profiler and rank the repository's hot functions by inclusive time, with their callers, call counts and advice: what to port together, and which loops to batch (`bridge` with `batch`). Profile before porting: leaf functions called one at a time are slower in Rust.",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Repository root." },
                "command": { "type": "array", "items": { "type": "string" }, "description": "The workload, run in root: [\"node\", \"bench.mjs\"]." },
                "lang": { "type": "string", "enum": ["ts", "python", "go", "java", "ruby"], "description": "Detected from the repository when omitted." },
            }, "required": ["root", "command"] },
        },
        {
            "name": "go",
            "description": "The hot-path loop in one call: profile the workload, pick the top portable function (the outermost loop; batched where profile says so), bridge it into rust/<function>-rs, record generated cases plus the calls the workload makes, write TASK.md, hand the port to `agent` (claude, codex, or a command reading the prompt on stdin), then build, diff and gate, and switch the unit on with `enable` only when every case matches, gate passes and `bench` shows it is not slower as its callers call it. Without `agent` it stops at `waiting_for_port`: port src/port.rs as TASK.md says and call go again; finished steps are kept. Returns the stage (waiting_for_port, mismatch, gate_failed, slower, on, passed), the speedup, the pick, the steps and what to do next. TASK.md includes the repository's port rules (`rules`).",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Repository root." },
                "command": { "type": "array", "items": { "type": "string" }, "description": "The workload, run in root: [\"python3\", \"app.py\"]." },
                "agent": { "type": "string", "description": "claude, codex, or a command that reads a prompt on stdin. Omit to port it yourself." },
                "function": { "type": "string", "description": "Port this hot function instead of the profile's pick." },
                "lang": { "type": "string", "enum": ["ts", "python", "go", "java", "ruby"], "description": "Detected from the repository when omitted." },
                "count": { "type": "integer", "description": "Generated cases per function (default 200)." },
                "rounds": { "type": "integer", "description": "Agent turns: the port, then one per failing diff or gate (default 3)." },
            }, "required": ["root", "command"] },
        },
        {
            "name": "enable",
            "description": "Switch a ported TS/JS, Python or PowerShell unit on: one marked block in each original source file rebinds the unit's functions to the built Rust module, for every caller, with no import changed. RUST_ANYTHING_MODE=old or RUST_ANYTHING_UNIT_<NAME>=original runs the original for one run. Refuses while recorded cases differ unless `force`, `partial`, or `only` names functions that match.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "only": { "type": "array", "items": { "type": "string" }, "description": "Switch on only these exported functions." },
                "partial": { "type": "boolean", "description": "Switch on only the functions whose recorded cases all match; leave the rest in the original." },
                "force": { "type": "boolean", "description": "Switch it on even when recorded cases differ." },
            }, "required": ["crate"] },
        },
        {
            "name": "disable",
            "description": "Switch a unit off: removes exactly the lines enable added, leaving the source files byte for byte as they were.",
            "inputSchema": { "type": "object", "properties": { "crate": crate_dir }, "required": ["crate"] },
        },
        {
            "name": "report",
            "description": "Write <crate>/report/REPORT.md and card.svg for a pull request or README: the unit, recorded cases matched (diff), speedup call included (bench), fallback rate, on or off, and the machine with its core count. Builds first; every number comes from this run.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "out": { "type": "string", "description": "Output folder; defaults to `<crate>/report`." },
                "min_ms": { "type": "integer", "description": "Minimum measured time per function and side, in milliseconds (default 200)." },
                "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." },
                "resolve_from": { "type": "string", "description": "Resolve npm packages the original imports from this folder's node_modules." },
            }, "required": ["crate"] },
        },
        {
            "name": "bench",
            "description": "Time the recorded cases on the original and on the built Rust module, bridge included; speedup below 1 means the port is slower as its callers call it. Returns the rows and a verdict with advice: ship, keep the slower functions, or port a bigger unit.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "resolve_from": { "type": "string", "description": "Resolve npm packages the original imports from this folder's node_modules." },
                "min_ms": { "type": "integer", "description": "Minimum measured time per function and side, in milliseconds (default 200)." },
                "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." },
            }, "required": ["crate"] },
        },
        {
            "name": "fuzz",
            "description": "Live differential test: generate fresh inputs from a new seed (the record generator, without the cases already recorded), run them through the original and the built port, and report mismatches the recording missed. Returns the seed to replay, counts and the first mismatches; `keep` appends mismatching (or all) fresh cases to cases.jsonl so the next diff and gate check them. Works wherever record and diff do on this machine (TS/JS, Python, C, Go, Ruby, Java, Kotlin, PHP, C#, PowerShell).",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "cases": { "type": "integer", "description": "Fresh cases per function (and scenarios per class) in each round (default 200)." },
                "seed": { "type": "integer", "description": "Seed of the first round; random when omitted (returned as `seed`). Later rounds count up from it." },
                "time": { "type": "number", "description": "Keep running new rounds until this many seconds have passed; one round when omitted." },
                "keep": { "type": "string", "enum": ["mismatches", "all"], "description": "Append the mismatching fresh cases, or all of them, to cases.jsonl." },
                "resolve_from": { "type": "string", "description": "Resolve npm packages the original imports from this folder's node_modules." },
                "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." },
            }, "required": ["crate"] },
        },
        {
            "name": "shadow",
            "description": "Generate a drop-in shadow (a TS or Python module, Go package, Ruby file, Java classes or C file) that runs the original and the Rust port side by side and reports mismatches; RUST_ANYTHING_MODE=old|shadow|new switches at runtime.",
            "inputSchema": { "type": "object", "properties": { "crate": crate_dir, "out": { "type": "string", "description": "Where to write the module; defaults per target: `<crate>/shadow/` (TS, Python, C), `<crate>/ruby/` (Ruby), `<crate>/go/shadow/` (Go), `<crate>/java/` (Java, Kotlin)." } }, "required": ["crate"] },
        },
        {
            "name": "map",
            "description": "Port map of a repository: each unit of the original with the crate that ports it, its functions in Rust and kept in the original, its cases and last diff, the whole app's glue (Rust only, or still handing cases to the original), the units not ported yet in the order analyze suggests, and every port whose original changed since it was recorded (`changed_upstream`: re-record it and diff). Read-only; the CLI's `map --write` saves it as rust/PORT_MAP.md.",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "crates": { "type": "array", "items": { "type": "string" }, "description": "Folders with crates kept outside the repository." }
            }, "required": ["repo"] },
        },
        {
            "name": "rules",
            "description": "Show the port rules every agent prompt includes (rust/RULES.md, committed with the port), or add one: behavior the port must keep exactly, learned from a correction. Read them before porting; when the user corrects how the port behaves, add the correction as a rule so every later unit and agent keeps it. An exact duplicate is skipped.",
            "inputSchema": { "type": "object", "properties": {
                "root": { "type": "string", "description": "Repository root, or a crate folder made by `bridge`." },
                "add": { "type": "array", "items": { "type": "string" }, "description": "Rules to append." }
            }, "required": ["root"] },
        },
        {
            "name": "package",
            "description": "Lay out installable packages for a gated port; writes files only, never builds other platforms or publishes. PyO3: a maturin project (abi3 wheel importable under the original module's name). CLI front door (`rust/app` of a Node CLI): a main npm package whose command runs `<name>-<platform>`'s front door binary, with the original pinned to its exact version as a dependency for every case the port hands back. napi: a main npm package whose loader picks `<name>-<platform>` from optionalDependencies and falls back to the original JS, one package per platform (this machine's binary copied in when built). wasm: one npm package with the built pkg/. PyO3 and napi also get a GitHub Actions workflow building macOS (arm64, x86_64), Linux (x86_64, aarch64) and Windows.",
            "inputSchema": { "type": "object", "properties": {
                "crate": crate_dir,
                "name": { "type": "string", "description": "Package name on npm or PyPI; defaults to the crate name." },
                "version": { "type": "string", "description": "Package version; defaults to the original package's." },
                "out": { "type": "string", "description": "Output folder; defaults to <crate>/package." },
                "fallback": { "type": "string", "description": "napi: module to load where no binary fits; defaults to the original file when it is plain JS that imports nothing." },
                "bin": { "type": "array", "items": { "type": "string" }, "description": "CLI front door: the commands the package installs; defaults to the package name without its scope." },
                "original": { "type": "string", "description": "CLI front door: the original npm package it falls back to, as <name>@<version>; defaults to the one in the repository's node_modules." },
                "dry_run": { "type": "boolean", "description": "List the files, write nothing." },
            }, "required": ["crate"] },
        },
        {
            "name": "gate",
            "description": "Definition of done: fmt, clippy -D warnings, cargo test, unsafe with SAFETY comments only, no unimplemented functions, build, API present, recorded behavior matches, no recorded values hard-coded in the port, no tests or assertions removed since the last passing gate, and the recorded cases not edited.",
            "inputSchema": { "type": "object", "properties": { "crate": crate_dir, "node_args": { "type": "array", "items": { "type": "string" }, "description": "Extra arguments for node, e.g. [\"--import=tsx\"]." } }, "required": ["crate"] },
        },
        {
            "name": "app_record",
            "description": "Record a whole CLI or server's behavior (with optional HTTP, SMTP and database effects): run the original on its own documented examples, an inputs file and semantic traps (non-ASCII and emoji, negative division, overflow, float extremes, map order), twice each. What differs between runs is reported by kind in `noise`: values (now [..] wildcards), order (lines compared in any order) and lines (a ... that matches anything): show every one to the user. Cases are trycmd files in <repo>/rust/app-cases/ for CLIs (argv, stdin, env, files in; stdout, stderr, exit code, files out) or HTTP cases in <repo>/rust/app-cases/http/, with recorded effects alongside.",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "run": { "type": "array", "items": { "type": "string" }, "description": "Command that runs the original, paths relative to repo: [\"python3\", \"cli.py\"]. Optional after the first record." },
                "inputs": { "type": "string", "description": "JSONL of extra cases: {\"name\", \"args\": [...], \"stdin\", \"env\": {...}, \"files\": {path: text}}." },
                "normalize": { "type": "array", "items": { "type": "string" }, "description": "Normalizers: timestamps, uuids, key-order, or name=regex." },
                "name": { "type": "string", "description": "Name the app is invoked by in its docs." },
                "timeout": { "type": "integer", "description": "Seconds before a run is killed (default 30)." },
                "traps": { "type": "boolean", "description": "Add semantic-trap cases (default true)." },
                "http": { "type": "boolean", "description": "The app is a server: `run` starts it (it takes its port from PORT or {port}); cases are requests from curl lines in its docs, `inputs` ({\"name\", \"method\", \"path\", \"headers\", \"body\" | \"body_base64\" | \"body_file\"}) and traps. Pass `false` to drop a saved server recording and record as a CLI again." },
                "port": { "type": "integer", "description": "A port the server always listens on (implies http); 0 removes a saved fixed port." },
                "effects": { "type": "array", "items": { "type": "string" }, "description": "Record what leaves the process: \"http\" (a local sink set as HTTP_PROXY, http_proxy and RUST_ANYTHING_HTTP_SINK), \"smtp\" (a local mail server, SMTP_HOST and SMTP_PORT, recording decoded headers and MIME parts) and \"smtp-raw\" (raw MIME DATA). Each case stores what reached them, in order, and replay compares it." },
                "effect_env": { "type": "array", "items": { "type": "string" }, "description": "Point the app's own settings at a sink: VAR=http-url, VAR=smtp-host, VAR=smtp-port or VAR=smtp-url." },
                "effect_response": { "type": "array", "items": { "type": "string" }, "description": "Canned responses for the HTTP effect sink: \"METHOD /path=STATUS [BODY|@FILE]\" (e.g. \"POST /v1/charges=201 @fixtures/charge.json\", \"POST /hooks/*=500\"). Matched in order with `*` wildcards, falling back to `200 {}`. Enables the outbound HTTP effect (`effects: [\"http\"]`, not server mode); saved for replay." },
                "db_reset": { "type": "string", "description": "Shell command that resets the database before each case (each start for servers), run in the folder the case runs in; a script it names by a relative path with a script extension (.py, .sh, .js, .ts, .rb, .php, .pl, .ps1, .bat, .cmd, .sql and similar) is the repository's, while other relative paths, extensionless scripts included, are the case folder's. Cases then run one at a time." },
                "db_dump": { "type": "string", "description": "Shell command that prints the database's state (e.g. `sqlite3 app.db .dump`), run after each case or request in the folder the case runs in, and recorded with the effects." },
                "args": { "type": "array", "items": { "type": "string" }, "description": "Servers: arguments every start gets after its command, the original's and every build's ([\"db.json\"]). Non-empty implies http; saved. Pass `[]` to clear saved args." },
                "fresh": { "type": "array", "items": { "type": "string" }, "description": "Repository files or folders the app reads or writes (a project tree, a database file), snapshotted and copied into a fresh working directory before every case (CLIs) or start (servers); a CLI's removed files are compared too. Saved. Pass `[]` to clear saved paths." },
                "pace": { "type": "integer", "description": "Servers: milliseconds to wait after each response, for work the server does after answering (a file watcher). Implies http; saved and used by replay; 0 removes it." },
                "ready_timeout": { "type": "integer", "description": "Servers: seconds to wait for the port before giving up (default 60). Implies http; saved with the cases and used by replay and the front door; 0 restores the default." },
                "ready": { "type": "string", "description": "Servers: a request that says the app is ready once it answers 2xx, e.g. \"GET /health\", for a framework that opens its port before its routes work. Implies http; saved with the cases and used by replay and the front door." },
                "float_tolerance": { "type": "number", "description": "Numeric output: numbers match when they differ by at most this times the larger magnitude (absolute below 1), e.g. 1e-9. Saved with the cases; 0 removes it." },
                "float_file": { "type": "array", "items": { "type": "string" }, "description": "Written files of raw floats, compared value by value within float_tolerance: `PATH=f32le` or `PATH=f64le`. Saved with the cases." },
            }, "required": ["repo"] },
        },
        {
            "name": "app_replay",
            "description": "Replay <repo>/rust/app-cases/ against a build (the original, the front door, the final Rust binary): passed/total, fallback rate when the build reports it, and a report with a diff per failing case (rust/app-cases/replay-report.md).",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "build": { "type": "string", "description": "Command line of the build to test; the original when omitted." },
                "port": { "type": "integer", "description": "Servers: the port the build always listens on; otherwise it gets a free one through PORT and {port}." },
                "timeout": { "type": "integer", "description": "Seconds before a run is killed (default: the one saved by `app record`, 30 seconds unless it was changed)." },
                "case": { "type": "string", "description": "Replay only this case, by the name the report gives it (a library: only this unit): the fast loop for fixing one mismatch." },
                "redact": { "type": "boolean", "description": "Write the report and return mismatches as shapes only (no outputs, arguments or request paths)." },
            }, "required": ["repo"] },
        },
        {
            "name": "app_front_door",
            "description": "Scaffold <repo>/rust/app, the Rust entry point for a CLI, server or library (wiring recorded effects too): for a CLI, a clap parser for the recorded commands and flags, every command running the original with the same argv, stdin and env (each fallback counted); for a server, an axum app that starts the original on a port of its own and proxies every route no unit answers; for a library, a Rust crate re-exporting each wired unit's port. Builds it and replays the cases: 100% passed at 100% fallback. Needs app_record first; never overwrites rust/app.",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "check": { "type": "boolean", "description": "Build and replay (default true)." },
                "library": { "type": "boolean", "description": "Treat the repository as a library even if it has a main: the front door is a Rust crate with the public API, re-exporting each wired unit's port. Found by default when there is no main and no server." },
            }, "required": ["repo"] },
        },
        {
            "name": "app_wire",
            "description": "After a unit passes gate, hand its commands to its port.rs: adds the unit crate to rust/app as a library, writes a glue stub (src/units/<unit>.rs, yours to write: parse args, call <crate>::port, print what the original prints), regenerates the routing with a kill switch RUST_ANYTHING_UNIT_<NAME>=original, then builds and replays. Run it again after writing the glue: 100% passed and a lower fallback rate.",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "unit": { "type": "string", "description": "Unit crate folder, its name under rust/, or the unit id it was bridged from." },
                "commands": { "type": "array", "items": { "type": "string" }, "description": "Commands the unit answers; kept from the last wire when omitted." },
                "routes": { "type": "array", "items": { "type": "string" }, "description": "Servers: routes the unit answers, axum syntax: \"GET /users/{id}\", \"/path\" for any method, \"*\" for everything else; kept from the last wire when omitted." },
                "skip_gate": { "type": "boolean", "description": "Do not run gate on the unit first." },
                "check": { "type": "boolean", "description": "Build and replay (default true)." },
            }, "required": ["repo", "unit"] },
        },
        {
            "name": "app_lift",
            "description": "Make rust/app a pure Rust app. Refuses (and lists every blocker) unless replay passes with fallback rate 0 and every unit passes gate. Then, on a copy first: removes the fallback and kill switches, turns each unit into a plain library (no cdylib, bindings or generated wrappers), adds a trycmd test running rust/app-cases on cargo test, checks fmt/clippy/tests, DECISIONS.md targets and assets, and replays the pure binary. Never touches the original.",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
            }, "required": ["repo"] },
        },
        {
            "name": "app_drift",
            "description": "Check that the recording still describes the original and the port still matches: replays the original against rust/app-cases (cases it no longer matches changed upstream, listed in `changed`), lists examples in its docs that no case records (`new`), and replays the build (`failing`, each with a verdict: \"original changed, port still old\" or \"port regression\"). Writes rust/app-cases/drift-report.md and leaves replay-report.md as it was. For CLIs and servers; for units, `map` lists every port whose original changed (`changed_upstream`).",
            "inputSchema": { "type": "object", "properties": {
                "repo": { "type": "string", "description": "Repository root." },
                "build": { "type": "string", "description": "Build to check; defaults to rust/app, built with cargo, when it exists." },
                "timeout": { "type": "integer", "description": "Seconds before a run is killed (default: the one saved by `app record`, 30 seconds unless it was changed)." },
            }, "required": ["repo"] },
        },
    ])
}

/// `bridge`'s `kind`: `None` (the plan's choice) when omitted or null, an error for any other
/// value than `"napi"` or `"wasm"`, so a mistyped argument never bridges with a different kind.
fn kind_arg(args: &Value) -> Result<Option<bridge::Kind>> {
    Ok(match args.get("kind") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s == "napi" => Some(bridge::Kind::Napi),
        Some(Value::String(s)) if s == "wasm" => Some(bridge::Kind::Wasm),
        Some(other) => anyhow::bail!("`kind` is \"napi\" or \"wasm\", not {other}"),
    })
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Profile {
    Host,
    Android,
}

/// `record`/`build`/`diff`'s `profile`: `Host` when omitted or null, an error for any other
/// value than `"host"` or `"android"`.
fn profile_arg(args: &Value) -> Result<Profile> {
    Ok(match args.get("profile") {
        None | Some(Value::Null) => Profile::Host,
        Some(Value::String(s)) if s == "host" => Profile::Host,
        Some(Value::String(s)) if s == "android" => Profile::Android,
        Some(other) => anyhow::bail!("`profile` is \"host\" or \"android\", not {other}"),
    })
}

/// `fuzz`'s `keep`: `None` when omitted or null, an error for any other value than `"mismatches"`
/// or `"all"`.
fn keep_arg(args: &Value) -> Result<Option<crate::fuzz::Keep>> {
    Ok(match args.get("keep") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s == "mismatches" => Some(crate::fuzz::Keep::Mismatches),
        Some(Value::String(s)) if s == "all" => Some(crate::fuzz::Keep::All),
        Some(other) => anyhow::bail!("`keep` is \"mismatches\" or \"all\", not {other}"),
    })
}

/// `bridge`'s `allocator`: the default when omitted (or null), an error for any other value
/// than the two names, so a mistyped argument never bridges with an allocator nobody chose.
fn allocator_arg(args: &Value) -> Result<bridge::Allocator> {
    Ok(match args.get("allocator") {
        None | Some(Value::Null) => bridge::Allocator::default(),
        Some(Value::String(s)) if s == "mimalloc" => bridge::Allocator::Mimalloc,
        Some(Value::String(s)) if s == "rusty_alloc" => bridge::Allocator::RustyAlloc,
        Some(other) => anyhow::bail!("`allocator` is \"mimalloc\" or \"rusty_alloc\", not {other}"),
    })
}

/// `bridge`'s `end_state`: none when omitted (or null), an error for any other value than the
/// three names.
fn end_state_arg(args: &Value) -> Result<Option<bridge::EndState>> {
    Ok(match args.get("end_state") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) if s == "replace" => Some(bridge::EndState::Replace),
        Some(Value::String(s)) if s == "bindings" => Some(bridge::EndState::Bindings),
        Some(Value::String(s)) if s == "scaffold" => Some(bridge::EndState::Scaffold),
        Some(other) => {
            anyhow::bail!("`end_state` is \"replace\", \"bindings\" or \"scaffold\", not {other}")
        }
    })
}

fn opt_string_list(args: &Value, key: &str) -> Result<Option<Vec<String>>> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Array(items)) => items
            .iter()
            .map(|v| {
                v.as_str()
                    .map(ToString::to_string)
                    .ok_or_else(|| anyhow::anyhow!("{key} must be an array of strings"))
            })
            .collect::<Result<Vec<_>>>()
            .map(Some),
        Some(_) => anyhow::bail!("{key} must be an array of strings"),
    }
}

/// A string argument, refused when it is there and not a string: `as_str()` alone drops a number or
/// an object without a word, so the tool runs with the argument silently missing.
fn opt_string(args: &Value, key: &str) -> Result<Option<String>> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(text)) => Ok(Some(text.clone())),
        Some(v) => anyhow::bail!("`{key}` must be a string, not {v}"),
    }
}

/// A string argument the tool cannot run without: absent or `null` is "missing", any other type is
/// refused by name, so `"unit": 5` is not reported as if `unit` had not been sent.
fn req_string(args: &Value, key: &str) -> Result<String> {
    opt_string(args, key)?.with_context(|| format!("missing `{key}`"))
}

fn opt_u64(args: &Value, key: &str) -> Result<Option<u64>> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => v
            .as_u64()
            .map(Some)
            .with_context(|| format!("`{key}` must be a non-negative integer, not {v}")),
    }
}

fn opt_usize(args: &Value, key: &str) -> Result<Option<usize>> {
    match opt_u64(args, key)? {
        None => Ok(None),
        Some(n) => Ok(Some(
            usize::try_from(n).with_context(|| format!("`{key}` is too large"))?,
        )),
    }
}

fn opt_u16(args: &Value, key: &str) -> Result<Option<u16>> {
    match opt_u64(args, key)? {
        None => Ok(None),
        Some(n) => {
            Ok(Some(u16::try_from(n).with_context(|| {
                format!("`{key}` must be at most {}", u16::MAX)
            })?))
        }
    }
}

fn opt_f64(args: &Value, key: &str) -> Result<Option<f64>> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => v
            .as_f64()
            .map(Some)
            .with_context(|| format!("`{key}` must be a number, not {v}")),
    }
}

fn opt_bool(args: &Value, key: &str) -> Result<Option<bool>> {
    match args.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(v) => v
            .as_bool()
            .map(Some)
            .with_context(|| format!("`{key}` must be a boolean, not {v}")),
    }
}

fn call(name: &str, args: &Value) -> Result<Value> {
    let path = |key: &str| -> Result<PathBuf> {
        opt_string(args, key)?
            .map(PathBuf::from)
            .with_context(|| format!("missing `{key}`"))
    };
    let opt_path =
        |key: &str| -> Result<Option<PathBuf>> { Ok(opt_string(args, key)?.map(PathBuf::from)) };
    let strings = |key: &str| -> Vec<String> {
        args[key]
            .as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(str::to_string))
                    .collect()
            })
            .unwrap_or_default()
    };
    let opt_timeout = || -> Result<Option<u64>> {
        match args.get("timeout") {
            None | Some(Value::Null) => Ok(None),
            Some(v) => {
                let secs = v
                    .as_u64()
                    .ok_or_else(|| anyhow::anyhow!("--timeout must be at least 1 second"))?;
                app::check_timeout(Some(secs))?;
                Ok(Some(secs))
            }
        }
    };
    Ok(match name {
        "analyze" => {
            let root = path("root")?;
            let lang = match args.get("lang") {
                Some(v) if !v.is_null() => {
                    Some(serde_json::from_value(v.clone()).context("`lang`")?)
                }
                _ => None,
            };
            let whole_app = opt_bool(args, "whole_app")?.unwrap_or(false);
            let mut report = analyze::analyze_opts(&root, lang)?;
            if whole_app {
                report.whole_app = Some(crate::whole_app::whole_app(&root, &report));
            }
            serde_json::to_value(report)?
        }
        "bridge" => {
            let kind = kind_arg(args)?;
            let root = path("root")?;
            let allocator = allocator_arg(args)?;
            let end_state = end_state_arg(args)?;
            let force = opt_bool(args, "force")?.unwrap_or(false);
            let outcome = bridge::bridge(&bridge::Options {
                root: &root,
                unit: &req_string(args, "unit")?,
                kind,
                out: opt_path("out")?,
                name: opt_string(args, "name")?,
                force,
                keep: strings("keep"),
                lang: None,
                batch: strings("batch"),
                as_json: strings("as_json"),
                allocator,
            })?;
            if let Some(end_state) = end_state {
                bridge::set_end_state(&outcome.crate_dir, end_state)?;
            }
            serde_json::to_value(outcome)?
        }
        "record" => {
            let crate_dir = path("crate")?;
            let profile = profile_arg(args)?;
            if profile == Profile::Host && !args["device"].is_null() {
                anyhow::bail!("`device` needs profile `android`");
            }
            let float_tolerance = opt_f64(args, "float_tolerance")?
                .map(crate::tolerance::check)
                .transpose()?;
            let ignore_key_order = opt_bool(args, "ignore_key_order")?;
            let pin_clock = opt_bool(args, "pin_clock")?;
            let count = opt_usize(args, "count")?.unwrap_or(200);
            let mut inputs = opt_path("inputs")?;
            // Read before the test capture below, which runs the test command and writes files.
            let module = opt_path("module")?;
            let resolve_from = opt_path("resolve_from")?;
            let test_command = strings("test_command");
            let captured = if test_command.is_empty() {
                None
            } else {
                let c = crate::from_tests::capture(&crate_dir, &test_command)?;
                inputs = Some(crate::from_tests::with_own_inputs(
                    &crate_dir,
                    inputs.as_deref(),
                    &c.inputs,
                )?);
                Some(serde_json::json!({
                    "per_function": c.per_function,
                    "not_captured": c.skipped,
                    "tests_passed": c.tests_passed,
                }))
            };
            if let Some(t) = float_tolerance {
                let mut manifest = bridge::Manifest::load(&crate_dir)?;
                manifest.float_tolerance = (t > 0.0).then_some(t);
                manifest.save(&crate_dir)?;
            }
            if let Some(on) = ignore_key_order {
                let mut manifest = bridge::Manifest::load(&crate_dir)?;
                manifest.ignore_key_order = on;
                manifest.save(&crate_dir)?;
            }
            let helper = match pin_clock {
                Some(on) => crate::pin::set(&crate_dir, on)?,
                None => None,
            };
            let opts = harness::RecordOptions {
                crate_dir: &crate_dir,
                count,
                inputs: inputs.as_deref(),
                module: module.as_deref(),
                resolve_from: resolve_from.as_deref(),
                node_args: &strings("node_args"),
            };
            let device = args["device"].as_str();
            let mut out = serde_json::to_value(match profile {
                Profile::Host => harness::record(&opts)?,
                Profile::Android => harness::record_android(&opts, device)?,
            })?;
            if let Some(c) = captured {
                out["from_tests"] = c;
            }
            if let Some(helper) = helper {
                out["pinned_helper"] = json!(helper);
            }
            out
        }
        "refresh" => {
            let batch = strings("batch");
            let crate_dir = path("crate")?;
            serde_json::to_value(if batch.is_empty() {
                bridge::refresh(&crate_dir)?
            } else {
                bridge::add_batches(&crate_dir, &batch)?
            })?
        }
        "profile" => {
            let command = strings("command");
            anyhow::ensure!(!command.is_empty(), "missing `command`");
            let lang = match args.get("lang") {
                Some(v) if !v.is_null() => {
                    Some(serde_json::from_value(v.clone()).context("`lang`")?)
                }
                _ => None,
            };
            serde_json::to_value(crate::profile::profile(&path("root")?, &command, lang)?)?
        }
        "build" => {
            let crate_dir = path("crate")?;
            match profile_arg(args)? {
                Profile::Android => {
                    if !args["abi"].is_null() && !args["abi"].is_array() {
                        anyhow::bail!("`abi` must be an array of Android ABI names");
                    }
                    let api = if args["api"].is_null() {
                        crate::android::DEFAULT_API
                    } else {
                        let v = args["api"]
                            .as_u64()
                            .context("`api` must be a non-negative integer")?;
                        u32::try_from(v).context("`api` is too large")?
                    };
                    serde_json::to_value(harness::build_android(
                        &crate_dir,
                        &crate::android::Options {
                            abis: strings("abi"),
                            api,
                            ndk: opt_path("ndk")?,
                            out: opt_path("jni_libs")?,
                        },
                    )?)?
                }
                Profile::Host => {
                    if !args["abi"].is_null()
                        || !args["api"].is_null()
                        || !args["ndk"].is_null()
                        || !args["jni_libs"].is_null()
                    {
                        anyhow::bail!("`abi`, `api`, `ndk` and `jni_libs` need profile `android`");
                    }
                    json!({ "artifact": harness::build(&crate_dir)? })
                }
            }
        }
        "diff" => {
            let crate_dir = path("crate")?;
            let module = opt_path("module")?;
            let opts = harness::DiffOptions {
                crate_dir: &crate_dir,
                module: module.as_deref(),
                ignore_key_order: opt_bool(args, "ignore_key_order")?.unwrap_or(false),
                node_args: &strings("node_args"),
                float_tolerance: opt_f64(args, "float_tolerance")?,
            };
            let redact = opt_bool(args, "redact")?.unwrap_or(false);
            let mut d = match profile_arg(args)? {
                Profile::Host => {
                    if !args["device"].is_null() || !args["jni_libs"].is_null() {
                        anyhow::bail!("`device` and `jni_libs` need profile `android`");
                    }
                    harness::diff(&opts)?
                }
                Profile::Android => harness::diff_android(
                    &opts,
                    args["device"].as_str(),
                    args["jni_libs"].as_str().map(Path::new),
                )?,
            };
            if redact {
                crate::mismatch::redact_diff(&mut d);
            }
            serde_json::to_value(d)?
        }
        "bench" => {
            let crate_dir = path("crate")?;
            let resolve_from = opt_path("resolve_from")?;
            let rows = harness::bench(&harness::BenchOptions {
                crate_dir: &crate_dir,
                min_ms: opt_u64(args, "min_ms")?.unwrap_or(200),
                resolve_from: resolve_from.as_deref(),
                node_args: &strings("node_args"),
            })?;
            let lang = bridge::Manifest::load(&crate_dir)
                .map(|m| m.language)
                .unwrap_or_default();
            let verdict = harness::verdict(&rows, lang);
            serde_json::json!({ "rows": rows, "verdict": verdict })
        }
        "fuzz" => {
            let crate_dir = path("crate")?;
            let resolve_from = opt_path("resolve_from")?;
            let keep = keep_arg(args)?;
            let time = match opt_f64(args, "time")? {
                Some(0.0) => Some(std::time::Duration::ZERO),
                Some(t) if t.is_finite() && t > 0.0 => {
                    Some(crate::process::seconds(t).context("`time` is too large")?)
                }
                Some(t) => {
                    anyhow::bail!("`time` must be 0 or more seconds (0 runs one round), not {t}")
                }
                None => None,
            };
            serde_json::to_value(crate::fuzz::fuzz(&crate::fuzz::FuzzOptions {
                crate_dir: &crate_dir,
                cases: opt_usize(args, "cases")?.unwrap_or(200),
                seed: opt_u64(args, "seed")?,
                time,
                keep,
                resolve_from: resolve_from.as_deref(),
                node_args: &strings("node_args"),
            })?)?
        }
        "shadow" => {
            let out = opt_path("out")?;
            serde_json::to_value(shadow::shadow(&path("crate")?, out.as_deref())?)?
        }
        "map" => {
            let crates: Vec<std::path::PathBuf> =
                strings("crates").into_iter().map(Into::into).collect();
            serde_json::to_value(crate::port_map::map(&path("repo")?, &crates)?)?
        }
        "rules" => serde_json::to_value(crate::rules::add(&path("root")?, &strings("add"))?)?,
        "package" => {
            let crate_dir = path("crate")?;
            serde_json::to_value(crate::package::package(&crate::package::Options {
                crate_dir: &crate_dir,
                name: opt_string(args, "name")?,
                version: opt_string(args, "version")?,
                out: opt_path("out")?,
                dry_run: opt_bool(args, "dry_run")?.unwrap_or(false),
                fallback: opt_string(args, "fallback")?,
                bin: strings("bin"),
                original: opt_string(args, "original")?,
            })?)?
        }
        "go" => {
            let command = strings("command");
            anyhow::ensure!(!command.is_empty(), "missing `command`");
            let lang = match args.get("lang") {
                Some(v) if !v.is_null() => {
                    Some(serde_json::from_value(v.clone()).context("`lang`")?)
                }
                _ => None,
            };
            let root = path("root")?;
            let count = opt_usize(args, "count")?.unwrap_or(200);
            let rounds = opt_usize(args, "rounds")?;
            serde_json::to_value(crate::go::go(&crate::go::Options {
                root: &root,
                command: &command,
                lang,
                agent: args["agent"].as_str().filter(|a| !a.trim().is_empty()),
                function: args["function"].as_str(),
                count,
                rounds,
            })?)?
        }
        "enable" => {
            let crate_dir = std::path::absolute(path("crate")?)?;
            let only = strings("only");
            let partial = opt_bool(args, "partial")?.unwrap_or(false);
            let force = opt_bool(args, "force")?.unwrap_or(false);
            anyhow::ensure!(
                !(partial && (!only.is_empty() || force)),
                "`partial` cannot be combined with `only` or `force`"
            );
            let diff_opts = harness::DiffOptions {
                crate_dir: &crate_dir,
                module: None,
                ignore_key_order: false,
                node_args: &[],
                float_tolerance: None,
            };
            let on = if partial || (!only.is_empty() && !force) {
                harness::build(&crate_dir)?;
                let (d, counts) = harness::diff_full_with_counts(&diff_opts)?;
                let manifest = bridge::Manifest::load(&crate_dir)?;
                let (selected, skipped) =
                    crate::enable::select_matching(&manifest, &d, &counts, &only)?;
                crate::enable::enable_only(&crate_dir, Some(&selected), &skipped)?
            } else if !only.is_empty() {
                crate::enable::enable_only(&crate_dir, Some(&only), &[])?
            } else {
                if !force {
                    harness::build(&crate_dir)?;
                    let d = harness::diff(&diff_opts)?;
                    anyhow::ensure!(
                        d.failed == 0 && d.missing.is_empty(),
                        "{}/{} recorded cases match; switch it on once they all do, or pass partial, only, or force",
                        d.passed,
                        d.total
                    );
                }
                crate::enable::enable(&crate_dir)?
            };
            serde_json::to_value(on)?
        }
        "disable" => serde_json::to_value(crate::enable::disable(&std::path::absolute(path(
            "crate",
        )?)?)?)?,
        "report" => {
            let crate_dir = std::path::absolute(path("crate")?)?;
            let out = opt_path("out")?.map(std::path::absolute).transpose()?;
            serde_json::to_value(crate::card::report(&crate::card::Options {
                crate_dir: &crate_dir,
                out: out.as_deref(),
                min_ms: opt_u64(args, "min_ms")?.unwrap_or(200),
                node_args: &strings("node_args"),
                resolve_from: opt_path("resolve_from")?.as_deref(),
            })?)?
        }
        "gate" => serde_json::to_value(gate::gate(
            Path::new(&path("crate")?),
            &strings("node_args"),
        )?)?,
        "app_record" => {
            let repo = path("repo")?;
            let timeout_secs = opt_timeout()?;
            let inputs = opt_path("inputs")?;
            let effect_response = match args.get("effect_response") {
                None | Some(Value::Null) => Vec::new(),
                Some(Value::Array(items)) => items
                    .iter()
                    .map(|v| {
                        v.as_str().map(ToString::to_string).ok_or_else(|| {
                            anyhow::anyhow!("effect_response must be an array of strings")
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
                Some(_) => anyhow::bail!("effect_response must be an array of strings"),
            };
            let db_reset = opt_string(args, "db_reset")?;
            let db_dump = opt_string(args, "db_dump")?;
            let name = opt_string(args, "name")?;
            let ready = opt_string(args, "ready")?;
            let effects = crate::effects::EffectsConfig::from_args_with_responses(
                &strings("effects"),
                &strings("effect_env"),
                &effect_response,
                &repo,
                db_reset.as_deref(),
                db_dump.as_deref(),
            )?;
            let server_args = opt_string_list(args, "args")?;
            let fresh = opt_string_list(args, "fresh")?;
            serde_json::to_value(app::record(&app::RecordOptions {
                repo: &repo,
                run: &strings("run"),
                inputs: inputs.as_deref(),
                normalize: &strings("normalize"),
                name: name.as_deref(),
                traps: opt_bool(args, "traps")?.unwrap_or(true),
                timeout_secs,
                http: opt_bool(args, "http")?,
                port: opt_u16(args, "port")?,
                server_args: server_args.as_deref(),
                fresh: fresh.as_deref(),
                pace_ms: opt_u64(args, "pace")?,
                float_tolerance: opt_f64(args, "float_tolerance")?,
                float_files: &strings("float_file"),
                ready_secs: opt_u64(args, "ready_timeout")?,
                ready_request: ready.as_deref(),
                effects,
            })?)?
        }
        "app_replay" => {
            let repo = path("repo")?;
            let timeout_secs = opt_timeout()?;
            let redact = opt_bool(args, "redact")?.unwrap_or(false);
            let build = opt_string(args, "build")?;
            let case = opt_string(args, "case")?;
            let opts = app::ReplayOptions {
                repo: &repo,
                build: build.as_deref(),
                timeout_secs,
                port: opt_u16(args, "port")?,
                case: case.as_deref(),
            };
            serde_json::to_value(if redact {
                app::replay_redacted(&opts)?
            } else {
                app::replay(&opts)?
            })?
        }
        "app_front_door" => {
            let repo = path("repo")?;
            serde_json::to_value(front_door::front_door(&front_door::FrontDoorOptions {
                repo: &repo,
                library: opt_bool(args, "library")?.unwrap_or(false),
                check: opt_bool(args, "check")?.unwrap_or(true),
            })?)?
        }
        "app_wire" => {
            let repo = path("repo")?;
            serde_json::to_value(front_door::wire(&front_door::WireOptions {
                repo: &repo,
                unit: &req_string(args, "unit")?,
                commands: &strings("commands"),
                routes: &strings("routes"),
                skip_gate: opt_bool(args, "skip_gate")?.unwrap_or(false),
                check: opt_bool(args, "check")?.unwrap_or(true),
            })?)?
        }
        "app_lift" => serde_json::to_value(lift::lift(&lift::LiftOptions {
            repo: &path("repo")?,
        })?)?,
        "app_drift" => {
            let repo = path("repo")?;
            let timeout_secs = opt_timeout()?;
            let build = opt_string(args, "build")?;
            serde_json::to_value(crate::drift::drift(&crate::drift::DriftOptions {
                repo: &repo,
                build: build.as_deref(),
                timeout_secs,
            })?)?
        }
        #[cfg(test)]
        "test_sleep" => {
            std::thread::sleep(std::time::Duration::from_millis(
                args["ms"].as_u64().unwrap_or(0),
            ));
            json!({ "slept": true })
        }
        _ => anyhow::bail!("unknown tool `{name}`"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_string_argument_is_none_when_it_is_absent_or_null() {
        // Issue #671: an absent or `null` argument means "not given", so the tool keeps the saved value
        // instead of failing; only a value of another type is refused.
        assert_eq!(opt_string(&json!({}), "ready").unwrap(), None);
        assert_eq!(
            opt_string(&json!({ "ready": null }), "ready").unwrap(),
            None
        );
        assert_eq!(
            opt_string(&json!({ "ready": "GET /health" }), "ready").unwrap(),
            Some("GET /health".to_string())
        );
        let err = opt_string(&json!({ "ready": 42 }), "ready")
            .unwrap_err()
            .to_string();
        assert_eq!(err, "`ready` must be a string, not 42");
    }

    #[test]
    fn an_optional_path_that_is_not_a_string_is_refused_not_dropped() {
        // Issue #686: `"out": 5` was dropped, so `package` wrote to the default `package/` directory
        // and the caller never learned `out` had been ignored.
        let err = call(
            "package",
            &json!({ "crate": "rust/math-rs", "dry_run": true, "out": 5 }),
        )
        .unwrap_err()
        .to_string();
        assert_eq!(err, "`out` must be a string, not 5");
        // `record` refuses a bad `module` before its test command runs: this one could never start.
        let err = call(
            "record",
            &json!({ "crate": "rust/math-rs", "test_command": ["/nonexistent/test-runner"], "module": 5 }),
        )
        .unwrap_err()
        .to_string();
        assert_eq!(err, "`module` must be a string, not 5");
    }

    #[test]
    fn a_required_path_that_is_not_a_string_is_named_not_reported_missing() {
        // Issue #685: `"root": 5` was reported as `missing `root``, which sends the caller looking
        // for a typo in the key instead of the value.
        let err = |args: Value| call("analyze", &args).unwrap_err().to_string();
        assert_eq!(err(json!({ "root": 5 })), "`root` must be a string, not 5");
        assert_eq!(err(json!({})), "missing `root`");
        assert_eq!(err(json!({ "root": null })), "missing `root`");
        let err = |tool: &str, args: Value| call(tool, &args).unwrap_err().to_string();
        assert_eq!(
            err("app_lift", json!({ "repo": true })),
            "`repo` must be a string, not true"
        );
        assert_eq!(
            err("package", json!({ "crate": ["x"] })),
            "`crate` must be a string, not [\"x\"]"
        );
    }

    #[test]
    fn a_wrong_type_argument_is_classified_as_invalid_arguments_through_handle() {
        // The type error keeps the machine-readable code a missing argument has, and a quoted value that
        // reads like a timeout is not reported as retryable.
        for arguments in [
            json!({ "root": ".", "unit": 5, "kind": "napi" }),
            json!({ "root": ".", "unit": { "x": "still running after 5s" }, "kind": "napi" }),
        ] {
            let response = handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
                "params": { "name": "bridge", "arguments": arguments } }))
            .unwrap();
            let s = &response["result"]["structuredContent"];
            assert_eq!(s["code"], "invalid_arguments", "{s}");
            assert_eq!(s["retryable"], false, "{s}");
        }
    }

    #[test]
    fn bridge_and_app_wire_refuse_a_unit_or_name_that_is_not_a_string() {
        // Issue #684: `"unit": 5` used to read as a missing unit, and `"name": 7` was dropped, so the
        // bridge was written under the default name. Both are refused before anything is written.
        let dir =
            std::env::temp_dir().join(format!("rust-anything-mcp-684-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let root = dir.to_str().unwrap();
        let err = |tool: &str, args: Value| call(tool, &args).unwrap_err().to_string();
        assert_eq!(
            err("bridge", json!({ "root": root, "unit": 5, "kind": "napi" })),
            "`unit` must be a string, not 5"
        );
        assert_eq!(
            err("app_wire", json!({ "repo": root, "unit": 5 })),
            "`unit` must be a string, not 5"
        );
        assert_eq!(
            err(
                "bridge",
                json!({ "root": root, "unit": "src/math.ts", "kind": "napi", "name": 7 })
            ),
            "`name` must be a string, not 7"
        );
        assert_eq!(
            err("bridge", json!({ "root": root, "kind": "napi" })),
            "missing `unit`"
        );
        assert_eq!(
            err("app_wire", json!({ "repo": root, "unit": null })),
            "missing `unit`"
        );
        let written = std::fs::read_dir(&dir).unwrap().count();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(written, 0);
    }

    #[test]
    fn package_refuses_a_string_argument_that_is_not_a_string() {
        // Issue #683: `"version": 2` was dropped, so the package was built with the crate's own
        // version and the caller never learned `version` had been ignored.
        let err = |key: &str| {
            let mut args = json!({ "crate": "rust/math-rs", "dry_run": true });
            args[key] = json!(2);
            call("package", &args).unwrap_err().to_string()
        };
        for key in ["name", "version", "fallback", "original"] {
            assert_eq!(err(key), format!("`{key}` must be a string, not 2"));
        }
    }

    #[test]
    fn app_replay_and_app_drift_refuse_a_build_or_case_that_is_not_a_string() {
        // Issue #682: `"case": 5` was dropped, so every case was replayed, and `"build": 5` replayed
        // the original instead of a build. Both are refused before anything runs.
        let err = |tool: &str, args: Value| call(tool, &args).unwrap_err().to_string();
        assert_eq!(
            err("app_replay", json!({ "repo": ".", "case": 5 })),
            "`case` must be a string, not 5"
        );
        assert_eq!(
            err("app_replay", json!({ "repo": ".", "build": 5 })),
            "`build` must be a string, not 5"
        );
        assert_eq!(
            err("app_drift", json!({ "repo": ".", "build": 5 })),
            "`build` must be a string, not 5"
        );
    }

    #[test]
    fn a_fuzz_time_too_large_for_a_duration_is_an_error_not_a_panic() {
        // The conversion used to be `Duration::from_secs_f64`, which panics above ~1.8e19 s.
        let err = call("fuzz", &json!({ "crate": ".", "time": 1e30 }))
            .unwrap_err()
            .to_string();
        assert!(err.contains("too large"), "{err}");
    }

    #[test]
    fn go_refuses_zero_rounds_and_rounds_without_agent() {
        let err0 = call(
            "go",
            &json!({ "root": ".", "command": ["python3", "app.py"], "rounds": 0 }),
        )
        .unwrap_err()
        .to_string();
        assert!(err0.contains("`--rounds` must be at least 1"), "{err0}");

        let err_no_agent = call(
            "go",
            &json!({ "root": ".", "command": ["python3", "app.py"], "rounds": 2 }),
        )
        .unwrap_err()
        .to_string();
        assert!(
            err_no_agent.contains("rounds are agent turns"),
            "{err_no_agent}"
        );
    }

    #[test]
    fn a_negative_fuzz_time_names_the_range_and_the_value() {
        let err = call("fuzz", &json!({ "crate": ".", "time": -1 }))
            .unwrap_err()
            .to_string();
        assert_eq!(
            err,
            "`time` must be 0 or more seconds (0 runs one round), not -1"
        );
    }

    #[test]
    fn end_state_is_one_of_three_names_or_none() {
        let pick = |v: Value| end_state_arg(&v);
        assert_eq!(pick(json!({})).unwrap(), None);
        assert_eq!(pick(json!({ "end_state": null })).unwrap(), None);
        assert_eq!(
            pick(json!({ "end_state": "replace" })).unwrap(),
            Some(bridge::EndState::Replace)
        );
        assert_eq!(
            pick(json!({ "end_state": "bindings" })).unwrap(),
            Some(bridge::EndState::Bindings)
        );
        assert_eq!(
            pick(json!({ "end_state": "scaffold" })).unwrap(),
            Some(bridge::EndState::Scaffold)
        );
        assert!(pick(json!({ "end_state": "delete" })).is_err());
    }

    #[test]
    fn allocator_defaults_only_when_omitted() {
        let pick = |v: Value| allocator_arg(&v);
        assert_eq!(pick(json!({})).unwrap(), bridge::Allocator::Mimalloc);
        assert_eq!(
            pick(json!({ "allocator": null })).unwrap(),
            bridge::Allocator::Mimalloc
        );
        assert_eq!(
            pick(json!({ "allocator": "mimalloc" })).unwrap(),
            bridge::Allocator::Mimalloc
        );
        assert_eq!(
            pick(json!({ "allocator": "rusty_alloc" })).unwrap(),
            bridge::Allocator::RustyAlloc
        );
        for bad in [
            json!(false),
            json!(42),
            json!("jemalloc"),
            json!(["rusty_alloc"]),
        ] {
            let err = pick(json!({ "allocator": bad.clone() })).unwrap_err();
            assert!(
                format!("{err}").contains("\"mimalloc\" or \"rusty_alloc\""),
                "{bad}: {err}"
            );
        }
    }

    #[test]
    fn bridge_refuses_a_bad_allocator_before_writing_anything() {
        let out =
            std::env::temp_dir().join(format!("rust-anything-mcp-alloc-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&out);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini");
        let err = call(
            "bridge",
            &json!({ "root": root, "unit": "packages/a/src/util.ts", "out": out, "allocator": 42 }),
        )
        .unwrap_err();
        assert!(format!("{err}").contains("`allocator`"), "{err}");
        assert!(!out.exists(), "a refused bridge must not create the crate");
    }

    #[test]
    fn kind_profile_and_keep_reject_unknown_or_non_string_values() {
        assert_eq!(kind_arg(&json!({})).unwrap(), None);
        assert_eq!(kind_arg(&json!({ "kind": null })).unwrap(), None);
        assert_eq!(
            kind_arg(&json!({ "kind": "napi" })).unwrap(),
            Some(bridge::Kind::Napi)
        );
        assert_eq!(
            kind_arg(&json!({ "kind": "wasm" })).unwrap(),
            Some(bridge::Kind::Wasm)
        );
        for bad in [json!("pyo3"), json!("nap"), json!(42), json!(true)] {
            let err = kind_arg(&json!({ "kind": bad.clone() })).unwrap_err();
            assert!(
                format!("{err}").contains("\"napi\" or \"wasm\""),
                "{bad}: {err}"
            );
        }

        assert_eq!(profile_arg(&json!({})).unwrap(), Profile::Host);
        assert_eq!(
            profile_arg(&json!({ "profile": null })).unwrap(),
            Profile::Host
        );
        assert_eq!(
            profile_arg(&json!({ "profile": "host" })).unwrap(),
            Profile::Host
        );
        assert_eq!(
            profile_arg(&json!({ "profile": "android" })).unwrap(),
            Profile::Android
        );
        for bad in [json!("ios"), json!(123), json!(false)] {
            let err = profile_arg(&json!({ "profile": bad.clone() })).unwrap_err();
            assert!(
                format!("{err}").contains("\"host\" or \"android\""),
                "{bad}: {err}"
            );
        }

        assert_eq!(keep_arg(&json!({})).unwrap(), None);
        assert_eq!(keep_arg(&json!({ "keep": null })).unwrap(), None);
        assert_eq!(
            keep_arg(&json!({ "keep": "mismatches" })).unwrap(),
            Some(crate::fuzz::Keep::Mismatches)
        );
        assert_eq!(
            keep_arg(&json!({ "keep": "all" })).unwrap(),
            Some(crate::fuzz::Keep::All)
        );
        for bad in [json!("some"), json!(42), json!(true)] {
            let err = keep_arg(&json!({ "keep": bad.clone() })).unwrap_err();
            assert!(
                format!("{err}").contains("\"mismatches\" or \"all\""),
                "{bad}: {err}"
            );
        }
    }

    #[test]
    fn bridge_refuses_a_bad_kind_before_writing_anything() {
        let out =
            std::env::temp_dir().join(format!("rust-anything-mcp-kind-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&out);
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/mini");
        for bad in [json!("nap"), json!(42)] {
            let err = call(
                "bridge",
                &json!({ "root": root, "unit": "packages/a/src/util.ts", "out": out, "kind": bad }),
            )
            .unwrap_err();
            assert!(format!("{err}").contains("`kind`"), "{err}");
            assert!(!out.exists(), "a refused bridge must not create the crate");
        }
    }

    #[test]
    fn speaks_mcp() {
        let init = handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": { "protocolVersion": "2025-06-18" } })).unwrap();
        assert_eq!(init["result"]["serverInfo"]["name"], "rust-anything");
        assert!(
            handle(&json!({ "jsonrpc": "2.0", "method": "notifications/initialized" })).is_none()
        );
        let list = handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/list" })).unwrap();
        let names: Vec<&str> = list["result"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t["name"].as_str().unwrap())
            .collect();
        assert_eq!(
            names,
            [
                "analyze",
                "bridge",
                "refresh",
                "record",
                "build",
                "diff",
                "profile",
                "go",
                "enable",
                "disable",
                "report",
                "bench",
                "fuzz",
                "shadow",
                "map",
                "rules",
                "package",
                "gate",
                "app_record",
                "app_replay",
                "app_front_door",
                "app_wire",
                "app_lift",
                "app_drift"
            ]
        );
        let bad = handle(&json!({ "jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": { "name": "gate", "arguments": {} } })).unwrap();
        assert_eq!(bad["result"]["isError"], true);
        assert_eq!(
            bad["result"]["structuredContent"]["code"],
            "invalid_arguments"
        );
        let unknown = handle(&json!({ "jsonrpc": "2.0", "id": 4, "method": "nope" })).unwrap();
        assert_eq!(unknown["error"]["code"], -32601);
    }

    #[test]
    fn tools_carry_annotations() {
        let list = handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/list" })).unwrap();
        let tools = list["result"]["tools"].as_array().unwrap();
        assert_eq!(tools.len(), TOOL_META.len());
        for tool in tools {
            assert!(tool["title"].is_string(), "{}", tool["name"]);
            for hint in [
                "readOnlyHint",
                "destructiveHint",
                "idempotentHint",
                "openWorldHint",
            ] {
                assert!(
                    tool["annotations"][hint].is_boolean(),
                    "{} {hint}",
                    tool["name"]
                );
            }
        }
        let by_name =
            |n: &str| tools.iter().find(|t| t["name"] == n).unwrap()["annotations"].clone();
        assert_eq!(by_name("analyze")["readOnlyHint"], true);
        assert_eq!(by_name("analyze")["openWorldHint"], false);
        for n in [
            "record",
            "diff",
            "gate",
            "shadow",
            "bench",
            "fuzz",
            "profile",
            "app_record",
            "app_replay",
        ] {
            assert_eq!(by_name(n)["readOnlyHint"], false, "{n}");
            assert_eq!(by_name(n)["openWorldHint"], true, "{n}");
        }
        assert_eq!(by_name("app_lift")["destructiveHint"], true);
        assert_eq!(by_name("bridge")["destructiveHint"], false);
    }

    #[test]
    fn failures_are_structured() {
        let dir = std::env::temp_dir().join(format!("ra-mcp-missing-{}", std::process::id()));
        let response = handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
            "params": { "name": "gate", "arguments": { "crate": dir } } }))
        .unwrap();
        let result = &response["result"];
        assert_eq!(result["isError"], true);
        let s = &result["structuredContent"];
        assert_eq!(s["message"], result["content"][0]["text"]);
        assert!(s["code"].is_string() && s["retryable"].is_boolean());

        let args = json!({ "crate": "rust/slug" });
        let cases = [
            (
                "no recorded cases to compare; run `rust-anything record` first",
                "no_recording",
                false,
                Some("rust-anything record rust/slug"),
            ),
            (
                "no cases in rust/app-cases: run `rust-anything app record` first",
                "no_recording",
                false,
                Some("rust-anything app record rust/slug -- <command that runs the original>"),
            ),
            (
                "slug changed since it was recorded: re-run `rust-anything record rust/slug`",
                "stale_recording",
                false,
                Some("rust-anything record rust/slug"),
            ),
            (
                "`cargo` not found on PATH; run `rust-anything doctor`",
                "missing_tool",
                false,
                Some("rust-anything doctor"),
            ),
            (
                "running cargo: `cargo` not found on PATH; install it with `rustup`, or run `rust-anything doctor`",
                "missing_tool",
                false,
                Some("rust-anything doctor"),
            ),
            (
                "cargo build failed (exit status: 101):\nerror[E0308]",
                "build_failed",
                false,
                Some("fix the compiler errors in the message, then rust-anything build rust/slug"),
            ),
            (
                "`node x.mjs` was still running after 600s and was stopped",
                "timed_out",
                true,
                None,
            ),
            ("missing `crate`", "invalid_arguments", false, None),
            (
                "`unit` must be a string, not 5",
                "invalid_arguments",
                false,
                None,
            ),
            (
                "`ready` must be a string, not {\"x\":\"still running after 5s\"}",
                "invalid_arguments",
                false,
                None,
            ),
            (
                "`port` must be a non-negative integer, not \"8080\"",
                "invalid_arguments",
                false,
                None,
            ),
            ("something else broke", "failed", false, None),
        ];
        for (text, code, retryable, next) in cases {
            let f = failure(text, &args);
            assert_eq!(f["code"], code, "{text}");
            assert_eq!(f["retryable"], retryable, "{text}");
            if let Some(next) = next {
                assert_eq!(f["next"], next, "{text}");
            }
        }
        assert!(failure("something else broke", &args)["next"].is_null());
    }

    #[test]
    fn serves_the_skill_over_the_skills_extension() {
        let init =
            handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }))
                .unwrap();
        let caps = &init["result"]["capabilities"];
        assert!(caps["resources"].is_object(), "{caps}");
        assert!(
            caps["extensions"]["io.modelcontextprotocol/skills"].is_object(),
            "{caps}"
        );

        let list =
            handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "skills/list", "params": {} }))
                .unwrap();
        let result = &list["result"];
        assert_eq!(result["resultType"], "complete");
        assert!(
            result["ttlMs"].is_u64() && result["cacheScope"].is_string(),
            "{result}"
        );
        let skill = &result["skills"][0];
        assert_eq!(skill["uri"], "skill://rust-anything/SKILL.md");
        assert_eq!(skill["frontmatter"]["name"], "rust-anything");
        assert!(
            skill["frontmatter"]["description"]
                .as_str()
                .unwrap()
                .starts_with("Port or migrate")
        );
        let files = skill["resources"].as_array().unwrap();
        let python = files
            .iter()
            .find(|f| f["uri"] == "skill://rust-anything/languages/python.md")
            .expect("languages/python.md in the manifest");
        let text = include_str!("../skills/rust-anything/languages/python.md");
        assert_eq!(python["size"], text.len());
        let digest = python["digest"].as_str().unwrap();
        assert!(
            digest.starts_with("sha256:") && digest.len() == 7 + 64,
            "{digest}"
        );

        let get = handle(&json!({ "jsonrpc": "2.0", "id": 3, "method": "skills/get",
            "params": { "uri": "skill://rust-anything/SKILL.md" } }))
        .unwrap();
        assert_eq!(
            get["result"]["skill"]["uri"],
            "skill://rust-anything/SKILL.md"
        );
        assert_eq!(get["result"]["resultType"], "complete");

        let read = handle(
            &json!({ "jsonrpc": "2.0", "id": 4, "method": "resources/read",
            "params": { "uri": "skill://rust-anything/languages/python.md" } }),
        )
        .unwrap();
        let content = &read["result"]["contents"][0];
        assert_eq!(content["text"], text);
        assert_eq!(content["mimeType"], "text/markdown");

        let missing = handle(
            &json!({ "jsonrpc": "2.0", "id": 5, "method": "resources/read",
            "params": { "uri": "skill://rust-anything/nope.md" } }),
        )
        .unwrap();
        assert_eq!(missing["error"]["code"], -32002);
    }

    #[test]
    fn skill_digests_are_sha256() {
        assert_eq!(
            skill_digest(""),
            "sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
        assert_eq!(
            skill_digest("abc"),
            "sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn serves_prompts() {
        let init =
            handle(&json!({ "jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {} }))
                .unwrap();
        assert!(init["result"]["capabilities"]["prompts"].is_object());
        let list = handle(&json!({ "jsonrpc": "2.0", "id": 2, "method": "prompts/list" })).unwrap();
        let names: Vec<&str> = list["result"]["prompts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|p| p["name"].as_str().unwrap())
            .collect();
        assert_eq!(names, ["port_unit", "port_whole_app"]);
        let get = |id: i64, name: &str, arguments: Value| {
            handle(
                &json!({ "jsonrpc": "2.0", "id": id, "method": "prompts/get",
                "params": { "name": name, "arguments": arguments } }),
            )
            .unwrap()
        };
        let unit = get(
            3,
            "port_unit",
            json!({ "path": "/repo", "unit": "src/slug.ts" }),
        );
        let text = unit["result"]["messages"][0]["content"]["text"]
            .as_str()
            .unwrap();
        assert!(text.contains("`src/slug.ts`") && text.contains("/repo"));
        assert!(text.contains("# Migration to Rust with rust-anything"));
        assert!(!text.starts_with("---"));
        let whole = get(4, "port_whole_app", json!({ "path": "/repo" }));
        let text = whole["result"]["messages"][0]["content"]["text"]
            .as_str()
            .unwrap();
        assert!(text.contains("/repo") && text.contains("# Whole app to a pure Rust app"));
        assert_eq!(get(5, "port_unit", json!({}))["error"]["code"], -32602);
        assert_eq!(
            get(6, "nope", json!({ "path": "/repo" }))["error"]["code"],
            -32602
        );
    }

    #[derive(Clone, Default)]
    struct Shared(Arc<Mutex<Vec<u8>>>);

    impl Write for Shared {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            lock(&self.0).extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn answers_ping_during_a_slow_call_and_honors_cancel() {
        let input = [
            json!({ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
                "params": { "name": "test_sleep", "arguments": { "ms": 400 }, "_meta": { "progressToken": "p1" } } }),
            json!({ "jsonrpc": "2.0", "id": 2, "method": "tools/call",
                "params": { "name": "test_sleep", "arguments": { "ms": 5000 }, "_meta": { "progressToken": null } } }),
            json!({ "jsonrpc": "2.0", "method": "notifications/cancelled", "params": { "requestId": 2 } }),
            json!({ "jsonrpc": "2.0", "id": 3, "method": "ping" }),
        ]
        .iter()
        .map(|m| format!("{m}\n"))
        .collect::<String>();
        let out = Shared::default();
        let start = std::time::Instant::now();
        serve_io(std::io::Cursor::new(input), out.clone()).unwrap();
        assert!(start.elapsed() < std::time::Duration::from_secs(3));
        let text = String::from_utf8(lock(&out.0).clone()).unwrap();
        let messages: Vec<Value> = text
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        let position = |id: i64| messages.iter().position(|m| m["id"] == id);
        assert!(position(3).unwrap() < position(1).unwrap(), "{text}");
        assert_eq!(
            messages[position(1).unwrap()]["result"]["structuredContent"]["slept"],
            true
        );
        assert_eq!(position(2), None, "{text}");
        let progress: Vec<&Value> = messages
            .iter()
            .filter(|m| m["method"] == "notifications/progress")
            .collect();
        assert_eq!(progress.len(), 2, "{text}");
        assert!(
            progress
                .iter()
                .all(|p| p["params"]["progressToken"] == "p1")
        );
        assert_eq!(progress[0]["params"]["progress"], 0);
        assert_eq!(progress[1]["params"]["progress"], 1);
        assert_eq!(progress[1]["params"]["total"], 1);
        assert!(progress[1]["params"]["message"].is_string());
    }

    #[test]
    fn app_record_rejects_non_string_array_effect_response() {
        let bad_str = call(
            "app_record",
            &json!({ "repo": ".", "effect_response": "GET /x=201" }),
        );
        assert!(bad_str.is_err());
        let bad_mixed = call(
            "app_record",
            &json!({ "repo": ".", "effect_response": ["GET /x=201", 42] }),
        );
        assert!(bad_mixed.is_err());
    }

    #[test]
    fn app_record_rejects_non_string_array_args_and_fresh() {
        assert_eq!(opt_string_list(&json!({}), "fresh").unwrap(), None);
        assert_eq!(
            opt_string_list(&json!({ "fresh": null }), "fresh").unwrap(),
            None
        );
        assert_eq!(
            opt_string_list(&json!({ "fresh": [] }), "fresh").unwrap(),
            Some(Vec::new())
        );
        assert_eq!(
            opt_string_list(&json!({ "args": ["db.json"] }), "args").unwrap(),
            Some(vec!["db.json".to_string()])
        );
        assert!(call("app_record", &json!({ "repo": ".", "fresh": [42] })).is_err());
        assert!(call("app_record", &json!({ "repo": ".", "fresh": "data" })).is_err());
        assert!(call("app_record", &json!({ "repo": ".", "args": [42] })).is_err());
        assert!(call("app_record", &json!({ "repo": ".", "args": "db.json" })).is_err());
    }

    #[test]
    fn app_record_and_app_replay_reject_zero_or_negative_timeout() {
        let dir = std::env::temp_dir().join(format!("ra-mcp-timeout-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        for tool in ["app_record", "app_replay"] {
            for timeout in [json!(0), json!(-1)] {
                let err = call(tool, &json!({ "repo": dir, "timeout": timeout })).unwrap_err();
                let msg = format!("{err:#}");
                assert!(
                    msg.contains("--timeout") && msg.contains("1 second"),
                    "{tool} with {timeout}: {msg}"
                );
            }
        }
        assert!(
            !dir.join("rust").exists(),
            "rejected timeout must not create rust/app-cases"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn numeric_args_reject_wrong_types_negatives_and_overflow() {
        assert_eq!(opt_u64(&json!({}), "min_ms").unwrap(), None);
        assert_eq!(opt_u64(&json!({ "min_ms": null }), "min_ms").unwrap(), None);
        assert_eq!(
            opt_u64(&json!({ "min_ms": 250 }), "min_ms").unwrap(),
            Some(250)
        );
        for bad in [json!(-1), json!("250"), json!(1.5), json!(true)] {
            let err = opt_u64(&json!({ "min_ms": bad.clone() }), "min_ms").unwrap_err();
            assert!(format!("{err}").contains("`min_ms`"), "{bad}: {err}");
        }

        assert_eq!(
            opt_u16(&json!({ "port": 8080 }), "port").unwrap(),
            Some(8080)
        );
        assert!(opt_u16(&json!({ "port": 70_000 }), "port").is_err());
        assert!(opt_u16(&json!({ "port": -1 }), "port").is_err());
        assert!(opt_u16(&json!({ "port": "8080" }), "port").is_err());

        assert_eq!(opt_f64(&json!({}), "time").unwrap(), None);
        assert_eq!(opt_f64(&json!({ "time": 2.5 }), "time").unwrap(), Some(2.5));
        assert_eq!(opt_f64(&json!({ "time": 2 }), "time").unwrap(), Some(2.0));
        for bad in [json!("2.5"), json!(true), json!([])] {
            let err = opt_f64(&json!({ "time": bad.clone() }), "time").unwrap_err();
            assert!(format!("{err}").contains("`time`"), "{bad}: {err}");
        }

        assert!(call("fuzz", &json!({ "crate": ".", "time": "10" })).is_err());
        assert!(call("fuzz", &json!({ "crate": ".", "seed": "42" })).is_err());
        assert!(call("fuzz", &json!({ "crate": ".", "cases": -1 })).is_err());
        assert!(call("bench", &json!({ "crate": ".", "min_ms": -5 })).is_err());
        assert!(call("report", &json!({ "crate": ".", "min_ms": "100" })).is_err());
        assert!(call("diff", &json!({ "crate": ".", "float_tolerance": "1e-9" })).is_err());
        assert!(call("record", &json!({ "crate": ".", "count": -1 })).is_err());
        let bad_tol = call(
            "record",
            &json!({ "crate": ".", "float_tolerance": -1.0, "test_command": ["nonexistent-test-cmd"] }),
        )
        .unwrap_err();
        assert!(
            format!("{bad_tol:#}").contains("--float-tolerance"),
            "{bad_tol:#}"
        );
        assert!(call("app_replay", &json!({ "repo": ".", "port": 70_000 })).is_err());
        assert!(call("app_record", &json!({ "repo": ".", "pace": -10 })).is_err());
        assert!(call("app_record", &json!({ "repo": ".", "ready_timeout": "30" })).is_err());
    }

    #[test]
    fn boolean_args_reject_non_boolean_values() {
        assert_eq!(opt_bool(&json!({}), "force").unwrap(), None);
        assert_eq!(opt_bool(&json!({ "force": null }), "force").unwrap(), None);
        assert_eq!(
            opt_bool(&json!({ "force": true }), "force").unwrap(),
            Some(true)
        );
        assert_eq!(
            opt_bool(&json!({ "force": false }), "force").unwrap(),
            Some(false)
        );
        for bad in [json!("true"), json!("false"), json!(1), json!(0), json!([])] {
            let err = opt_bool(&json!({ "force": bad.clone() }), "force").unwrap_err();
            assert!(format!("{err}").contains("`force`"), "{bad}: {err}");
        }

        for (tool, args) in [
            ("analyze", json!({ "root": ".", "whole_app": "true" })),
            (
                "bridge",
                json!({ "root": ".", "unit": "src/a.ts", "force": "true" }),
            ),
            (
                "record",
                json!({ "crate": ".", "ignore_key_order": "true", "test_command": ["nonexistent"] }),
            ),
            (
                "record",
                json!({ "crate": ".", "pin_clock": 1, "test_command": ["nonexistent"] }),
            ),
            ("diff", json!({ "crate": ".", "ignore_key_order": "false" })),
            ("package", json!({ "crate": ".", "dry_run": "true" })),
            ("enable", json!({ "crate": ".", "partial": "true" })),
            ("enable", json!({ "crate": ".", "force": 1 })),
            ("app_record", json!({ "repo": ".", "http": "false" })),
            ("app_record", json!({ "repo": ".", "traps": "false" })),
            ("app_front_door", json!({ "repo": ".", "check": "false" })),
            ("app_front_door", json!({ "repo": ".", "library": 1 })),
            (
                "app_wire",
                json!({ "repo": ".", "unit": "a", "skip_gate": "true" }),
            ),
            (
                "app_wire",
                json!({ "repo": ".", "unit": "a", "check": "false" }),
            ),
        ] {
            let err = call(tool, &args).unwrap_err();
            assert!(
                format!("{err:#}").contains("must be a boolean"),
                "{tool} {args}: {err:#}"
            );
        }
    }
}
