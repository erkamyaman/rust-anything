//! The Rust port of `src/service.ts` (json-server, MIT), and of `src/rust/service-calls.ts`, the
//! adapter that runs a list of Service calls so they can be recorded and compared.

#![forbid(unsafe_code)]

use serde_json::{Map, Value};

use self::service::{FindOpts, Found, Service};

/// The Service, as the app uses it.
pub mod service {
    use std::borrow::Cow;
    use std::cell::RefCell;
    use std::collections::HashMap;

    use json_server_js::value::{self, get_key, own_key, strict_eq, truthy};
    use json_server_js::{collate, dotprop, inflection, json};
    use serde_json::{Map, Value};

    /// The properties every JS object inherits from `Object.prototype`: `db.data[name]` finds
    /// them on any database.
    const PROTOTYPE_METHODS: [&str; 11] = [
        "constructor",
        "__defineGetter__",
        "__defineSetter__",
        "hasOwnProperty",
        "__lookupGetter__",
        "__lookupSetter__",
        "isPrototypeOf",
        "propertyIsEnumerable",
        "toString",
        "valueOf",
        "toLocaleString",
    ];

    /// A value as `===` tells values apart; objects and arrays equal nothing else, so they have
    /// none.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    enum Key {
        Undefined,
        Null,
        Bool(bool),
        Number(u64),
        Str(String),
    }

    impl Key {
        fn of(v: Option<&Value>) -> Option<Key> {
            Some(match v {
                None => Key::Undefined,
                Some(Value::Null) => Key::Null,
                Some(Value::Bool(b)) => Key::Bool(*b),
                Some(Value::Number(n)) => {
                    let x = n.as_f64()?;
                    if x.is_nan() {
                        return None;
                    }
                    Key::Number(if x == 0.0 { 0 } else { x.to_bits() })
                }
                Some(Value::String(s)) => Key::Str(s.clone()),
                Some(_) => return None,
            })
        }
    }

    enum Relation<'a> {
        Skip,
        Fail(String),
        Found {
            related: String,
            singular: bool,
            own: String,
            items: &'a [Value],
            index: HashMap<Key, Vec<usize>>,
        },
    }

    /// What `db.data[name]` gives.
    pub enum Lookup<'a> {
        Undefined,
        Value(&'a Value),
        /// `__proto__`: `Object.prototype`, an object with no own keys.
        Prototype,
        /// An inherited method.
        Function,
    }

    /// What a read answers.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Found {
        Undefined,
        Value(Value),
        /// `Object.prototype`: printed as `{}`.
        Prototype,
        /// A function: `res.json` sends no body.
        Function,
        /// A list already printed as `JSON.stringify(list, null, 2)` prints it.
        Json(String),
    }

    /// `randomBytes(8).toString('base64url')`.
    pub fn random_id() -> String {
        const ALPHABET: &[u8; 64] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";
        let mut bytes = [0u8; 8];
        if getrandom::fill(&mut bytes).is_err() {
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_nanos());
            bytes = (nanos as u64).to_le_bytes();
        }
        let mut bits: u128 = 0;
        for b in bytes {
            bits = (bits << 8) | u128::from(b);
        }
        bits <<= 2;
        (0..11)
            .map(|i| ALPHABET[((bits >> (60 - 6 * i)) & 63) as usize] as char)
            .collect()
    }

    /// The options of `find`, as `app.ts` passes them.
    #[derive(Debug, Clone, Default)]
    pub struct FindOpts {
        pub r#where: Value,
        pub sort: Option<String>,
        pub page: Option<f64>,
        pub per_page: Option<f64>,
        pub embed: Vec<String>,
    }

    /// `Service` over the database's data.
    pub struct Service {
        pub data: Map<String, Value>,
        /// The object PUT or PATCH on `__proto__` made the prototype of the data, if any: JS
        /// then finds its keys on every name the data does not have.
        pub prototype: Option<Value>,
    }

    fn not_a_function(method: &str) -> String {
        format!("relatedData.{method} is not a function")
    }

    impl Service {
        pub fn new(data: Map<String, Value>) -> Self {
            Service {
                data,
                prototype: None,
            }
        }

        /// `this.#db.data[name]`.
        pub fn get(&self, name: &str) -> Lookup<'_> {
            if let Some(v) = self.data.get(name) {
                return Lookup::Value(v);
            }
            if let Some(proto) = &self.prototype {
                if let Some(v) = proto.get(name) {
                    return Lookup::Value(v);
                }
                if name == "__proto__" {
                    return Lookup::Value(proto);
                }
            }
            if name == "__proto__" {
                Lookup::Prototype
            } else if PROTOTYPE_METHODS.contains(&name) {
                Lookup::Function
            } else {
                Lookup::Undefined
            }
        }

        /// What `embed(db, name, item, related)` looks up, worked out once for all the items:
        /// the related list indexed by the key `===` compares.
        fn relation(&self, name: &str, related: &str) -> Relation<'_> {
            let singular = inflection::singularize(related) == related;
            let lookup = if singular {
                self.get(&inflection::pluralize(related))
            } else {
                self.get(related)
            };
            let items = match lookup {
                Lookup::Undefined => return Relation::Skip,
                Lookup::Value(v) if !truthy(Some(v)) => return Relation::Skip,
                Lookup::Value(Value::Array(items)) => items,
                _ => {
                    return Relation::Fail(not_a_function(if singular {
                        "find"
                    } else {
                        "filter"
                    }));
                }
            };
            let mut index: HashMap<Key, Vec<usize>> = HashMap::new();
            let field = if singular {
                "id".to_string()
            } else {
                format!("{}Id", inflection::singularize(name))
            };
            for (i, r) in items.iter().enumerate() {
                if let Some(k) = Key::of(get_key(r, &field).as_deref()) {
                    let slot = index.entry(k).or_default();
                    if !singular || slot.is_empty() {
                        slot.push(i);
                    }
                }
            }
            Relation::Found {
                related: related.to_string(),
                singular,
                own: if singular {
                    format!("{related}Id")
                } else {
                    "id".to_string()
                },
                items,
                index,
            }
        }

        fn embed(relation: &Relation<'_>, item: &Value) -> Result<Value, String> {
            let Value::Object(fields) = item else {
                return Ok(item.clone());
            };
            let (related, singular, own, items, index) = match relation {
                Relation::Skip => return Ok(item.clone()),
                Relation::Fail(e) => return Err(e.clone()),
                Relation::Found {
                    related,
                    singular,
                    own,
                    items,
                    index,
                } => (related, *singular, own, items, index),
            };
            let hits = Key::of(own_key(fields, own))
                .and_then(|k| index.get(&k))
                .map_or(&[][..], Vec::as_slice);
            let mut out = fields.clone();
            if singular {
                match hits.first() {
                    Some(&i) => {
                        out.insert(related.clone(), items[i].clone());
                    }
                    None => {
                        out.shift_remove(related);
                    }
                }
            } else {
                let matched = hits.iter().map(|&i| items[i].clone()).collect();
                out.insert(related.clone(), Value::Array(matched));
            }
            Ok(Value::Object(out))
        }

        /// `findById(name, id, { _embed })`.
        pub fn find_by_id(&self, name: &str, id: &str, embed: &[String]) -> Result<Found, String> {
            let Lookup::Value(Value::Array(items)) = self.get(name) else {
                return Ok(Found::Undefined);
            };
            let id = Value::String(id.to_string());
            let Some(mut item) = items
                .iter()
                .find(|i| strict_eq(get_key(i, "id").as_deref(), Some(&id)))
                .cloned()
            else {
                return Ok(Found::Undefined);
            };
            for related in embed {
                item = Self::embed(&self.relation(name, related), &item)?;
            }
            Ok(Found::Value(item))
        }

        /// `find(name, opts)`.
        pub fn find(&self, name: &str, opts: &FindOpts) -> Result<Found, String> {
            let items = match self.get(name) {
                Lookup::Undefined => return Ok(Found::Undefined),
                Lookup::Prototype => return Ok(Found::Prototype),
                Lookup::Function => return Ok(Found::Function),
                Lookup::Value(Value::Array(items)) => items,
                Lookup::Value(other) => return Ok(Found::Value(other.clone())),
            };
            let mut results: Vec<Cow<'_, Value>> = items.iter().map(Cow::Borrowed).collect();
            for related in &opts.embed {
                let relation = self.relation(name, related);
                results = results
                    .iter()
                    .map(|item| Self::embed(&relation, item).map(Cow::Owned))
                    .collect::<Result<_, _>>()?;
            }
            let everything = matches!(&opts.r#where, Value::Object(m) if m.is_empty());
            let mut kept = Vec::with_capacity(results.len());
            for item in results {
                if everything || matches_where_rs::port::matches(Some(&item), Some(&opts.r#where))?
                {
                    kept.push(item);
                }
            }
            if let Some(sort) = opts.sort.as_deref().filter(|s| !s.is_empty()) {
                kept = sort_on(kept, sort)?;
            }
            if let Some(page) = opts.page {
                use paginate_rs::port::page::{page_of, result, slice_bounds};
                let total = kept.len() as f64;
                let p = page_of(total, page, opts.per_page.unwrap_or(10.0));
                let (s, e) = slice_bounds(kept.len(), p.start, p.end);
                let Value::Object(meta) = result(&p, total, Value::Null) else {
                    return Ok(Found::Undefined);
                };
                let mut out = String::from("{");
                for (key, value) in meta.iter().filter(|(k, _)| *k != "data") {
                    out.push_str("\n  ");
                    json::escape(key, &mut out);
                    out.push_str(": ");
                    json::write(value, 2, 1, &mut out);
                    out.push(',');
                }
                out.push_str("\n  \"data\": ");
                json::write_list(kept[s..e].iter().map(AsRef::as_ref), 2, 1, &mut out);
                out.push_str("\n}");
                return Ok(Found::Json(out));
            }
            let mut out = String::new();
            json::write_list(kept.iter().map(AsRef::as_ref), 2, 0, &mut out);
            Ok(Found::Json(out))
        }

        /// `create(name, data)`: `None` when `name` is not a list.
        pub fn create(&mut self, name: &str, body: Map<String, Value>) -> Option<Value> {
            let id = match self.get(name) {
                Lookup::Value(Value::Array(_)) => random_id(),
                _ => return None,
            };
            let mut item = body;
            item.insert("id".into(), Value::String(id));
            let item = Value::Object(item);
            if let Some(Value::Array(items)) = self.data.get_mut(name) {
                items.push(item.clone());
            }
            Some(item)
        }

        fn update_or_patch(
            &mut self,
            name: &str,
            body: Map<String, Value>,
            patch: bool,
        ) -> Option<Value> {
            let next = match self.get(name) {
                Lookup::Undefined | Lookup::Value(Value::Array(_)) => return None,
                Lookup::Value(Value::Object(item)) if patch => {
                    let mut next = item.clone();
                    next.extend(body);
                    next
                }
                Lookup::Value(Value::String(s)) if patch => {
                    let mut next: Map<String, Value> = s
                        .encode_utf16()
                        .enumerate()
                        .map(|(i, u)| {
                            (i.to_string(), Value::String(String::from_utf16_lossy(&[u])))
                        })
                        .collect();
                    next.extend(body);
                    next
                }
                _ => body,
            };
            if name == "__proto__" && !self.data.contains_key(name) {
                let next = Value::Object(next);
                self.prototype = Some(next.clone());
                return Some(next);
            }
            let next = Value::Object(next);
            self.data.insert(name.to_string(), next.clone());
            Some(next)
        }

        /// `update(name, body)`.
        pub fn update(&mut self, name: &str, body: Map<String, Value>) -> Option<Value> {
            self.update_or_patch(name, body, false)
        }

        /// `patch(name, body)`.
        pub fn patch(&mut self, name: &str, body: Map<String, Value>) -> Option<Value> {
            self.update_or_patch(name, body, true)
        }

        fn update_or_patch_by_id(
            &mut self,
            name: &str,
            id: &str,
            body: Map<String, Value>,
            patch: bool,
        ) -> Option<Value> {
            let Some(Value::Array(items)) = self.data.get_mut(name) else {
                return None;
            };
            let wanted = Value::String(id.to_string());
            let index = items
                .iter()
                .position(|i| strict_eq(get_key(i, "id").as_deref(), Some(&wanted)))?;
            let mut next = match (&items[index], patch) {
                (Value::Object(item), true) => item.clone(),
                _ => Map::new(),
            };
            next.extend(body);
            next.insert("id".into(), wanted);
            let next = Value::Object(next);
            items[index] = next.clone();
            Some(next)
        }

        /// `updateById(name, id, body)`.
        pub fn update_by_id(
            &mut self,
            name: &str,
            id: &str,
            body: Map<String, Value>,
        ) -> Option<Value> {
            self.update_or_patch_by_id(name, id, body, false)
        }

        /// `patchById(name, id, body)`.
        pub fn patch_by_id(
            &mut self,
            name: &str,
            id: &str,
            body: Map<String, Value>,
        ) -> Option<Value> {
            self.update_or_patch_by_id(name, id, body, true)
        }

        /// `destroyById(name, id, dependent)`.
        pub fn destroy_by_id(
            &mut self,
            name: &str,
            id: &str,
            dependent: &[String],
        ) -> Option<Value> {
            let wanted = Value::String(id.to_string());
            let Some(Value::Array(items)) = self.data.get_mut(name) else {
                return None;
            };
            let index = items
                .iter()
                .position(|i| strict_eq(get_key(i, "id").as_deref(), Some(&wanted)))?;
            let item = items.remove(index);
            let foreign_key = format!("{}Id", inflection::singularize(name));
            for (key, items) in self.data.iter_mut() {
                if key == name {
                    continue;
                }
                if let Value::Array(items) = items {
                    for item in items.iter_mut() {
                        if let Value::Object(fields) = item
                            && strict_eq(own_key(fields, &foreign_key), Some(&wanted))
                        {
                            fields.insert(foreign_key.clone(), Value::Null);
                        }
                    }
                }
            }
            for (key, items) in self.data.iter_mut() {
                if key == name || !dependent.contains(key) {
                    continue;
                }
                if let Value::Array(items) = items {
                    items.retain(|item| {
                        !matches!(get_key(item, &foreign_key).as_deref(), Some(Value::Null))
                    });
                }
            }
            Some(item)
        }
    }

    fn falsy_but_not_zero(v: Option<&Value>) -> bool {
        !truthy(v) && !matches!(v, Some(Value::Number(n)) if n.as_f64() == Some(0.0))
    }

    /// sort-on 7.0.0 (MIT): `sortOn(items, fields)`, with `-field` for descending, compared as
    /// JS compares (ICU collation for two strings). A path dot-prop rejects throws, but only once
    /// the sort compares two items.
    pub fn sort_on<'a>(
        items: Vec<Cow<'a, Value>>,
        sort: &str,
    ) -> Result<Vec<Cow<'a, Value>>, String> {
        let fields: Vec<&str> = sort.split(',').collect();
        let error: RefCell<Option<String>> = RefCell::new(None);
        let mut items = items;
        crate::js::sort(&mut items, |a, b| {
            if error.borrow().is_some() {
                return 0.0;
            }
            let mut out = 0.0;
            for field in &fields {
                let descending = field.starts_with('-');
                let path = if descending { &field[1..] } else { field };
                let (x, y) = match (
                    dotprop::get_property(a, path),
                    dotprop::get_property(b, path),
                ) {
                    (Ok(x), Ok(y)) => (x, y),
                    (Err(e), _) | (_, Err(e)) => {
                        *error.borrow_mut() = Some(e);
                        return 0.0;
                    }
                };
                let (x, y) = (x.as_deref(), y.as_deref());
                if strict_eq(x, y) {
                    out = 0.0;
                    continue;
                }
                if falsy_but_not_zero(y) {
                    return if descending { 1.0 } else { -1.0 };
                }
                if falsy_but_not_zero(x) {
                    return if descending { -1.0 } else { 1.0 };
                }
                if let (Some(Value::String(x)), Some(Value::String(y))) = (x, y) {
                    out = if descending {
                        collate::locale_compare(y, x)
                    } else {
                        collate::locale_compare(x, y)
                    };
                    if out != 0.0 {
                        return out;
                    }
                    continue;
                }
                let less = value::lt(x, y);
                return match (descending, less) {
                    (true, true) | (false, false) => 1.0,
                    _ => -1.0,
                };
            }
            out
        });
        match error.into_inner() {
            Some(e) => Err(e),
            None => Ok(items),
        }
    }
}

