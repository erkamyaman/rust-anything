//! Where a mismatch starts, which mismatches are the same difference, and mismatches without
//! their values.
//!
//! `locate_value` finds the first place a unit's expected and actual results part (the channel,
//! the JSON path, the kind of difference and, for text, the character); `group_mismatches` and
//! `replay_groups` count the cases that differ the same way under a short stable id; the
//! `redact_*` functions replace arguments and outputs with types, lengths and hashes.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::app::{Failure, ReplayOutcome};
use crate::harness::DiffOutcome;

/// Characters of context kept on each side of the first difference.
const CONTEXT: usize = 16;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct At {
    /// `return`, `throw`, `return vs throw`, `call N`, or a replay field (`status`, `body`, ...).
    pub channel: String,
    /// JSON path of the first difference, `$` for the whole value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// `value`, `type`, `missing key`, `extra key`, `length`, `text`, `lines added`,
    /// `lines missing`, `trailing newline`, `binary`, `no response`, or `<expected>→<actual>` for
    /// a status.
    pub kind: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: Option<Offset>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_len: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub actual_len: Option<usize>,
    /// A few characters around the difference on each side, the differing one in `⟨⟩`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub context: Option<Context>,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offset {
    pub char: usize,
    pub byte: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Context {
    pub expected: String,
    pub actual: String,
}

impl At {
    pub fn new(kind: &str) -> At {
        At {
            kind: kind.to_string(),
            ..At::default()
        }
    }

    /// `return $.rest (value)`: where and what kind.
    pub fn label(&self) -> String {
        let mut out = self.channel.clone();
        if let Some(path) = &self.path {
            out += &format!(" {path}");
        }
        format!("{out} ({})", self.kind)
    }

    /// `at return $.rest, char 3: expected "…ab⟨c⟩…", got "…ab⟨d⟩…"`, or the label and the
    /// shape when there is no context.
    pub fn describe(&self) -> String {
        let place = match &self.path {
            Some(path) => format!("{} {path}", self.channel),
            None => self.channel.clone(),
        };
        match (&self.context, &self.offset) {
            (Some(c), Some(o)) => format!(
                "at {place}{}, char {}: expected {:?}, got {:?}",
                self.line
                    .zip(self.column)
                    .filter(|(l, _)| *l > 1 || self.path.is_none())
                    .map(|(l, c)| format!(", line {l}, column {c}"))
                    .unwrap_or_default(),
                o.char,
                c.expected,
                c.actual
            ),
            (Some(c), None) => format!(
                "at {place} ({}): expected {}, got {}",
                self.kind, c.expected, c.actual
            ),
            _ => format!("at {}", self.shape()),
        }
    }

    /// Where and what kind, with line, column, types and lengths: no values.
    pub fn shape(&self) -> String {
        let mut out = self.label();
        if let Some(line) = self.line {
            out += &format!(", line {line}");
        }
        if let Some(column) = self.column {
            out += &format!(", column {column}");
        }
        if let Some(o) = &self.offset {
            out += &format!(", char {}", o.char);
        }
        if let (Some(e), Some(a)) = (&self.expected_type, &self.actual_type)
            && e != a
        {
            out += &format!(", expected {e}, got {a}");
        }
        if let (Some(e), Some(a)) = (self.expected_len, self.actual_len) {
            out += &format!(", length {e} expected, {a} got");
        }
        out
    }
}

fn type_name(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn key_path(path: &str, key: &str) -> String {
    let plain = !key.is_empty()
        && !key.starts_with(|c: char| c.is_ascii_digit())
        && key
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '$');
    if plain {
        format!("{path}.{key}")
    } else {
        format!("{path}[{}]", Value::String(key.to_string()))
    }
}

/// The char and byte index where two texts first part.
fn first_difference(expected: &str, actual: &str) -> (usize, usize) {
    let mut a = actual.chars();
    for (i, (byte, e)) in expected.char_indices().enumerate() {
        if a.next() != Some(e) {
            return (i, byte);
        }
    }
    (expected.chars().count(), expected.len())
}

fn around(text: &str, at: usize) -> String {
    let chars: Vec<char> = text.chars().collect();
    let start = at.saturating_sub(CONTEXT);
    let end = (at + 1 + CONTEXT).min(chars.len());
    let mut out = String::new();
    if start > 0 {
        out.push('…');
    }
    out.extend(&chars[start..at.min(chars.len())]);
    out.push('⟨');
    if let Some(c) = chars.get(at) {
        out.push(*c);
    }
    out.push('⟩');
    if at < chars.len() {
        out.extend(&chars[at + 1..end]);
    }
    if end < chars.len() {
        out.push('…');
    }
    out
}

/// The first difference between two texts, as a `kind` at a character.
pub(crate) fn text_at(kind: &str, expected: &str, actual: &str) -> At {
    let (char, byte) = first_difference(expected, actual);
    let before = &expected[..byte];
    let line = before.matches('\n').count() + 1;
    let column = before
        .rsplit_once('\n')
        .map_or(before, |(_, l)| l)
        .chars()
        .count()
        + 1;
    At {
        kind: kind.to_string(),
        line: Some(line),
        column: Some(column),
        offset: Some(Offset { char, byte }),
        expected_len: Some(expected.chars().count()),
        actual_len: Some(actual.chars().count()),
        context: Some(Context {
            expected: around(expected, char),
            actual: around(actual, char),
        }),
        ..At::default()
    }
}

fn strings_match(expected: &str, actual: &str) -> bool {
    expected == actual || (expected.contains("[..]") && crate::app::line_matches(expected, actual))
}

fn walk(expected: &Value, actual: &Value, path: &str) -> Option<At> {
    let at = |kind: &str| At {
        path: Some(path.to_string()),
        kind: kind.to_string(),
        expected_type: Some(type_name(expected).to_string()),
        actual_type: Some(type_name(actual).to_string()),
        ..At::default()
    };
    match (expected, actual) {
        (Value::String(e), Value::String(a)) => {
            if strings_match(e, a) {
                return None;
            }
            Some(At {
                path: Some(path.to_string()),
                expected_type: Some("string".into()),
                actual_type: Some("string".into()),
                ..text_at("value", e, a)
            })
        }
        _ if expected == actual => None,
        (Value::Object(e), Value::Object(a)) => {
            for (k, ev) in e {
                match a.get(k) {
                    None => {
                        return Some(At {
                            path: Some(key_path(path, k)),
                            kind: "missing key".into(),
                            expected_type: Some(type_name(ev).into()),
                            ..At::default()
                        });
                    }
                    Some(av) => {
                        if let Some(found) = walk(ev, av, &key_path(path, k)) {
                            return Some(found);
                        }
                    }
                }
            }
            if let Some((k, av)) = a.iter().find(|(k, _)| !e.contains_key(*k)) {
                return Some(At {
                    path: Some(key_path(path, k)),
                    kind: "extra key".into(),
                    actual_type: Some(type_name(av).into()),
                    ..At::default()
                });
            }
            Some(at("value"))
        }
        (Value::Array(e), Value::Array(a)) => {
            for (i, (ev, av)) in e.iter().zip(a).enumerate() {
                if let Some(found) = walk(ev, av, &format!("{path}[{i}]")) {
                    return Some(found);
                }
            }
            if e.len() != a.len() {
                return Some(At {
                    expected_len: Some(e.len()),
                    actual_len: Some(a.len()),
                    ..at("length")
                });
            }
            Some(at("value"))
        }
        _ if type_name(expected) != type_name(actual) => Some(at("type")),
        _ => Some(At {
            context: Some(Context {
                expected: expected.to_string(),
                actual: actual.to_string(),
            }),
            ..at("value")
        }),
    }
}

/// The first difference inside two JSON values, rooted at `$`, with no channel.
pub fn locate_in(expected: &Value, actual: &Value) -> At {
    walk(expected, actual, "$").unwrap_or_else(|| At {
        path: Some("$".into()),
        ..At::new("value")
    })
}

fn is_error(step: &Value) -> bool {
    step.get("error").is_some()
}

/// The first difference between a unit's recorded result and the port's: `{ok}` and `{error}`
/// results, or a class scenario's list of steps.
pub fn locate_value(expected: &Value, actual: &Value) -> At {
    if let (Value::Array(e), Value::Array(a)) = (expected, actual) {
        let differs = |i: &usize| e[*i] != a[*i];
        let n = e.len().min(a.len());
        let step = (0..n)
            .find(|i| differs(i) && !(is_error(&e[*i]) && is_error(&a[*i])))
            .or_else(|| (0..n).find(differs));
        return match step {
            Some(i) => At {
                channel: format!("call {i}"),
                ..locate_value(&e[i], &a[i])
            },
            None => At {
                channel: format!("call {n}"),
                expected_len: Some(e.len()),
                actual_len: Some(a.len()),
                ..At::new("length")
            },
        };
    }
    let (e_ok, a_ok) = (expected.get("ok"), actual.get("ok"));
    let (e_err, a_err) = (expected.get("error"), actual.get("error"));
    match (e_ok, a_ok, e_err, a_err) {
        (Some(e), Some(a), _, _) => At {
            channel: "return".into(),
            ..locate_in(e, a)
        },
        (_, _, Some(e), Some(a)) => At {
            channel: "throw".into(),
            ..locate_in(e, a)
        },
        (Some(_), None, _, Some(_)) | (None, Some(_), Some(_), _) => At {
            channel: "return vs throw".into(),
            expected_type: Some(if e_ok.is_some() { "return" } else { "throw" }.into()),
            actual_type: Some(if a_ok.is_some() { "return" } else { "throw" }.into()),
            ..At::new("type")
        },
        _ if actual.get("missing").is_some() => At {
            channel: "return".into(),
            ..At::new("no response")
        },
        _ => At {
            channel: "return".into(),
            ..locate_in(expected, actual)
        },
    }
}

/// Adds `at`, the first difference, to every mismatch.
pub fn annotate(outcome: &mut DiffOutcome) {
    for m in &mut outcome.mismatches {
        if m.get("at").is_some() {
            continue;
        }
        let at = locate_value(&m["expected"], &m["actual"]);
        if let (Value::Object(map), Ok(at)) = (m, serde_json::to_value(at)) {
            map.insert("at".into(), at);
        }
    }
}

/// A difference many cases share: the same function, channel, path (indices dropped) and kind.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Group {
    pub id: String,
    #[serde(rename = "fn", default, skip_serializing_if = "Option::is_none")]
    pub function: Option<String>,
    pub channel: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    pub kind: String,
    pub cases: usize,
    /// One case that differs this way: a unit's mismatch, or an app case's name.
    pub example: Value,
}

