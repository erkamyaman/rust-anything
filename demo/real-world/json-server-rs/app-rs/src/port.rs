//! The Rust port of `src/app.ts` and `src/bin.ts` (json-server, MIT): the HTTP app, the
//! database file and the command line; and of `src/rust/app-requests.ts`, the adapter that
//! sends the app a list of requests so they can be recorded and compared.

#![forbid(unsafe_code)]

pub mod cli;
pub mod db;
pub mod index;
pub mod server;

use serde_json::{Map, Value};

use self::server::{App, Request};

/// What Node's client and the adapter leave out of a response.
const SKIPPED: [&str; 7] = [
    "date",
    "connection",
    "keep-alive",
    "x-powered-by",
    "etag",
    "last-modified",
    "transfer-encoding",
];

fn is_token(s: &str) -> bool {
    !s.is_empty()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"!#$%&'*+-.^_`|~".contains(&b))
}

/// What `http.request` throws before sending, as the adapter records it.
fn client_error(method: &str, path: &str, headers: &Map<String, Value>) -> Option<String> {
    if path.chars().any(|c| !('\u{21}'..='\u{FF}').contains(&c)) {
        return Some("Request path contains unescaped characters".into());
    }
    if !method.is_empty() && !is_token(method) {
        return Some(format!("Method must be a valid HTTP token [\"{method}\"]"));
    }
    for (name, value) in headers {
        if !is_token(name) {
            return Some(format!(
                "Header name must be a valid HTTP token [\"{name}\"]"
            ));
        }
        let text = json_server_js::value::to_js_string(Some(value));
        if text
            .chars()
            .any(|c| (c < ' ' && c != '\t') || c == '\u{7f}' || c > '\u{ff}')
        {
            return Some(format!("Invalid character in header content [\"{name}\"]"));
        }
    }
    None
}

fn one(app: &App, r: &Value) -> Value {
    let field = |k: &str| r.get(k).filter(|v| !v.is_null());
    let method = field("method")
        .map(|m| json_server_js::value::to_js_string(Some(m)))
        .unwrap_or_else(|| "GET".into());
    let path = field("path")
        .map(|p| json_server_js::value::to_js_string(Some(p)))
        .unwrap_or_default();
    let headers = match field("headers") {
        Some(Value::Object(h)) => h.clone(),
        _ => Map::new(),
    };
    if let Some(threw) = client_error(&method, &path, &headers) {
        return serde_json::json!({ "threw": threw });
    }
    let method = if method.is_empty() {
        "GET".to_string()
    } else {
        method.to_uppercase()
    };
    let path = if path.is_empty() {
        "/".to_string()
    } else {
        path
    };
    let body = field("body").map(|b| json_server_js::value::to_js_string(Some(b)));
    let latin1: String = path
        .chars()
        .map(|c| if (c as u32) < 0x80 { c } else { '\u{80}' })
        .collect();
    let mut sent: Vec<(String, String)> = vec![("host".into(), "127.0.0.1".into())];
    sent.push(("connection".into(), "close".into()));
    for (k, v) in &headers {
        sent.push((k.clone(), json_server_js::value::to_js_string(Some(v))));
    }
    let mut body_bytes = body.unwrap_or_default().into_bytes();
    let has_length = headers
        .keys()
        .any(|k| k.eq_ignore_ascii_case("content-length"));
    // Node's client sends a body on these methods without a Content-Length: the server reads
    // a request with no body, then fails to parse the body as the next request and answers 400.
    let unframed = matches!(
        method.as_str(),
        "GET" | "HEAD" | "DELETE" | "OPTIONS" | "TRACE" | "CONNECT"
    ) && !body_bytes.is_empty()
        && !has_length;
    if unframed {
        body_bytes.clear();
    } else if !has_length {
        sent.push(("content-length".into(), body_bytes.len().to_string()));
    }
    let request = Request {
        method,
        target: latin1,
        headers: sent,
        body: body_bytes,
    };
    let (res, _) = app.handle(&request);
    if unframed {
        return serde_json::json!({ "status": 400, "headers": {}, "body": "" });
    }
    let mut out = Map::new();
    for (k, v) in &res.headers {
        let k = k.to_ascii_lowercase();
        if SKIPPED.contains(&k.as_str()) {
            continue;
        }
        let joined = match out.get(&k).and_then(Value::as_str) {
            Some(prev) => format!("{prev}, {v}"),
            None => v.clone(),
        };
        out.insert(k, Value::String(joined));
    }
    serde_json::json!({
        "status": res.status,
        "headers": out,
        "body": String::from_utf8_lossy(&res.body),
    })
}

/// Port of `appRequests` (`src/rust/app-requests.ts:52`).
/// `requests` is iterated as `for...of` does; data that is not an object of collections serves
/// as an empty database and comes back as it was.
pub async fn app_requests(
    data: serde_json::Value,
    requests: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let requests: Vec<Value> = match requests {
        Value::Array(items) => items,
        Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
        _ => return Err("requests is not iterable".into()),
    };
    let (map, other) = match data {
        Value::Object(map) => (map, None),
        other => (Map::new(), Some(other)),
    };
    let app = App::new(map, &[]);
    let responses: Vec<Value> = requests.iter().map(|r| one(&app, r)).collect();
    let mut made = Vec::new();
    for res in &responses {
        if res.get("status").and_then(Value::as_u64) == Some(201)
            && let Some(body) = res.get("body").and_then(Value::as_str)
            && let Ok(Value::Object(item)) = json_server_js::json::parse(body)
            && let Some(Value::String(id)) = item.get("id")
        {
            made.push(id.clone());
        }
    }
    let data = other.unwrap_or_else(|| Value::Object(app.into_data()));
    let out = serde_json::json!({ "responses": responses, "data": data });
    if made.is_empty() {
        return Ok(out);
    }
    let mut text = serde_json::to_string(&out).map_err(|e| e.to_string())?;
    for (i, id) in made.iter().enumerate() {
        let quoted = serde_json::to_string(id).map_err(|e| e.to_string())?;
        text = text.replace(&quoted, &format!("\"#{}\"", i + 1));
        text = text.replace(id.as_str(), &format!("#{}", i + 1));
    }
    serde_json::from_str(&text).map_err(|e| e.to_string())
}
