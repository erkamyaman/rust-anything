//! `createApp` (`src/app.ts`) on @tinyhttp/app 3.0.1's router, as it runs: every middleware
//! and route whose path matches, tried in order, re-matched each time `next()` is called on the
//! URL the last middleware left. That is why `HEAD /posts` ends in `POST`'s 400 and `//posts`
//! skips the CORS headers; the port keeps both.

use std::path::PathBuf;
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard};

use json_server_js::url::{self, QueryValue};
use json_server_js::{json, number, value};
use serde_json::{Map, Value};
use service_rs::port::service::{FindOpts, Found, Service};

use json_server_js::sirv::{self, Folder};

use super::index;

/// A request as the server received it.
#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    /// The request target as sent: path and query, not decoded.
    pub target: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

/// Headers Node keeps the first of when a request repeats them; others are joined with `, `.
const SINGLE_HEADERS: [&str; 18] = [
    "age",
    "authorization",
    "content-length",
    "content-type",
    "etag",
    "expires",
    "from",
    "host",
    "if-modified-since",
    "if-unmodified-since",
    "last-modified",
    "location",
    "max-forwards",
    "proxy-authorization",
    "referer",
    "retry-after",
    "server",
    "user-agent",
];

impl Request {
    /// `req.headers[name]`, as Node folds repeated headers.
    pub fn header(&self, name: &str) -> Option<String> {
        let mut values = self
            .headers
            .iter()
            .filter(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str());
        let first = values.next()?;
        if SINGLE_HEADERS.contains(&name) {
            return Some(first.to_string());
        }
        let mut out = first.to_string();
        for v in values {
            out.push_str(", ");
            out.push_str(v);
        }
        Some(out)
    }
}

/// A response as Node would send it.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    /// In the order they were set; a header set to a list appears once per value.
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    /// The connection closes after this response (Node's answer to a request it cannot parse).
    pub close: bool,
    ended: bool,
    head_written: bool,
}

impl Response {
    fn new() -> Response {
        Response {
            status: 200,
            headers: Vec::new(),
            body: Vec::new(),
            close: false,
            ended: false,
            head_written: false,
        }
    }

