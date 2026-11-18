//! dot-prop 10.1.0 (MIT): `parsePath`, `getProperty` and `setProperty` on JSON values.

use std::borrow::Cow;

use serde_json::{Map, Value};

use crate::value::get_key;

const MAX_ARRAY_INDEX: u64 = 1_000_000;

/// One path segment: a property name, or an array index.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Seg {
    Key(String),
    Index(u64),
}

impl Seg {
    fn key(&self) -> Cow<'_, str> {
        match self {
            Seg::Key(k) => Cow::Borrowed(k),
            Seg::Index(i) => Cow::Owned(i.to_string()),
        }
    }
}

fn should_coerce(segment: &str) -> Option<u64> {
    if segment == "0" {
        return Some(0);
    }
    let bytes = segment.as_bytes();
    if bytes.first().is_some_and(|b| (b'1'..=b'9').contains(b))
        && bytes.iter().all(u8::is_ascii_digit)
    {
        let n: f64 = segment.parse().ok()?;
        if n <= 9007199254740991.0 && n <= MAX_ARRAY_INDEX as f64 {
            return Some(n as u64);
        }
    }
    None
}

fn disallowed(segment: &str) -> bool {
    matches!(segment, "__proto__" | "prototype" | "constructor")
}

/// Pushes a segment; `false` when it is a disallowed key (the whole path is then empty).
fn process(segment: &str, parts: &mut Vec<Seg>) -> bool {
    if disallowed(segment) {
        return false;
    }
    match should_coerce(segment) {
        Some(n) if !segment.is_empty() => parts.push(Seg::Index(n)),
        _ => parts.push(Seg::Key(segment.to_string())),
    }
    true
}

#[derive(PartialEq)]
enum Part {
    Start,
    Property,
    Index,
    IndexEnd,
}

/// `parsePath`: `Ok(vec![])` when the path names a disallowed key, `Err` with dot-prop's message.
pub fn parse_path(path: &str) -> Result<Vec<Seg>, String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut part = Part::Start;
    let mut escaping = false;
    for (n, c) in path.chars().enumerate() {
        let position = n + 1;
        if escaping {
            current.push(c);
            escaping = false;
            continue;
        }
        if c == '\\' {
            if part == Part::Index {
                return Err(format!(
                    "Invalid character '{c}' in an index at position {position}"
                ));
            }
            if part == Part::IndexEnd {
                return Err(format!(
                    "Invalid character '{c}' after an index at position {position}"
                ));
            }
            escaping = true;
            if part == Part::Start {
                part = Part::Property;
            }
            continue;
        }
        match c {
            '.' => {
                if part == Part::Index {
                    return Err(format!(
                        "Invalid character '{c}' in an index at position {position}"
                    ));
                }
                if part == Part::IndexEnd {
                    part = Part::Property;
                    continue;
                }
                if !process(&current, &mut parts) {
                    return Ok(Vec::new());
                }
                current.clear();
                part = Part::Property;
            }
            '[' => {
                if part == Part::Index {
                    return Err(format!(
                        "Invalid character '{c}' in an index at position {position}"
                    ));
                }
                if part == Part::IndexEnd {
                    part = Part::Index;
                    continue;
                }
                if part == Part::Property || part == Part::Start {
                    if (!current.is_empty() || part == Part::Property)
                        && !process(&current, &mut parts)
                    {
                        return Ok(Vec::new());
                    }
                    current.clear();
                }
                part = Part::Index;
            }
            ']' => {
                if part == Part::Index {
                    if current.is_empty() {
                        let last = match parts.pop() {
                            Some(seg) => seg.key().into_owned(),
                            None => String::new(),
                        };
                        current = last + "[]";
                        part = Part::Property;
                    } else {
                        let n: Option<f64> = current.parse().ok();
                        let valid = n.is_some_and(|n| {
                            n.is_finite()
                                && (0.0..=9007199254740991.0).contains(&n)
                                && n <= MAX_ARRAY_INDEX as f64
                                && current == (n as u64).to_string()
                        });
                        if valid {
                            parts.push(Seg::Index(n.unwrap_or(0.0) as u64));
                        } else {
                            parts.push(Seg::Key(current.clone()));
                        }
                        current.clear();
                        part = Part::IndexEnd;
                    }
                    continue;
                }
                if part == Part::IndexEnd {
                    return Err(format!(
                        "Invalid character '{c}' after an index at position {position}"
                    ));
                }
                current.push(c);
            }
            _ => {
                if part == Part::Index && !c.is_ascii_digit() {
                    return Err(format!(
                        "Invalid character '{c}' in an index at position {position}"
                    ));
                }
                if part == Part::IndexEnd {
                    return Err(format!(
                        "Invalid character '{c}' after an index at position {position}"
                    ));
                }
                if part == Part::Start {
                    part = Part::Property;
                }
                current.push(c);
            }
        }
    }
    if escaping {
        current.push('\\');
    }
    match part {
        Part::Property => {
            if !process(&current, &mut parts) {
                return Ok(Vec::new());
            }
        }
        Part::Index => return Err("Index was not closed".into()),
        Part::Start => parts.push(Seg::Key(String::new())),
        Part::IndexEnd => {}
    }
    Ok(parts)
}

