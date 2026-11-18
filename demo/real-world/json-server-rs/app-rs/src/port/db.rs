//! The database file as `bin.ts` keeps it: lowdb's `JSONFile` (`JSON.stringify(data, null, 2)`,
//! written to a temporary file and renamed), `NormalizedAdapter` (drops `$schema` on read, makes
//! every item id a string, gives items without one a random id, and writes `$schema` back), and
//! the reload when the file changes on disk.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use json_server_js::{json, number};
use serde_json::{Map, Value};

pub const SCHEMA_PATH: &str = "./node_modules/json-server/schema.json";

/// `NormalizedAdapter.read`'s changes, on the parsed file. `Err` where JS throws.
pub fn normalize(data: &mut Map<String, Value>) -> Result<(), String> {
    data.shift_remove("$schema");
    for value in data.values_mut() {
        let Value::Array(items) = value else { continue };
        for item in items.iter_mut() {
            match item {
                Value::Object(fields) => match fields.get("id") {
                    Some(Value::Number(n)) => {
                        let text = number::number_to_string(n.as_f64().unwrap_or(f64::NAN));
                        fields.insert("id".into(), Value::String(text));
                    }
                    None => {
                        let id = service_rs::port::service::random_id();
                        fields.insert("id".into(), Value::String(id));
                    }
                    Some(_) => {}
                },
                Value::Null => {
                    return Err("Cannot read properties of null (reading 'id')".into());
                }
                Value::Array(_) => {}
                other => {
                    let kind = match other {
                        Value::Bool(_) => "boolean",
                        Value::Number(_) => "number",
                        _ => "string",
                    };
                    let shown = json_server_js::value::to_js_string(Some(other));
                    return Err(format!("Cannot create property 'id' on {kind} '{shown}'"));
                }
            }
        }
    }
    Ok(())
}

/// The file's text: `JSON.stringify({ ...data, $schema }, null, 2)`.
pub fn serialize(data: &Map<String, Value>) -> String {
    if data.contains_key("$schema") {
        let mut out = data.clone();
        out.insert("$schema".into(), Value::String(SCHEMA_PATH.into()));
        return json::stringify(&Value::Object(out), 2);
    }
    let mut text = String::with_capacity(1024);
    json::write_map(data, 2, 0, &mut text);
    let schema = format!("\"$schema\": \"{SCHEMA_PATH}\"");
    if data.is_empty() {
        return format!("{{\n  {schema}\n}}");
    }
    text.truncate(text.len() - 2);
    text.push_str(&format!(",\n  {schema}\n}}"));
    text
}

/// Parses the file's text into the database's data (`None` for `null`).
pub fn parse(text: &str) -> Result<Option<Map<String, Value>>, String> {
    match json::parse(text)? {
        Value::Object(mut map) => {
            normalize(&mut map)?;
            Ok(Some(map))
        }
        Value::Null => Ok(None),
        _ => Err("the database file must hold a JSON object".into()),
    }
}

type Stamp = Option<(SystemTime, u64)>;

fn stamp(path: &Path) -> Stamp {
    let meta = std::fs::metadata(path).ok()?;
    Some((meta.modified().ok()?, meta.len()))
}

struct Shared {
    /// The file as last read or written by us.
    seen: Mutex<Stamp>,
    /// Writes taken from the data and not finished yet.
    pending: AtomicUsize,
    next: AtomicU64,
    /// The newest write on disk; its lock also keeps writes one at a time.
    written: Mutex<u64>,
}

/// The file behind the database.
pub struct Store {
    pub path: PathBuf,
    shared: Arc<Shared>,
}

/// One write of the data, taken under the data's lock and done outside it.
pub struct Write {
    path: PathBuf,
    text: String,
    generation: u64,
    shared: Arc<Shared>,
}