fn text(call: &Map<String, Value>, key: &str) -> String {
    json_server_js::value::to_js_string(call.get(key))
}

fn strings(v: Option<&Value>) -> Vec<String> {
    match v {
        None => Vec::new(),
        Some(Value::Array(items)) => items
            .iter()
            .map(|i| json_server_js::value::to_js_string(Some(i)))
            .collect(),
        Some(other) => vec![json_server_js::value::to_js_string(Some(other))],
    }
}

fn body(call: &Map<String, Value>) -> Map<String, Value> {
    match call.get("body") {
        Some(Value::Object(m)) => m.clone(),
        _ => Map::new(),
    }
}

fn found(f: Found) -> Value {
    match f {
        Found::Undefined => serde_json::json!({ "undefined": true }),
        Found::Value(v) => v,
        Found::Prototype => Value::Object(Map::new()),
        Found::Function => Value::Null,
        Found::Json(text) => json_server_js::json::parse(&text).unwrap_or(Value::Null),
    }
}

fn written(v: Option<Value>) -> Value {
    v.unwrap_or_else(|| serde_json::json!({ "undefined": true }))
}

fn run(service: &mut Service, call: &Map<String, Value>) -> Result<Value, String> {
    let name = text(call, "name");
    let id = match call.get("id") {
        None | Some(Value::Null) => String::new(),
        some => json_server_js::value::to_js_string(some),
    };
    let page = call.get("page").map(|p| p.as_f64().unwrap_or(f64::NAN));
    let per_page = match call.get("perPage") {
        None | Some(Value::Null) => None,
        Some(p) => Some(p.as_f64().unwrap_or(f64::NAN)),
    };
    if call.get("op").is_none() {
        return Ok(Value::Object(Map::new()));
    }
    Ok(match text(call, "op").as_str() {
        "find" => found(
            service.find(
                &name,
                &FindOpts {
                    r#where: call
                        .get("where")
                        .cloned()
                        .unwrap_or_else(|| Value::Object(Map::new())),
                    sort: call
                        .get("sort")
                        .map(|s| json_server_js::value::to_js_string(Some(s))),
                    page,
                    per_page,
                    embed: strings(call.get("embed")),
                },
            )?,
        ),
        "findById" => found(service.find_by_id(&name, &id, &strings(call.get("embed")))?),
        "create" => written(service.create(&name, body(call))),
        "update" => written(service.update(&name, body(call))),
        "patch" => written(service.patch(&name, body(call))),
        "updateById" => written(service.update_by_id(&name, &id, body(call))),
        "patchById" => written(service.patch_by_id(&name, &id, body(call))),
        "destroyById" => {
            written(service.destroy_by_id(&name, &id, &strings(call.get("dependent"))))
        }
        other => serde_json::json!({ "unknownOp": other }),
    })
}