    /// `res.setHeader(name, value)`.
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.set_header_values(name, &[value.to_string()]);
    }

    /// `res.setHeader(name, [values])`: one header line per value.
    pub fn set_header_values(&mut self, name: &str, values: &[String]) {
        match self
            .headers
            .iter()
            .position(|(k, _)| k.eq_ignore_ascii_case(name))
        {
            Some(at) => {
                self.headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
                for (i, v) in values.iter().enumerate() {
                    self.headers.insert(at + i, (name.to_string(), v.clone()));
                }
            }
            None => {
                for v in values {
                    self.headers.push((name.to_string(), v.clone()));
                }
            }
        }
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str())
    }

    fn remove_header(&mut self, name: &str) {
        self.headers.retain(|(k, _)| !k.eq_ignore_ascii_case(name));
    }

    /// `res.writeHead(status)`: the headers are fixed, so Node will not add a Content-Length.
    pub fn write_head(&mut self) {
        self.head_written = true;
    }

    /// `res.end(body)`.
    pub fn end(&mut self, body: Vec<u8>) {
        self.body = body;
        self.ended = true;
    }

    /// What Node adds and drops when the response goes out.
    fn finish(&mut self, method: &str) {
        let bodiless = method == "HEAD" || self.status == 204 || self.status == 304;
        if !self.head_written && !bodiless && self.header("Content-Length").is_none() {
            let len = self.body.len().to_string();
            self.headers.push(("Content-Length".into(), len));
        }
        if bodiless {
            self.body.clear();
        }
    }

    /// Whether Node sends this body in chunks (no Content-Length).
    pub fn chunked(&self) -> bool {
        self.header("Content-Length").is_none() && self.status != 204 && self.status != 304
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Pattern {
    /// `use(fn)`: any path.
    Any,
    /// `get('/')`.
    Root,
    /// `options('*')`.
    Wild,
    /// `'/:name'` as a route.
    Name,
    /// `'/:name/:id'`.
    NameId,
    /// `use('/:name', fn)`: the first segment, and anything under it.
    NamePrefix,
}

impl Pattern {
    fn path(self) -> &'static str {
        match self {
            Pattern::Any | Pattern::Root => "/",
            Pattern::Wild => "*",
            Pattern::Name | Pattern::NamePrefix => "/:name",
            Pattern::NameId => "/:name/:id",
        }
    }

    /// The raw captures when `pathname` matches (regexparam's regexes, by hand).
    fn captures(self, pathname: &str) -> Option<Vec<(&'static str, String)>> {
        let rest = pathname.strip_prefix('/');
        match self {
            Pattern::Any => (pathname.is_empty() || rest.is_some()).then(Vec::new),
            Pattern::Root => (pathname.is_empty() || pathname == "/").then(Vec::new),
            Pattern::Wild => rest.map(|r| vec![("wild", r.to_string())]),
            Pattern::Name => {
                let r = rest?;
                let r = r.strip_suffix('/').filter(|s| !s.is_empty()).unwrap_or(r);
                (!r.is_empty() && !r.contains('/')).then(|| vec![("name", r.to_string())])
            }
            Pattern::NameId => {
                let r = rest?;
                let (name, tail) = r.split_once('/')?;
                let id = tail
                    .strip_suffix('/')
                    .filter(|s| !s.is_empty())
                    .unwrap_or(tail);
                (!name.is_empty() && !id.is_empty() && !id.contains('/'))
                    .then(|| vec![("name", name.to_string()), ("id", id.to_string())])
            }
            Pattern::NamePrefix => {
                let r = rest?;
                let name = r.split('/').next().unwrap_or("");
                (!name.is_empty()).then(|| vec![("name", name.to_string())])
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Handler {
    Static(usize),
    Cors,
    Preflight,
    Body,
    Index,
    List,
    Item,
    Create,
    Update,
    UpdateById,
    Patch,
    PatchById,
    Destroy,
    Answer,
}

#[derive(Debug, Clone, Copy)]
struct Def {
    handler: Handler,
    middleware: bool,
    method: Option<&'static str>,
    pattern: Pattern,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Slot {
    Extensions,
    Def(usize),
    HeadEnd,
    NoMatch,
}

impl Slot {
    /// `(path, is middleware)` of the entries tinyhttp adds itself.
    fn shape(self, defs: &[Def]) -> (Pattern, bool, bool) {
        match self {
            Slot::Extensions | Slot::HeadEnd => (Pattern::Any, true, false),
            Slot::NoMatch => (Pattern::Any, false, false),
            Slot::Def(i) => (defs[i].pattern, defs[i].middleware, true),
        }
    }
}

enum Flow {
    Next,
    Fail(String),
    Done,
}

struct Ctx<'r> {
    req: &'r Request,
    url: String,
    base_url: String,
    original_url: String,
    params: Map<String, Value>,
    query: Vec<(String, QueryValue)>,
    body: Option<Value>,
    data: Option<Found>,
    res: Response,
    changed: bool,
}

impl Ctx<'_> {
    fn param(&self, key: &str) -> String {
        self.params
            .get(key)
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string()
    }

    fn query_list(&self, key: &str) -> Vec<String> {
        url::query_get(&self.query, key).map_or_else(Vec::new, QueryValue::to_vec)
    }
}

fn lead(x: &str) -> String {
    if x.starts_with('/') {
        x.to_string()
    } else {
        format!("/{x}")
    }
}

fn trail(x: &str) -> &str {
    x.strip_suffix('/').unwrap_or(x)
}

/// `str.substring(0, n)` / `str.substring(n)` on a URL by UTF-16 length `n`.
fn split_utf16(s: &str, n: usize) -> (&str, &str) {
    let mut units = 0;
    for (i, c) in s.char_indices() {
        if units >= n {
            return (&s[..i], &s[i..]);
        }
        units += c.len_utf16();
    }
    (s, "")
}

/// The methods Node's HTTP parser accepts (`http.METHODS`); it answers any other with a 400.
const NODE_METHODS: [&str; 35] = [
    "ACL",
    "BIND",
    "CHECKOUT",
    "CONNECT",
    "COPY",
    "DELETE",
    "GET",
    "HEAD",
    "LINK",
    "LOCK",
    "M-SEARCH",
    "MERGE",
    "MKACTIVITY",
    "MKCALENDAR",
    "MKCOL",
    "MOVE",
    "NOTIFY",
    "OPTIONS",
    "PATCH",
    "POST",
    "PROPFIND",
    "PROPPATCH",
    "PURGE",
    "PUT",
    "QUERY",
    "REBIND",
    "REPORT",
    "SEARCH",
    "SOURCE",
    "SUBSCRIBE",
    "TRACE",
    "UNBIND",
    "UNLINK",
    "UNLOCK",
    "UNSUBSCRIBE",
];

const RESERVED_QUERY_KEYS: [&str; 5] = ["_sort", "_page", "_per_page", "_embed", "_where"];

/// `-0` comes out of `parseWhere` marked for JS; JSON-side it is just 0.
fn unmark(v: Value) -> Value {
    match v {
        Value::Object(map) if map.len() == 1 && map.contains_key("$num") => value::num(0.0),
        Value::Object(map) => Value::Object(map.into_iter().map(|(k, v)| (k, unmark(v))).collect()),
        Value::Array(items) => Value::Array(items.into_iter().map(unmark).collect()),
        other => other,
    }
}

fn is_item(v: Option<&Value>) -> Option<Map<String, Value>> {
    match v {
        Some(Value::Object(m)) => Some(m.clone()),
        _ => None,
    }
}

fn etag(body: &[u8]) -> String {
    use base64_like::encode;
    let digest = sha1_smol::Sha1::from(body).digest().bytes();
    let hash = encode(&digest);
    format!("W/\"{:x}-{}\"", body.len(), &hash[..27])
}

mod base64_like {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    /// Standard base64 with padding.
    pub fn encode(bytes: &[u8]) -> String {
        let mut out = String::new();
        for chunk in bytes.chunks(3) {
            let b = [
                chunk[0],
                *chunk.get(1).unwrap_or(&0),
                *chunk.get(2).unwrap_or(&0),
            ];
            let n = (u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2]);
            for i in 0..4 {
                if i <= chunk.len() {
                    out.push(ALPHABET[((n >> (18 - 6 * i)) & 63) as usize] as char);
                } else {
                    out.push('=');
                }
            }
        }
        out
    }
}

/// jshttp `fresh` as tinyhttp has it, for a response with an ETag and no Last-Modified.
fn fresh(req: &Request, etag: &str) -> bool {
    let modified_since = req.header("if-modified-since").filter(|s| !s.is_empty());
    let none_match = req.header("if-none-match").filter(|s| !s.is_empty());
    if modified_since.is_none() && none_match.is_none() {
        return false;
    }
    if let Some(cc) = req.header("cache-control")
        && cc.split(',').any(|d| d.trim() == "no-cache")
    {
        return false;
    }
    if let Some(none_match) = &none_match
        && none_match != "*"
    {
        let same = |t: &str| t == etag || format!("W/{t}") == etag || t == format!("W/{etag}");
        if !none_match.split(',').map(str::trim).any(same) {
            return false;
        }
    }
    modified_since.is_none()
}

/// The app: the Service over the database, and the folders sirv serves. Reads share the
/// Service; a write has it alone.
pub struct App {
    pub service: RwLock<Service>,
    folders: Vec<Folder>,
    defs: Vec<Def>,
}

impl App {
    /// `createApp(db, { static })`: `public` and the static folders, relative to the working
    /// directory.
    pub fn new(data: Map<String, Value>, static_dirs: &[PathBuf]) -> App {
        let mut folders = vec![Folder::new(std::path::Path::new("public"))];
        folders.extend(static_dirs.iter().map(|d| Folder::new(d)));
        let mut defs: Vec<Def> = (0..folders.len())
            .map(|i| Def {
                handler: Handler::Static(i),
                middleware: true,
                method: None,
                pattern: Pattern::Any,
            })
            .collect();
        let route = |handler, method, pattern| Def {
            handler,
            middleware: false,
            method: Some(method),
            pattern,
        };
        defs.push(Def {
            handler: Handler::Cors,
            middleware: true,
            method: None,
            pattern: Pattern::Any,
        });
        defs.push(route(Handler::Preflight, "OPTIONS", Pattern::Wild));
        defs.push(Def {
            handler: Handler::Body,
            middleware: true,
            method: None,
            pattern: Pattern::Any,
        });
        defs.push(route(Handler::Index, "GET", Pattern::Root));
        defs.push(route(Handler::List, "GET", Pattern::Name));
        defs.push(route(Handler::Item, "GET", Pattern::NameId));
        defs.push(route(Handler::Create, "POST", Pattern::Name));
        defs.push(route(Handler::Update, "PUT", Pattern::Name));
        defs.push(route(Handler::UpdateById, "PUT", Pattern::NameId));
        defs.push(route(Handler::Patch, "PATCH", Pattern::Name));
        defs.push(route(Handler::PatchById, "PATCH", Pattern::NameId));
        defs.push(route(Handler::Destroy, "DELETE", Pattern::NameId));
        defs.push(Def {
            handler: Handler::Answer,
            middleware: true,
            method: None,
            pattern: Pattern::NamePrefix,
        });
        App {
            service: RwLock::new(Service::new(data)),
            folders,
            defs,
        }
    }

    /// The Service, shared with other reads.
    pub fn read(&self) -> RwLockReadGuard<'_, Service> {
        self.service.read().unwrap_or_else(PoisonError::into_inner)
    }

    /// The Service, alone.
    pub fn write(&self) -> RwLockWriteGuard<'_, Service> {
        self.service.write().unwrap_or_else(PoisonError::into_inner)
    }

    /// The data, once the app is done with it.
    pub fn into_data(self) -> Map<String, Value> {
        self.service
            .into_inner()
            .unwrap_or_else(PoisonError::into_inner)
            .data
    }

    /// Answers one request; the flag says whether the data changed (and the file must be
    /// written).
    pub fn handle(&self, req: &Request) -> (Response, bool) {
        if req
            .target
            .bytes()
            .any(|b| b >= 0x80 || b <= 0x20 || b == 0x7f)
            || !(req.target.starts_with('/') || req.target == "*")
            || !NODE_METHODS.contains(&req.method.as_str())
        {
            let mut res = Response::new();
            res.status = 400;
            res.close = true;
            res.head_written = true;
            return (res, false);
        }
        let mut ctx = Ctx {
            req,
            url: req.target.clone(),
            base_url: String::new(),
            original_url: String::new(),
            params: Map::new(),
            query: Vec::new(),
            body: None,
            data: None,
            res: Response::new(),
            changed: false,
        };
        ctx.res.set_header("X-Powered-By", "tinyhttp");
        let mut mw = vec![Slot::Extensions];
        let mut idx = 0;
        loop {
            ctx.original_url = format!("{}{}", ctx.base_url, ctx.url);
            let pathname = ctx.url.split('?').next().unwrap_or("").to_string();
            let matched: Vec<Slot> = (0..self.defs.len())
                .map(Slot::Def)
                .filter(|slot| {
                    let Slot::Def(i) = slot else { return false };
                    let d = &self.defs[*i];
                    d.pattern.captures(&pathname).is_some()
                        && (req.method == "HEAD" || d.method.is_none_or(|m| m == req.method))
                        && !mw.contains(slot)
                })
                .collect();
            if !matched.is_empty() {
                if idx != 0 {
                    idx = mw.len();
                    ctx.params = Map::new();
                }
                mw.extend(matched);
                mw.push(Slot::HeadEnd);
            } else {
                mw.push(Slot::NoMatch);
            }
            let slot = mw[idx];
            idx += 1;
            match self.run(slot, &mut ctx, &pathname) {
                Flow::Done => break,
                Flow::Fail(message) => {
                    on_error(&mut ctx.res, &message);
                    break;
                }
                Flow::Next => {
                    if ctx.res.ended {
                        break;
                    }
                    if idx >= mw.len() {
                        no_match(&mut ctx.res);
                        break;
                    }
                }
            }
        }
        ctx.res.finish(&req.method);
        (ctx.res, ctx.changed)
    }

    fn run(&self, slot: Slot, ctx: &mut Ctx, pathname: &str) -> Flow {
        let (pattern, middleware, has_regex) = slot.shape(&self.defs);
        let mut params: Vec<(&str, String)> = Vec::new();
        if has_regex {
            for (key, raw) in pattern.captures(pathname).unwrap_or_default() {
                if raw.is_empty() {
                    continue;
                }
                match url::decode_uri_component(&raw) {
                    Ok(v) => params.push((key, v)),
                    Err(_) => {
                        eprintln!("URIError: URI malformed");
                        send_status(ctx, 400);
                        return Flow::Done;
                    }
                }
            }
        }
        let mut prefix = pattern.path().to_string();
        for key in ["wild", "name", "id"] {
            let value = params
                .iter()
                .find(|(k, _)| *k == key)
                .map_or("undefined", |(_, v)| v.as_str());
            if key == "wild" && pattern == Pattern::Wild {
                prefix = prefix.replacen('*', value, 1);
            } else if prefix.contains(&format!(":{key}")) {
                prefix = prefix.replacen(&format!(":{key}"), value, 1);
            }
        }
        for (k, v) in params {
            ctx.params.insert(k.to_string(), Value::String(v));
        }
        if middleware {
            let n = prefix.encode_utf16().count();
            let (base, rest) = split_utf16(&ctx.original_url, n);
            ctx.url = lead(rest);
            ctx.base_url = trail(base).to_string();
        }
        match slot {
            Slot::Extensions => {
                let raw = ctx.req.target.as_str();
                let query = raw.find('?').map_or("", |i| &raw[i + 1..]);
                ctx.query = url::querystring(query);
                Flow::Next
            }
            Slot::HeadEnd => {
                if ctx.req.method == "HEAD" {
                    ctx.res.status = 204;
                    ctx.res.end(Vec::new());
                    return Flow::Done;
                }
                Flow::Next
            }
            Slot::NoMatch => {
                no_match(&mut ctx.res);
                Flow::Done
            }
            Slot::Def(i) => self.call(self.defs[i].handler, ctx),
        }
    }

    fn call(&self, handler: Handler, ctx: &mut Ctx) -> Flow {
        match handler {
            Handler::Static(i) => self.serve_static(i, ctx),
            Handler::Cors => {
                let allowed = ctx.req.header("access-control-request-headers").map(|h| {
                    h.split(',')
                        .map(|p| number::trim(p).to_string())
                        .collect::<Vec<_>>()
                });
                cors(ctx, allowed)
            }
            Handler::Preflight => cors(ctx, None),
            Handler::Body => parse_body(ctx),
            Handler::Index => {
                let html = index::render(&self.read().data).into_bytes();
                send_text(ctx, html, "text/html; charset=utf-8");
                Flow::Done
            }
            Handler::List => match self.list(ctx) {
                Ok(found) => {
                    ctx.data = Some(found);
                    Flow::Next
                }
                Err(e) => Flow::Fail(e),
            },
            Handler::Item => {
                let embed = ctx.query_list("_embed");
                match self
                    .read()
                    .find_by_id(&ctx.param("name"), &ctx.param("id"), &embed)
                {
                    Ok(found) => {
                        ctx.data = Some(found);
                        Flow::Next
                    }
                    Err(e) => Flow::Fail(e),
                }
            }
            Handler::Create | Handler::Update | Handler::Patch => {
                let Some(body) = is_item(ctx.body.as_ref()) else {
                    return bad_body(ctx);
                };
                let name = ctx.param("name");
                let done = match handler {
                    Handler::Create => self.write().create(&name, body),
                    Handler::Update => self.write().update(&name, body),
                    _ => self.write().patch(&name, body),
                };
                self.wrote(ctx, done)
            }
            Handler::UpdateById | Handler::PatchById => {
                let Some(body) = is_item(ctx.body.as_ref()) else {
                    return bad_body(ctx);
                };
                let (name, id) = (ctx.param("name"), ctx.param("id"));
                let done = if handler == Handler::UpdateById {
                    self.write().update_by_id(&name, &id, body)
                } else {
                    self.write().patch_by_id(&name, &id, body)
                };
                self.wrote(ctx, done)
            }
            Handler::Destroy => {
                let dependent = ctx.query_list("_dependent");
                let done =
                    self.write()
                        .destroy_by_id(&ctx.param("name"), &ctx.param("id"), &dependent);
                self.wrote(ctx, done)
            }
            Handler::Answer => {
                match ctx.data.take() {
                    None | Some(Found::Undefined) => {
                        ctx.res.status = 404;
                        send_json(
                            ctx,
                            &Found::Value(serde_json::json!({ "error": "Not Found" })),
                        );
                    }
                    Some(found) => {
                        if ctx.req.method == "POST" {
                            ctx.res.status = 201;
                        }
                        send_json(ctx, &found);
                    }
                }
                Flow::Done
            }
        }
    }

    fn wrote(&self, ctx: &mut Ctx, done: Option<Value>) -> Flow {
        ctx.changed |= done.is_some();
        ctx.data = Some(done.map_or(Found::Undefined, Found::Value));
        Flow::Next
    }

    fn list(&self, ctx: &Ctx) -> Result<Found, String> {
        let query = ctx.url.split('?').nth(1).unwrap_or("");
        let params = url::search_params(query);
        let filter: Vec<(String, String)> = params
            .iter()
            .filter(|(k, _)| !RESERVED_QUERY_KEYS.contains(&k.as_str()))
            .cloned()
            .collect();
        let serialized = url::serialize_search_params(&filter);
        let mut r#where = Value::Object(parse_where_rs::port::parse_where_pairs(
            &url::search_params(&serialized),
        )?);
        let get = |key: &str| {
            params
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str())
        };
        if let Some(raw) = get("_where")
            && let Ok(parsed @ (Value::Object(_) | Value::Array(_))) = json::parse(raw)
        {
            r#where = parsed;
        }
        let int = |key: &str| get(key).map(number::parse_int).filter(|n| !n.is_nan());
        let opts = FindOpts {
            r#where: unmark(r#where),
            sort: get("_sort").map(str::to_string),
            page: int("_page"),
            per_page: int("_per_page"),
            embed: ctx.query_list("_embed"),
        };
        self.read().find(&ctx.param("name"), &opts)
    }

    fn serve_static(&self, i: usize, ctx: &mut Ctx) -> Flow {
        let raw = ctx.url.as_str();
        let mut pathname = raw.to_string();
        if raw.len() > 1 {
            if let Some(h) = raw[1..].find('#') {
                pathname = raw[..h + 1].to_string();
            }
            if let Some(q) = pathname[1..].find('?') {
                pathname.truncate(q + 1);
            }
        }
        if pathname.contains('%')
            && let Ok(decoded) = url::decode_uri(&pathname)
        {
            pathname = decoded;
        }
        let Some((file, headers, size)) = self.folders[i].lookup(&pathname) else {
            return Flow::Next;
        };
        let range = ctx.req.header("range");
        match sirv::send(&file, headers, size, range.as_deref()) {
            Ok(served) => {
                ctx.res.status = served.status;
                for (k, v) in &served.headers {
                    ctx.res.set_header(k, v);
                }
                if served.head_written {
                    ctx.res.write_head();
                }
                ctx.res.end(served.body);
                Flow::Done
            }
            Err(e) => Flow::Fail(e),
        }
    }
}