impl Group {
    /// `count: return $ (value)`.
    pub fn label(&self) -> String {
        let mut out = String::new();
        if let Some(f) = &self.function {
            out += &format!("{f}: ");
        }
        out += &self.channel;
        if let Some(path) = &self.path {
            out += &format!(" {path}");
        }
        format!("{out} ({})", self.kind)
    }
}

/// `$.items[2].name` as `$.items[].name`: the same difference at another index is the same.
fn without_indices(path: &str) -> String {
    let mut out = String::new();
    let mut chars = path.chars().peekable();
    while let Some(c) = chars.next() {
        out.push(c);
        if c == '[' && chars.peek().is_some_and(char::is_ascii_digit) {
            while chars.peek().is_some_and(char::is_ascii_digit) {
                chars.next();
            }
        }
    }
    out
}

fn fnv(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for b in bytes {
        hash ^= u64::from(*b);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

fn group_id(function: Option<&str>, at: &At, path: Option<&str>) -> String {
    let key = format!(
        "{}\0{}\0{}\0{}",
        function.unwrap_or_default(),
        at.channel,
        path.unwrap_or_default(),
        at.kind
    );
    format!("{:016x}", fnv(key.as_bytes()))[..8].to_string()
}

struct Builder {
    groups: Vec<(Group, BTreeSet<usize>)>,
}

impl Builder {
    fn add(&mut self, function: Option<&str>, at: &At, case: usize, example: impl Fn() -> Value) {
        let path = at.path.as_deref().map(without_indices);
        let id = group_id(function, at, path.as_deref());
        match self.groups.iter_mut().find(|(g, _)| g.id == id) {
            Some((g, cases)) => {
                if cases.insert(case) {
                    g.cases += 1;
                }
            }
            None => self.groups.push((
                Group {
                    id,
                    function: function.map(str::to_string),
                    channel: at.channel.clone(),
                    path,
                    kind: at.kind.clone(),
                    cases: 1,
                    example: example(),
                },
                BTreeSet::from([case]),
            )),
        }
    }

    fn finish(self) -> Vec<Group> {
        let mut out: Vec<Group> = self.groups.into_iter().map(|(g, _)| g).collect();
        out.sort_by_key(|g| std::cmp::Reverse(g.cases));
        out
    }
}

/// The distinct differences among a unit's mismatches, most cases first.
pub fn group_mismatches(mismatches: &[Value]) -> Vec<Group> {
    let mut b = Builder { groups: Vec::new() };
    for (i, m) in mismatches.iter().enumerate() {
        let at = serde_json::from_value::<At>(m["at"].clone())
            .unwrap_or_else(|_| locate_value(&m["expected"], &m["actual"]));
        b.add(m["fn"].as_str(), &at, i, || m.clone());
    }
    b.finish()
}

/// The distinct differences among an app replay's failing cases, most cases first.
pub fn replay_groups(failures: &[Failure]) -> Vec<Group> {
    let mut b = Builder { groups: Vec::new() };
    for (i, f) in failures.iter().enumerate() {
        for p in &f.problems {
            let at = p.at.clone().unwrap_or_else(|| At {
                channel: p.field.clone(),
                ..At::new("value")
            });
            b.add(None, &at, i, || Value::String(f.case.clone()));
        }
    }
    b.finish()
}

fn canonical(v: &Value) -> Value {
    match v {
        Value::Object(map) => {
            let mut keys: Vec<&String> = map.keys().collect();
            keys.sort();
            Value::Object(
                keys.into_iter()
                    .map(|k| (k.clone(), canonical(&map[k])))
                    .collect(),
            )
        }
        Value::Array(items) => Value::Array(items.iter().map(canonical).collect()),
        other => other.clone(),
    }
}

/// FNV-64 of the value's JSON with sorted keys.
pub fn hash(v: &Value) -> String {
    format!("{:016x}", fnv(canonical(v).to_string().as_bytes()))
}

/// A hash for strings, arrays and objects; a small number or a boolean is too easy to guess
/// from its hash, so those keep only their type.
fn hashable(v: &Value) -> Option<String> {
    matches!(v, Value::String(_) | Value::Array(_) | Value::Object(_)).then(|| hash(v))
}

fn inner(result: &Value) -> &Value {
    result
        .get("ok")
        .or_else(|| result.get("error"))
        .unwrap_or(result)
}

fn without_context(at: &Value) -> Value {
    let mut at = at.clone();
    if let Value::Object(map) = &mut at {
        map.remove("context");
    }
    at
}

/// A mismatch as its shape: the function, a hash of the arguments, hashes of the results and
/// where they part.
pub fn redact_mismatch(m: &Value) -> Value {
    let mut out = json!({ "fn": m["fn"], "case": hash(&m["args"]) });
    for (key, side) in [("expected_hash", "expected"), ("actual_hash", "actual")] {
        if let Some(h) = hashable(inner(&m[side])) {
            out[key] = Value::String(h);
        }
    }
    let at = if m.get("at").is_some() {
        m["at"].clone()
    } else {
        serde_json::to_value(locate_value(&m["expected"], &m["actual"])).unwrap_or_default()
    };
    out["at"] = without_context(&at);
    out
}

/// A diff with no argument or output values left in it.
pub fn redact_diff(outcome: &mut DiffOutcome) {
    for m in &mut outcome.mismatches {
        *m = redact_mismatch(m);
    }
    for e in &mut outcome.stale_known {
        *e = json!({ "fn": e["fn"], "case": hash(&e["args"]) });
    }
    for g in &mut outcome.groups {
        g.example = redact_mismatch(&g.example);
    }
}

fn is_request(args: &[String]) -> bool {
    args.len() == 2
        && args[1].starts_with('/')
        && !args[0].is_empty()
        && args[0].chars().all(|c| c.is_ascii_uppercase())
}

/// A replay with no outputs, arguments or request paths left in it: each problem is its shape.
pub fn redact_replay(outcome: &mut ReplayOutcome) {
    for f in &mut outcome.failures {
        f.args = if is_request(&f.args) {
            vec![f.args[0].clone(), hash(&Value::String(f.args[1].clone()))]
        } else {
            vec![hash(&json!(f.args))]
        };
        for p in &mut f.problems {
            if let Some(at) = &mut p.at {
                at.context = None;
            }
            p.detail = match &p.at {
                Some(at) => format!("{}\n", at.shape()),
                None => "differs\n".to_string(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_nested_value_names_its_path_and_the_character() {
        let at = locate_value(
            &json!({ "ok": { "items": [{ "name": "abc" }, { "name": "abcdef" }] } }),
            &json!({ "ok": { "items": [{ "name": "abc" }, { "name": "abXdef" }] } }),
        );
        assert_eq!(at.channel, "return");
        assert_eq!(at.path.as_deref(), Some("$.items[1].name"));
        assert_eq!(at.kind, "value");
        assert_eq!(at.offset, Some(Offset { char: 2, byte: 2 }));
        assert_eq!(at.column, Some(3));
        let c = at.context.unwrap();
        assert_eq!(c.expected, "ab⟨c⟩def");
        assert_eq!(c.actual, "ab⟨X⟩def");
    }

    #[test]
    fn unequal_arrays_and_keys() {
        let at = locate_in(&json!([1, 2, 3]), &json!([1, 2]));
        assert_eq!(
            (at.kind.as_str(), at.path.as_deref()),
            ("length", Some("$"))
        );
        assert_eq!((at.expected_len, at.actual_len), (Some(3), Some(2)));
        let at = locate_in(&json!({ "a": 1, "b": 2 }), &json!({ "a": 1 }));
        assert_eq!(
            (at.kind.as_str(), at.path.as_deref()),
            ("missing key", Some("$.b"))
        );
        let at = locate_in(&json!({ "a": 1 }), &json!({ "a": 1, "my key": 2 }));
        assert_eq!(at.kind, "extra key");
        assert_eq!(at.path.as_deref(), Some(r#"$["my key"]"#));
        let at = locate_in(&json!({ "a": 1 }), &json!({ "a": "1" }));
        assert_eq!(at.kind, "type");
        assert_eq!(at.expected_type.as_deref(), Some("number"));
        assert_eq!(at.actual_type.as_deref(), Some("string"));
    }

    #[test]
    fn an_error_against_a_value_is_return_vs_throw() {
        let at = locate_value(&json!({ "error": "boom" }), &json!({ "ok": 3 }));
        assert_eq!(at.channel, "return vs throw");
        assert_eq!(at.kind, "type");
        assert_eq!(at.expected_type.as_deref(), Some("throw"));
        let at = locate_value(&json!({ "error": "bad x" }), &json!({ "error": "bad y" }));
        assert_eq!((at.channel.as_str(), at.kind.as_str()), ("throw", "value"));
        let at = locate_value(
            &json!([{ "ok": true }, { "ok": 1 }, { "ok": 2 }]),
            &json!([{ "ok": true }, { "ok": 1 }, { "ok": 5 }]),
        );
        assert_eq!(at.channel, "call 2");
        assert_eq!(at.path.as_deref(), Some("$"));
    }

    #[test]
    fn unicode_offsets_count_chars_and_bytes() {
        let at = locate_in(&json!("héllo wörld"), &json!("héllo wørld"));
        assert_eq!(at.offset, Some(Offset { char: 7, byte: 8 }));
        let c = at.context.unwrap();
        assert_eq!(c.expected, "héllo w⟨ö⟩rld");
        assert_eq!(c.actual, "héllo w⟨ø⟩rld");
        let at = locate_in(&json!("ab"), &json!("abc"));
        assert_eq!(at.context.unwrap().expected, "ab⟨⟩");
        let long = "x".repeat(40);
        let at = locate_in(
            &json!(format!("{long}a{long}")),
            &json!(format!("{long}b{long}")),
        );
        let c = at.context.unwrap();
        assert!(
            c.expected.starts_with('…') && c.expected.ends_with('…'),
            "{c:?}"
        );
        assert_eq!(c.expected.chars().count(), 2 * CONTEXT + 5);
    }

    #[test]
    fn the_same_bug_at_other_indices_is_one_group() {
        let mut outcome = DiffOutcome {
            mismatches: (0..4)
                .map(|i| {
                    let mut items = vec![json!("a"); 5];
                    let mut wrong = items.clone();
                    wrong[i] = json!("b");
                    items[i] = json!("c");
                    json!({ "fn": "f", "args": [i], "expected": { "ok": items }, "actual": { "ok": wrong } })
                })
                .chain([json!({ "fn": "g", "args": [], "expected": { "ok": 1 }, "actual": { "ok": 2 } })])
                .collect(),
            ..DiffOutcome::default()
        };
        annotate(&mut outcome);
        let groups = group_mismatches(&outcome.mismatches);
        assert_eq!(groups.len(), 2, "{groups:?}");
        assert_eq!(groups[0].cases, 4);
        assert_eq!(groups[0].path.as_deref(), Some("$[]"));
        assert_eq!(groups[0].label(), "f: return $[] (value)");
        assert_eq!(groups[0].id.len(), 8);
        assert_eq!(groups[1].label(), "g: return $ (value)");
        assert_ne!(groups[0].id, groups[1].id);
    }

    #[test]
    fn a_redacted_mismatch_keeps_its_shape_only() {
        let m = json!({ "fn": "f", "args": ["secret input"], "expected": { "ok": { "name": "Ada" } }, "actual": { "ok": { "name": "Bob" } } });
        let r = redact_mismatch(&m);
        let text = r.to_string();
        for value in ["secret", "Ada", "Bob"] {
            assert!(!text.contains(value), "{text}");
        }
        assert_eq!(r["at"]["path"], "$.name");
        assert_eq!(r["case"], hash(&json!(["secret input"])));
        assert!(r["expected_hash"].is_string());
        let n = redact_mismatch(
            &json!({ "fn": "f", "args": [1], "expected": { "ok": 1 }, "actual": { "ok": 2 } }),
        );
        assert!(n.get("expected_hash").is_none(), "{n}");
        assert_eq!(n["at"]["expected_type"], "number");
        assert_eq!(
            hash(&json!({ "a": 1, "b": 2 })),
            hash(&json!({ "b": 2, "a": 1 }))
        );
    }
}
