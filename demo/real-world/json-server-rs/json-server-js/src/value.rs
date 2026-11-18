//! JavaScript operators on JSON values. `None` stands for `undefined`.

use std::cmp::Ordering;

use serde_json::{Map, Value};

use crate::number::{number_to_string, string_to_number};

/// A JS number as a JSON value (non-finite numbers cannot be stored and become `null`, as
/// `JSON.stringify` prints them; `-0` is stored as `0`, which no JSON output tells apart).
pub fn num(n: f64) -> Value {
    let n = if n == 0.0 { 0.0 } else { n };
    serde_json::Number::from_f64(n).map_or(Value::Null, Value::Number)
}

/// A key JS treats as an array index, enumerated first and in numeric order.
pub fn is_array_index(key: &str) -> bool {
    if !key.as_bytes().first().is_some_and(u8::is_ascii_digit) {
        return false;
    }
    if key == "0" {
        return true;
    }
    !key.is_empty()
        && key.len() <= 10
        && !key.starts_with('0')
        && key.bytes().all(|b| b.is_ascii_digit())
        && key.parse::<u64>().is_ok_and(|n| n < u32::MAX as u64)
}

/// The order `Object.keys` gives: array-index keys ascending, then the others as inserted.
pub fn ordered_keys(map: &Map<String, Value>) -> Vec<&String> {
    let mut index: Vec<&String> = map.keys().filter(|k| is_array_index(k)).collect();
    if index.is_empty() {
        return map.keys().collect();
    }
    index.sort_by_key(|k| k.parse::<u64>().unwrap_or(0));
    index.extend(map.keys().filter(|k| !is_array_index(k)));
    index
}

/// JS truthiness.
pub fn truthy(v: Option<&Value>) -> bool {
    match v {
        None | Some(Value::Null) => false,
        Some(Value::Bool(b)) => *b,
        Some(Value::Number(n)) => n.as_f64().is_some_and(|x| x != 0.0 && !x.is_nan()),
        Some(Value::String(s)) => !s.is_empty(),
        Some(Value::Array(_) | Value::Object(_)) => true,
    }
}

/// JS `===`. Two objects or arrays are never the same value here: they always come from
/// different places.
pub fn strict_eq(a: Option<&Value>, b: Option<&Value>) -> bool {
    match (a, b) {
        (None, None) => true,
        (Some(Value::Null), Some(Value::Null)) => true,
        (Some(Value::Bool(x)), Some(Value::Bool(y))) => x == y,
        (Some(Value::Number(x)), Some(Value::Number(y))) => x.as_f64() == y.as_f64(),
        (Some(Value::String(x)), Some(Value::String(y))) => x == y,
        _ => false,
    }
}

/// JS `String(value)`.
pub fn to_js_string(v: Option<&Value>) -> String {
    match v {
        None => "undefined".into(),
        Some(Value::Null) => "null".into(),
        Some(Value::Array(items)) => items
            .iter()
            .map(|item| match item {
                Value::Null => String::new(),
                other => to_js_string(Some(other)),
            })
            .collect::<Vec<_>>()
            .join(","),
        Some(other) => primitive_string(other),
    }
}

fn primitive_string(v: &Value) -> String {
    match v {
        Value::Null => "null".into(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => number_to_string(n.as_f64().unwrap_or(f64::NAN)),
        Value::String(s) => s.clone(),
        Value::Array(_) => to_js_string(Some(v)),
        Value::Object(_) => "[object Object]".into(),
    }
}

enum Prim<'a> {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    Str(std::borrow::Cow<'a, str>),
}

fn to_primitive(v: Option<&Value>) -> Prim<'_> {
    match v {
        None => Prim::Undefined,
        Some(Value::Null) => Prim::Null,
        Some(Value::Bool(b)) => Prim::Bool(*b),
        Some(Value::Number(n)) => Prim::Number(n.as_f64().unwrap_or(f64::NAN)),
        Some(Value::String(s)) => Prim::Str(s.as_str().into()),
        Some(other) => Prim::Str(to_js_string(Some(other)).into()),
    }
}

fn prim_number(p: &Prim) -> f64 {
    match p {
        Prim::Undefined => f64::NAN,
        Prim::Null => 0.0,
        Prim::Bool(b) => f64::from(u8::from(*b)),
        Prim::Number(n) => *n,
        Prim::Str(s) => string_to_number(s),
    }
}