fn cors(ctx: &mut Ctx, allowed: Option<Vec<String>>) -> Flow {
    let res = &mut ctx.res;
    res.set_header("Access-Control-Allow-Origin", "*");
    res.set_header(
        "Access-Control-Allow-Methods",
        "GET, HEAD, PUT, PATCH, POST, DELETE",
    );
    res.set_header_values(
        "Access-Control-Allow-Headers",
        &allowed.unwrap_or_else(|| vec!["content-type".to_string()]),
    );
    if ctx.req.method == "OPTIONS" {
        res.status = 204;
        res.set_header("Content-Length", "0");
        res.end(Vec::new());
        return Flow::Done;
    }
    Flow::Next
}

const PAYLOAD_LIMIT: usize = 102_400;

/// milliparsec checks the limit before each chunk it reads, against the chunks before it, so a
/// body passes while all but its last 64 KiB read stay under the limit.
const READ_SIZE: usize = 65_536;

/// milliparsec's `json()`: every POST, PUT, PATCH and DELETE body is JSON, whatever its type;
/// an empty body is `{}`.
fn parse_body(ctx: &mut Ctx) -> Flow {
    if !matches!(ctx.req.method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE") {
        return Flow::Next;
    }
    let before_last_read = ctx.req.body.len().saturating_sub(1) / READ_SIZE * READ_SIZE;
    if before_last_read > PAYLOAD_LIMIT {
        return Flow::Fail(format!("Payload too large. Limit: {PAYLOAD_LIMIT} bytes"));
    }
    let bytes = ctx
        .req
        .body
        .strip_prefix(b"\xEF\xBB\xBF".as_slice())
        .unwrap_or(&ctx.req.body);
    let text = String::from_utf8_lossy(bytes);
    if text.is_empty() {
        ctx.body = Some(Value::Object(Map::new()));
        return Flow::Next;
    }
    match json::parse(&text) {
        Ok(v) => {
            ctx.body = Some(v);
            Flow::Next
        }
        Err(e) => Flow::Fail(e),
    }
}

