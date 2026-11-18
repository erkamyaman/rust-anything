//! Checks json-server-js against what Node answered on the same inputs (`oracle.mjs`).

use json_server_js::{collate, dotprop, inflection, json, number, url};
use serde_json::{Map, Value};

fn oracle() -> Value {
    let text = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/node-oracle.json"
    ))
    .expect("tests/node-oracle.json");
    json::parse(&text).expect("oracle JSON")
}

fn report(what: &str, failures: Vec<String>, total: usize) {
    assert!(
        failures.is_empty(),
        "{what}: {} of {total} differ from Node:\n{}",
        failures.len(),
        failures
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn s(v: &Value) -> &str {
    v.as_str().unwrap_or_default()
}

#[test]
fn json_parse_and_stringify_match_v8() {
    let o = oracle();
    let cases = o["json"].as_array().unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let input = s(&case[0]);
        let got = match json::parse(input) {
            Ok(v) => serde_json::json!({ "ok": json::stringify(&v, 2) }),
            Err(e) => serde_json::json!({ "err": e }),
        };
        if got != case[1] {
            failures.push(format!("{input:?}: node {} rust {got}", case[1]));
        }
    }
    report("JSON.parse", failures, cases.len());
}

#[test]
fn number_conversion_matches_v8() {
    let o = oracle();
    let cases = o["numbers"].as_array().unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let got = number::number_to_string(number::string_to_number(s(&case[0])));
        if got != s(&case[1]) {
            failures.push(format!("{:?}: node {} rust {got}", s(&case[0]), case[1]));
        }
    }
    report("Number()", failures, cases.len());
}

#[test]
fn locale_compare_matches_icu() {
    let o = oracle();
    let cases = o["collate"].as_array().unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let got = collate::locale_compare(s(&case[0]), s(&case[1]));
        if Some(got) != case[2].as_f64() {
            failures.push(format!(
                "{:?} vs {:?}: node {} rust {got}",
                s(&case[0]),
                s(&case[1]),
                case[2]
            ));
        }
    }
    report("localeCompare", failures, cases.len());
}

#[test]
fn inflection_matches_the_package() {
    let o = oracle();
    let cases = o["inflect"].as_array().unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let (w, sing, plur) = (s(&case[0]), s(&case[1]), s(&case[2]));
        let (rs, rp) = (inflection::singularize(w), inflection::pluralize(w));
        if rs != sing || rp != plur {
            failures.push(format!("{w:?}: node {sing}/{plur} rust {rs}/{rp}"));
        }
    }
    report("inflection", failures, cases.len());
}

#[test]
fn query_strings_parse_as_node() {
    let o = oracle();
    let cases = o["search"].as_array().unwrap();
    let mut failures = Vec::new();
    for case in cases {
        let q = s(&case[0]);
        let pairs = url::search_params(q);
        let got_pairs: Value = pairs
            .iter()
            .map(|(k, v)| serde_json::json!([k, v]))
            .collect();
        let got_string = url::serialize_search_params(&pairs);
        let got_qs: Value = url::querystring(q)
            .into_iter()
            .map(|(k, v)| match v {
                url::QueryValue::One(x) => serde_json::json!([k, x]),
                url::QueryValue::Many(xs) => serde_json::json!([k, xs]),
            })
            .collect();
        let mut node_qs = case[3].as_array().cloned().unwrap_or_default();
        let mut rust_qs = got_qs.as_array().cloned().unwrap_or_default();
        let key = |v: &Value| v[0].as_str().unwrap_or_default().to_string();
        node_qs.sort_by_key(key);
        rust_qs.sort_by_key(key);
        if got_pairs != case[1] || got_string != s(&case[2]) || node_qs != rust_qs {
            failures.push(format!(
                "{q:?}: node {} {} {:?} rust {got_pairs} {got_string} {rust_qs:?}",
                case[1], case[2], node_qs
            ));
        }
    }
    report("URLSearchParams/querystring", failures, cases.len());
}

#[test]
fn dot_prop_matches_the_package() {
    let o = oracle();
    let cases = o["dotprop"].as_array().unwrap();
    let sample = serde_json::json!({ "a": { "b": [10, { "lt": "x" }], "0": "zero" }, "b": "héllo", "1": [1, 2], "length": 3 });
    let mut failures = Vec::new();
    for case in cases {
        let p = s(&case[0]);
        let mut root = Map::new();
        let set = match dotprop::set_property(&mut root, &format!("{p}.eq"), serde_json::json!(1)) {
            Ok(()) => serde_json::json!({ "ok": json::stringify(&Value::Object(root), 0) }),
            Err(e) => serde_json::json!({ "err": e }),
        };
        let get = match dotprop::get_property(&sample, p) {
            Ok(None) => serde_json::json!({ "ok": "<undefined>" }),
            Ok(Some(v)) => serde_json::json!({ "ok": json::stringify(&v, 0) }),
            Err(e) => serde_json::json!({ "err": e }),
        };
        if set != case[1] || get != case[2] {
            failures.push(format!(
                "{p:?}: node {} {} rust {set} {get}",
                case[1], case[2]
            ));
        }
    }
    report("dot-prop", failures, cases.len());
}