impl Store {
    /// Reads the file (a missing file is an empty database).
    pub fn open(path: &Path) -> Result<(Store, Map<String, Value>), String> {
        let seen = stamp(path);
        let data = match std::fs::read_to_string(path) {
            Ok(text) => parse(&text)?.unwrap_or_default(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Map::new(),
            Err(e) => return Err(e.to_string()),
        };
        let shared = Arc::new(Shared {
            seen: Mutex::new(seen),
            pending: AtomicUsize::new(0),
            next: AtomicU64::new(0),
            written: Mutex::new(0),
        });
        Ok((
            Store {
                path: path.to_path_buf(),
                shared,
            },
            data,
        ))
    }

    /// The new data when someone else changed the file since we last read or wrote it. A file
    /// that does not parse is reported and the data kept, as json-server does.
    pub fn changed(&mut self) -> Option<Map<String, Value>> {
        if self.shared.pending.load(Ordering::SeqCst) > 0 {
            return None;
        }
        let now = stamp(&self.path);
        {
            let mut seen = lock(&self.shared.seen);
            if now == *seen {
                return None;
            }
            *seen = now;
        }
        let text = std::fs::read_to_string(&self.path).ok()?;
        match parse(&text) {
            Ok(data) => data,
            Err(message) => {
                println!("\nError parsing {}\n{message}", self.path.display());
                None
            }
        }
    }

    /// Takes the data to write. Call [`Write::finish`] without holding the data's lock: writes
    /// that queue up are coalesced, as lowdb's writer does, and only the newest reaches the disk.
    pub fn write(&self, data: &Map<String, Value>) -> Write {
        self.shared.pending.fetch_add(1, Ordering::SeqCst);
        Write {
            path: self.path.clone(),
            text: serialize(data),
            generation: self.shared.next.fetch_add(1, Ordering::SeqCst) + 1,
            shared: Arc::clone(&self.shared),
        }
    }
}

fn lock<T>(m: &Mutex<T>) -> std::sync::MutexGuard<'_, T> {
    match m.lock() {
        Ok(g) => g,
        Err(poisoned) => poisoned.into_inner(),
    }
}

impl Write {
    /// Writes the data as lowdb does (to `.<name>.tmp` beside the file, then renamed over it),
    /// unless a newer write already did.
    pub fn finish(self) -> Result<(), String> {
        let result = (|| {
            let mut written = lock(&self.shared.written);
            if *written >= self.generation {
                return Ok(());
            }
            let name = self
                .path
                .file_name()
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let tmp = self.path.with_file_name(format!(".{name}.tmp"));
            std::fs::write(&tmp, &self.text).map_err(|e| e.to_string())?;
            std::fs::rename(&tmp, &self.path).map_err(|e| e.to_string())?;
            *written = self.generation;
            *lock(&self.shared.seen) = stamp(&self.path);
            Ok(())
        })();
        self.shared.pending.fetch_sub(1, Ordering::SeqCst);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn files_normalize_and_serialize_like_lowdb() {
        let data = parse(r#"{"$schema":"x","a":[{"id":5},{"id":"s"}],"b":{"c":1}}"#)
            .unwrap()
            .unwrap();
        assert_eq!(data["a"][0]["id"], "5");
        assert!(!data.contains_key("$schema"));
        assert_eq!(
            serialize(&data),
            "{\n  \"a\": [\n    {\n      \"id\": \"5\"\n    },\n    {\n      \"id\": \"s\"\n    }\n  ],\n  \"b\": {\n    \"c\": 1\n  },\n  \"$schema\": \"./node_modules/json-server/schema.json\"\n}"
        );
        assert!(parse("[1]").is_err());
    }

    #[test]
    fn queued_writes_coalesce_and_never_reload_our_own() {
        let dir = std::env::temp_dir().join(format!("json-server-db-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("db.json");
        std::fs::write(&path, r#"{"a":[]}"#).unwrap();
        let (mut store, mut data) = Store::open(&path).unwrap();
        data.insert("b".into(), Value::Bool(true));
        let older = store.write(&data);
        data.insert("c".into(), Value::Bool(true));
        let newer = store.write(&data);
        assert!(store.changed().is_none());
        newer.finish().unwrap();
        older.finish().unwrap();
        let on_disk = parse(&std::fs::read_to_string(&path).unwrap())
            .unwrap()
            .unwrap();
        assert!(on_disk.contains_key("c"));
        assert!(store.changed().is_none());
        std::thread::sleep(std::time::Duration::from_millis(20));
        std::fs::write(&path, r#"{"z":[]}"#).unwrap();
        let reloaded = store.changed().unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert!(reloaded.contains_key("z"));
    }
}