fn bad_body(ctx: &mut Ctx) -> Flow {
    ctx.res.status = 400;
    send_json(
        ctx,
        &Found::Value(serde_json::json!({ "error": "Body must be a JSON object" })),
    );
    Flow::Done
}

/// `res.json(data)`.
fn send_json(ctx: &mut Ctx, data: &Found) {
    ctx.res.set_header("Content-Type", "application/json");
    let body = match data {
        Found::Undefined | Found::Function | Found::Value(Value::Null) => {
            ctx.res.remove_header("Content-Length");
            ctx.res.write_head();
            Vec::new()
        }
        Found::Prototype => b"{}".to_vec(),
        Found::Json(text) => text.clone().into_bytes(),
        Found::Value(Value::String(s)) => s.clone().into_bytes(),
        Found::Value(v) => json::stringify(v, 2).into_bytes(),
    };
    ctx.res.end(body);
}

/// `res.send(text)` with the type already chosen, ETag and freshness included.
fn send_text(ctx: &mut Ctx, body: Vec<u8>, content_type: &str) {
    ctx.res.set_header("Content-Type", content_type);
    let mut body = body;
    if !body.is_empty() && ctx.res.header("etag").is_none() {
        let tag = etag(&body);
        ctx.res.set_header("etag", &tag);
    }
    let status = ctx.res.status;
    let is_get = matches!(ctx.req.method.as_str(), "GET" | "HEAD");
    if is_get
        && ((200..300).contains(&status) || status == 304)
        && let Some(tag) = ctx.res.header("etag").map(str::to_string)
        && fresh(ctx.req, &tag)
    {
        ctx.res.status = 304;
    }
    if ctx.res.status == 204 || ctx.res.status == 304 {
        ctx.res.remove_header("Content-Type");
        ctx.res.remove_header("Content-Length");
        ctx.res.remove_header("Transfer-Encoding");
        body = Vec::new();
    }
    ctx.res.end(body);
}

/// `res.sendStatus(code)`.
fn send_status(ctx: &mut Ctx, code: u16) {
    ctx.res.status = code;
    let text = match code {
        400 => "Bad Request",
        _ => "Error",
    };
    send_text(ctx, text.as_bytes().to_vec(), "text/plain; charset=utf-8");
}

/// tinyhttp's `onError` for an `Error`: status 500 and the message, no type.
fn on_error(res: &mut Response, message: &str) {
    eprintln!("Error: {message}");
    res.status = 500;
    res.write_head();
    res.end(message.as_bytes().to_vec());
}

fn no_match(res: &mut Response) {
    res.status = 404;
    res.write_head();
    res.end(b"Not Found".to_vec());
}