/// `getProperty(object, path)` on an object: `Ok(None)` for `undefined`.
pub fn get_property<'a>(object: &'a Value, path: &str) -> Result<Option<Cow<'a, Value>>, String> {
    let parts = parse_path(path)?;
    if parts.is_empty() {
        return Ok(None);
    }
    let mut current: Cow<'a, Value> = Cow::Borrowed(object);
    for (i, seg) in parts.iter().enumerate() {
        let next = match current {
            Cow::Borrowed(v) => get_key(v, &seg.key()),
            Cow::Owned(ref v) => get_key(v, &seg.key()).map(|c| Cow::Owned(c.into_owned())),
        };
        match next {
            None => return Ok(None),
            Some(v) if v.is_null() => {
                return Ok(if i == parts.len() - 1 { Some(v) } else { None });
            }
            Some(v) => current = v,
        }
    }
    Ok(Some(current))
}

fn slot<'a>(container: &'a mut Value, seg: &Seg) -> Option<&'a mut Value> {
    match container {
        Value::Object(map) => Some(map.entry(seg.key().into_owned()).or_insert(Value::Null)),
        Value::Array(items) => {
            let i = match seg {
                Seg::Index(i) => *i as usize,
                Seg::Key(k) if crate::value::is_array_index(k) => k.parse().ok()?,
                Seg::Key(_) => return None,
            };
            if items.len() <= i {
                items.resize(i + 1, Value::Null);
            }
            items.get_mut(i)
        }
        _ => None,
    }
}

/// `setProperty(root, path, value)`. A named property set on an array is not part of its JSON,
/// so such writes are dropped.
pub fn set_property(root: &mut Map<String, Value>, path: &str, value: Value) -> Result<(), String> {
    let parts = parse_path(path)?;
    if parts.is_empty() {
        return Ok(());
    }
    let mut holder = Value::Object(std::mem::take(root));
    {
        let mut current = &mut holder;
        for (i, seg) in parts.iter().enumerate() {
            let last = i == parts.len() - 1;
            let next_is_index = matches!(parts.get(i + 1), Some(Seg::Index(_)));
            let Some(target) = slot(current, seg) else {
                break;
            };
            if last {
                *target = value;
                break;
            }
            if !matches!(target, Value::Object(_) | Value::Array(_)) {
                *target = if next_is_index {
                    Value::Array(Vec::new())
                } else {
                    Value::Object(Map::new())
                };
            }
            current = target;
        }
    }
    if let Value::Object(map) = holder {
        *root = map;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn paths_parse_as_dot_prop_does() {
        assert_eq!(
            parse_path("a.b[0].c").unwrap(),
            [
                Seg::Key("a".into()),
                Seg::Key("b".into()),
                Seg::Index(0),
                Seg::Key("c".into())
            ]
        );
        assert_eq!(
            parse_path("a[b]").unwrap_err(),
            "Invalid character 'b' in an index at position 3"
        );
        assert_eq!(parse_path("a[]").unwrap(), [Seg::Key("a[]".into())]);
        assert_eq!(parse_path("a\\.b").unwrap(), [Seg::Key("a.b".into())]);
        assert!(parse_path("__proto__.x").unwrap().is_empty());
        assert_eq!(parse_path("").unwrap(), [Seg::Key(String::new())]);
        assert_eq!(parse_path("a[1").unwrap_err(), "Index was not closed");
    }

    #[test]
    fn set_and_get_follow_dot_prop() {
        let mut root = Map::new();
        set_property(&mut root, "a.b.eq", json!(1)).unwrap();
        set_property(&mut root, "x[1].lt", json!(2)).unwrap();
        assert_eq!(
            Value::Object(root),
            json!({"a": {"b": {"eq": 1}}, "x": [null, {"lt": 2}]})
        );
        let item = json!({"t": "héllo", "tags": ["a", "b"], "n": null});
        assert_eq!(
            get_property(&item, "t.length").unwrap().unwrap().as_f64(),
            Some(5.0)
        );
        assert_eq!(
            get_property(&item, "tags.1").unwrap().unwrap().as_str(),
            Some("b")
        );
        assert_eq!(
            get_property(&item, "n").unwrap().as_deref(),
            Some(&Value::Null)
        );
        assert!(get_property(&item, "n.x").unwrap().is_none());
    }
}
