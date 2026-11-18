//! The Rust port of `src/parse-where.ts` (json-server, MIT): a query string such as
//! `views:gt=100&title:contains=json` becomes a where object, `{"views":{"gt":100},...}`.

#![forbid(unsafe_code)]

use json_server_js::{dotprop, number, url};
use serde_json::{Map, Value};

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

fn operator(op: &str) -> Option<&'static str> {
    OPERATORS.iter().copied().find(|o| *o == op)
}

fn is_line_terminator(c: char) -> bool {
    matches!(c, '\n' | '\r' | '\u{2028}' | '\u{2029}')
}

/// `splitKey`: the path and the operator; `None` for an unknown operator after a colon.
fn split_key(key: &str) -> (&str, Option<&'static str>) {
    if let Some(colon) = key.rfind(':') {
        let (path, op) = (&key[..colon], &key[colon + 1..]);
        if op.is_empty() {
            return (key, Some("eq"));
        }
        return (path, operator(op));
    }
    // `/^(.*)_([a-z]+)$/`: `.` stops at line terminators, so the path cannot hold one.
    if let Some(underscore) = key.rfind('_') {
        let (path, op) = (&key[..underscore], &key[underscore + 1..]);
        let matched = !op.is_empty()
            && op.bytes().all(|b| b.is_ascii_lowercase())
            && !path.contains(is_line_terminator);
        if matched
            && !path.is_empty()
            && let Some(op) = operator(op)
        {
            return (path, Some(op));
        }
    }
    (key, Some("eq"))
}

/// `coerceValue`: `true`, `false`, `null`, finite numbers, or the string itself.
pub fn coerce_value(value: &str) -> Value {
    match value {
        "true" => return Value::Bool(true),
        "false" => return Value::Bool(false),
        "null" => return Value::Null,
        _ => {}
    }
    if number::trim(value).is_empty() {
        return Value::String(value.to_string());
    }
    let n = number::string_to_number(value);
    if n.is_finite() {
        return js::number(n);
    }
    Value::String(value.to_string())
}

/// `parseWhere` into an existing object, for callers that already hold the pairs.
pub fn parse_where_pairs(pairs: &[(String, String)]) -> Result<Map<String, Value>, String> {
    let mut out = Map::new();
    for (raw_key, raw_value) in pairs {
        let (path, Some(op)) = split_key(raw_key) else {
            continue;
        };
        let full = format!("{path}.{op}");
        let value = if op == "in" {
            Value::Array(
                raw_value
                    .split(',')
                    .map(|part| coerce_value(number::trim(part)))
                    .collect(),
            )
        } else {
            coerce_value(raw_value)
        };
        dotprop::set_property(&mut out, &full, value)?;
    }
    Ok(out)
}

/// Port of `parseWhere` (`src/parse-where.ts:58`).
pub fn parse_where(query: String) -> Result<serde_json::Value, String> {
    parse_where_pairs(&url::search_params(&query)).map(Value::Object)
}
