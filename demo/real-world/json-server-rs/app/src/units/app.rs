//! Glue from the front door to `app-rs`: the request as Node would hand it to tinyhttp, the
//! database file kept in step with the disk, and the answer as Node would send it.
//!
//! Reads run side by side; a write has the data alone. The file's lock is always taken before
//! the data's, so the two never wait on each other in a circle.

use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard, OnceLock, PoisonError};

use app_rs::port::cli;
use app_rs::port::db::Store;
use app_rs::port::server::{App, Request as AppRequest};
use axum::body::Body;
use axum::extract::Request;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use axum::response::Response;

struct State {
    app: App,
    store: Mutex<Option<Store>>,
    options: Option<cli::Options>,
}

fn keys(data: &serde_json::Map<String, serde_json::Value>) -> Vec<&String> {
    json_server_js::value::ordered_keys(data)
}

fn store(st: &State) -> MutexGuard<'_, Option<Store>> {
    st.store.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Opens the database before the server listens, and prints what `bin.ts` prints once it does.
pub fn banner() -> String {
    let st = state();
    match &st.options {
        Some(options) => cli::banner(options, &keys(&st.app.read().data)),
        None => String::new(),
    }
}

fn state() -> &'static State {
    static STATE: OnceLock<State> = OnceLock::new();
    STATE.get_or_init(|| {
        let args: Vec<String> = std::env::args().skip(1).collect();
        let options = cli::parse(
            &args,
            std::env::var("PORT").ok(),
            std::env::var("HOST").ok(),
        );
        let (store, data, statics) = match &options {
            Ok(o) => match Store::open(&o.file) {
                Ok((store, data)) => (Some(store), data, o.static_dirs.clone()),
                Err(e) => {
                    eprintln!("{}: {e}", o.file.display());
                    std::process::exit(1);
                }
            },
            Err(_) => (None, Default::default(), Vec::new()),
        };
        State {
            app: App::new(data, &statics),
            store: Mutex::new(store),
            options: options.ok(),
        }
    })
}

/// Takes the file's new contents when someone else changed it.
fn reload(st: &State) {
    let mut guard = store(st);
    let Some(store) = guard.as_mut() else { return };
    let Some(data) = store.changed() else { return };
    let mut service = st.app.write();
    let mut before: Vec<&String> = keys(&service.data);
    let mut after: Vec<&String> = keys(&data);
    before.sort();
    after.sort();
    if before != after
        && let Some(options) = &st.options
    {
        println!("\n{}", cli::routes(options, &keys(&data)).trim_end());
    }
    service.data = data;
    service.prototype = None;
}

/// Answers one request in Rust.
pub async fn answer(req: Request) -> Response {
    let (parts, body) = req.into_parts();
    let body = axum::body::to_bytes(body, usize::MAX)
        .await
        .unwrap_or_default();
    let target = parts
        .uri
        .path_and_query()
        .map_or_else(|| parts.uri.to_string(), |p| p.as_str().to_string());
    let headers = parts
        .headers
        .iter()
        .map(|(k, v)| {
            let value: String = v.as_bytes().iter().map(|&b| char::from(b)).collect();
            (k.as_str().to_string(), value)
        })
        .collect();
    let request = AppRequest {
        method: parts.method.as_str().to_string(),
        target,
        headers,
        body: body.to_vec(),
    };
    let st = state();
    reload(st);
    let (res, changed) = st.app.handle(&request);
    let write = if changed {
        let guard = store(st);
        guard.as_ref().map(|s| s.write(&st.app.read().data))
    } else {
        None
    };
    if let Some(write) = write {
        match tokio::task::spawn_blocking(move || write.finish()).await {
            Ok(Ok(())) => {}
            Ok(Err(e)) => eprintln!("writing the database file: {e}"),
            Err(e) => eprintln!("writing the database file: {e}"),
        }
    }
    let mut out = Response::builder()
        .status(StatusCode::from_u16(res.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR));
    for (k, v) in &res.headers {
        if let (Ok(name), Ok(value)) = (
            HeaderName::from_bytes(k.as_bytes()),
            HeaderValue::from_bytes(v.as_bytes()),
        ) {
            out = out.header(name, value);
        }
    }
    if res.close {
        out = out.header("connection", "close");
    }
    out.body(Body::from(res.body))
        .unwrap_or_else(|_| Response::new(Body::empty()))
}

/// `route` is the route as wired (`GET /users/{id}`, or `*`), `params` its path parameters.
pub async fn handle(_route: &str, _params: HashMap<String, String>, req: Request) -> Response {
    answer(req).await
}