fn is_known(op: Option<&Value>) -> bool {
    matches!(
        op.and_then(Value::as_str),
        Some(
            "find"
                | "findById"
                | "create"
                | "update"
                | "patch"
                | "updateById"
                | "patchById"
                | "destroyById"
        )
    )
}

/// Port of `serviceCalls` (`src/rust/service-calls.ts:22`). `calls` is iterated as `for...of`
/// does: an array's items, a string's code points, a TypeError for anything else.
pub async fn service_calls(
    data: serde_json::Value,
    calls: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let calls: Vec<Value> = match calls {
        Value::Array(items) => items,
        Value::String(s) => s.chars().map(|c| Value::String(c.to_string())).collect(),
        _ => return Err("calls is not iterable".into()),
    };
    let (map, other_root) = match data {
        Value::Object(map) => (map, None),
        other => (Map::new(), Some(other)),
    };
    let mut service = Service::new(map);
    let mut results = Vec::with_capacity(calls.len());
    let mut made: Vec<String> = Vec::new();
    for call in &calls {
        let outcome = match call {
            Value::Null => Err("Cannot read properties of null (reading 'op')".to_string()),
            Value::Object(call) if other_root.is_some() && is_known(call.get("op")) => {
                Err("serviceCalls: the data is not an object of collections".to_string())
            }
            Value::Object(call) => run(&mut service, call),
            _ => Ok(Value::Object(Map::new())),
        };
        if let (Ok(Value::Object(item)), Some(Value::String(op))) = (&outcome, call.get("op"))
            && op == "create"
            && let Some(Value::String(id)) = item.get("id")
        {
            made.push(id.clone());
        }
        results.push(outcome.unwrap_or_else(|e| serde_json::json!({ "threw": e })));
    }
    let data = other_root.unwrap_or(Value::Object(service.data));
    let out = serde_json::json!({ "results": results, "data": data });
    if made.is_empty() {
        return Ok(out);
    }
    let mut text = serde_json::to_string(&out).map_err(|e| e.to_string())?;
    for (i, id) in made.iter().enumerate() {
        text = text.replace(&format!("\"{id}\""), &format!("\"#{}\"", i + 1));
    }
    serde_json::from_str(&text).map_err(|e| e.to_string())
}