/// JS `Number(value)`.
pub fn to_number(v: Option<&Value>) -> f64 {
    prim_number(&to_primitive(v))
}

/// Strings compare by UTF-16 code units in JS.
pub fn utf16_cmp(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// JS abstract relational comparison `a < b`: `None` when the answer is `undefined` (a NaN).
fn less_than(a: Option<&Value>, b: Option<&Value>) -> Option<bool> {
    let (pa, pb) = (to_primitive(a), to_primitive(b));
    if let (Prim::Str(x), Prim::Str(y)) = (&pa, &pb) {
        return Some(utf16_cmp(x, y) == Ordering::Less);
    }
    let (x, y) = (prim_number(&pa), prim_number(&pb));
    if x.is_nan() || y.is_nan() {
        return None;
    }
    Some(x < y)
}

/// JS `a < b`.
pub fn lt(a: Option<&Value>, b: Option<&Value>) -> bool {
    less_than(a, b) == Some(true)
}

/// JS `a > b`.
pub fn gt(a: Option<&Value>, b: Option<&Value>) -> bool {
    less_than(b, a) == Some(true)
}

/// JS `a <= b`.
pub fn le(a: Option<&Value>, b: Option<&Value>) -> bool {
    less_than(b, a) == Some(false)
}

/// JS `a >= b`.
pub fn ge(a: Option<&Value>, b: Option<&Value>) -> bool {
    less_than(a, b) == Some(false)
}

/// JS `typeof v === 'object' && v !== null && !Array.isArray(v)`.
pub fn is_plain_object(v: Option<&Value>) -> bool {
    matches!(v, Some(Value::Object(_)))
}

/// `map[key]`: a linear scan for the small objects records are, which beats hashing the key.
pub fn own_key<'a>(map: &'a Map<String, Value>, key: &str) -> Option<&'a Value> {
    if map.len() <= 16 {
        map.iter().find(|(k, _)| k.as_str() == key).map(|(_, v)| v)
    } else {
        map.get(key)
    }
}

/// What `obj[key]` reads on a JSON value, for the properties JSON values have: object keys,
/// array and string indexes, and `length`. Inherited methods read as `undefined`.
pub fn get_key<'a>(v: &'a Value, key: &str) -> Option<std::borrow::Cow<'a, Value>> {
    use std::borrow::Cow;
    match v {
        Value::Object(map) => own_key(map, key).map(Cow::Borrowed),
        Value::Array(items) => {
            if key == "length" {
                return Some(Cow::Owned(num(items.len() as f64)));
            }
            if is_array_index(key) {
                return items.get(key.parse::<usize>().ok()?).map(Cow::Borrowed);
            }
            None
        }
        Value::String(s) => {
            if key == "length" {
                return Some(Cow::Owned(num(s.encode_utf16().count() as f64)));
            }
            if is_array_index(key) {
                let i = key.parse::<usize>().ok()?;
                let unit = s.encode_utf16().nth(i)?;
                return Some(Cow::Owned(Value::String(String::from_utf16_lossy(&[unit]))));
            }
            None
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn comparisons_follow_javascript() {
        let n = |x: f64| num(x);
        let s = |x: &str| Value::String(x.into());
        assert!(lt(Some(&n(1.0)), Some(&s("2"))));
        assert!(lt(Some(&s("10")), Some(&s("9"))));
        assert!(!lt(None, Some(&n(1.0))));
        assert!(!ge(None, Some(&n(1.0))));
        assert!(le(Some(&Value::Null), Some(&n(0.0))));
        assert!(ge(Some(&json!([2])), Some(&n(2.0))));
        assert!(lt(Some(&json!({})), Some(&s("[object Z"))));
        assert!(strict_eq(Some(&n(1.0)), Some(&json!(1))));
        assert!(!strict_eq(Some(&json!({})), Some(&json!({}))));
        assert_eq!(
            to_js_string(Some(&json!([1, null, [2, 3], {}]))),
            "1,,2,3,[object Object]"
        );
        assert!(is_array_index("10") && !is_array_index("01") && !is_array_index("4294967295"));
        let m = json!({"b": 1, "10": 2, "2": 3, "a": 4});
        let keys: Vec<&String> = ordered_keys(m.as_object().unwrap());
        assert_eq!(keys, ["2", "10", "b", "a"]);
    }
}
