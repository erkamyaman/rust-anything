//! The Rust port of `src/matches-where.ts` (json-server, MIT): whether an item matches a where
//! object, with JavaScript's comparison rules.

#![forbid(unsafe_code)]

use std::borrow::Cow;

use json_server_js::value::{self, get_key, is_array_index, ordered_keys};
use serde_json::Value;

use crate::js;

const OPERATORS: [&str; 10] = [
    "lt",
    "lte",
    "gt",
    "gte",
    "eq",
    "ne",
    "in",
    "contains",
    "startsWith",
    "endsWith",
];

fn defined(v: &Value) -> Option<&Value> {
    (!js::is_undefined(v)).then_some(v)
}

fn is_json_object(v: Option<&Value>) -> bool {
    matches!(v, Some(Value::Object(_)))
}

/// `Object.entries(where)`, or the TypeError JS throws for `null` and `undefined`.
type Entries<'a> = Vec<(Cow<'a, str>, Cow<'a, Value>)>;

type StringTest = fn(&str, &str) -> bool;

fn entries(r#where: Option<&Value>) -> Result<Entries<'_>, String> {
    Ok(match r#where {
        None | Some(Value::Null) => {
            return Err("Cannot convert undefined or null to object".into());
        }
        Some(Value::Object(map)) if !map.keys().any(|k| is_array_index(k)) => map
            .iter()
            .map(|(k, v)| (Cow::Borrowed(k.as_str()), Cow::Borrowed(v)))
            .collect(),
        Some(Value::Object(map)) => ordered_keys(map)
            .into_iter()
            .map(|k| (Cow::Borrowed(k.as_str()), Cow::Borrowed(&map[k.as_str()])))
            .collect(),
        Some(Value::Array(items)) => items
            .iter()
            .enumerate()
            .map(|(i, v)| (Cow::Owned(i.to_string()), Cow::Borrowed(v)))
            .collect(),
        Some(Value::String(s)) => s
            .encode_utf16()
            .enumerate()
            .map(|(i, u)| {
                let unit = Value::String(String::from_utf16_lossy(&[u]));
                (Cow::Owned(i.to_string()), Cow::Owned(unit))
            })
            .collect(),
        Some(_) => Vec::new(),
    })
}

/// `obj[key]`, or the TypeError JS throws when `obj` is `null` or `undefined`.
fn field<'a>(obj: Option<&'a Value>, key: &str) -> Result<Option<Cow<'a, Value>>, String> {
    match obj {
        None => Err(format!(
            "Cannot read properties of undefined (reading '{key}')"
        )),
        Some(Value::Null) => Err(format!("Cannot read properties of null (reading '{key}')")),
        Some(v) => Ok(get_key(v, key).filter(|f| !js::is_undefined(f))),
    }
}

/// `matchesWhere(obj, where)` on values that may be `undefined` (`None`).
pub fn matches(obj: Option<&Value>, r#where: Option<&Value>) -> Result<bool, String> {
    for (key, value) in entries(r#where)? {
        let value = defined(&value);
        if key == "or" {
            let Some(Value::Array(subs)) = value else {
                return Ok(false);
            };
            if subs.is_empty() {
                return Ok(false);
            }
            let mut matched = false;
            for sub in subs {
                let sub = defined(sub);
                if is_json_object(sub) && matches(obj, sub)? {
                    matched = true;
                    break;
                }
            }
            if !matched {
                return Ok(false);
            }
            continue;
        }
        let field = field(obj, &key)?;
        let field = field.as_deref();
        if let Some(Value::Object(ops)) = value {
            let known: Vec<&str> = OPERATORS
                .iter()
                .copied()
                .filter(|op| ops.contains_key(*op))
                .collect();
            if !known.is_empty() {
                if field.is_none() {
                    return Ok(false);
                }
                let op = |name: &str| ops.get(name).and_then(defined);
                let has = |name: &str| known.contains(&name);
                if has("lt") && !value::lt(field, op("lt")) {
                    return Ok(false);
                }
                if has("lte") && !value::le(field, op("lte")) {
                    return Ok(false);
                }
                if has("gt") && !value::gt(field, op("gt")) {
                    return Ok(false);
                }
                if has("gte") && !value::ge(field, op("gte")) {
                    return Ok(false);
                }
                if has("eq") && !value::strict_eq(field, op("eq")) {
                    return Ok(false);
                }
                if has("ne") && value::strict_eq(field, op("ne")) {
                    return Ok(false);
                }
                if has("in") {
                    let any = match op("in") {
                        Some(Value::Array(values)) => {
                            values.iter().any(|v| value::strict_eq(field, defined(v)))
                        }
                        other => value::strict_eq(field, other),
                    };
                    if !any {
                        return Ok(false);
                    }
                }
                let tests: [(&str, StringTest); 3] = [
                    ("contains", |a, b| a.contains(b)),
                    ("startsWith", |a, b| a.starts_with(b)),
                    ("endsWith", |a, b| a.ends_with(b)),
                ];
                for (name, test) in tests {
                    if has(name) {
                        let Some(Value::String(text)) = field else {
                            return Ok(false);
                        };
                        let needle = value::to_js_string(op(name)).to_lowercase();
                        if !test(&text.to_lowercase(), &needle) {
                            return Ok(false);
                        }
                    }
                }
                continue;
            }
            if is_json_object(field) && !matches(field, value)? {
                return Ok(false);
            }
            continue;
        }
        return Ok(false);
    }
    Ok(true)
}

/// Port of `matchesWhere` (`src/matches-where.ts:24`).
pub fn matches_where(obj: serde_json::Value, r#where: serde_json::Value) -> Result<bool, String> {
    matches(defined(&obj), defined(&r#where))
}
