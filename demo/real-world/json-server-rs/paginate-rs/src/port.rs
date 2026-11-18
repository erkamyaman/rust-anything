//! The Rust port of `src/paginate.ts` (json-server, MIT).

#![forbid(unsafe_code)]

use serde_json::Value;

use crate::js;

use self::page::{page_of, result, slice_bounds};

/// The page arithmetic, shared with the app, which slices its items without copying them.
pub mod page {
    use serde_json::{Map, Value};

    use crate::js;

    /// The numbers of a page: `first`, `prev`, `next`, `last`, `pages`, and the slice of items to
    /// show, computed with JavaScript's arithmetic (NaN and Infinity included).
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Page {
        pub first: f64,
        pub prev: Option<f64>,
        pub next: Option<f64>,
        pub last: f64,
        pub pages: f64,
        pub start: f64,
        pub end: f64,
    }

    fn js_max(a: f64, b: f64) -> f64 {
        if a.is_nan() || b.is_nan() {
            f64::NAN
        } else {
            a.max(b)
        }
    }

    fn js_min(a: f64, b: f64) -> f64 {
        if a.is_nan() || b.is_nan() {
            f64::NAN
        } else {
            a.min(b)
        }
    }

    /// The page arithmetic of `paginate` for `total` items (`NaN` when `items.length` is not a
    /// number).
    pub fn page_of(total: f64, page: f64, per_page: f64) -> Page {
        let safe_per_page = if per_page.is_finite() && per_page > 0.0 {
            per_page.floor()
        } else {
            1.0
        };
        let pages = js_max(1.0, (total / safe_per_page).ceil());
        let safe_page = if page.is_finite() { page.floor() } else { 1.0 };
        let current = js_max(1.0, js_min(safe_page, pages));
        let start = (current - 1.0) * safe_per_page;
        Page {
            first: 1.0,
            prev: (current > 1.0).then_some(current - 1.0),
            next: (current < pages).then_some(current + 1.0),
            last: pages,
            pages,
            start,
            end: start + safe_per_page,
        }
    }

    /// `Array.prototype.slice(start, end)` bounds on a length.
    pub fn slice_bounds(len: usize, start: f64, end: f64) -> (usize, usize) {
        let len_f = len as f64;
        let clamp = |x: f64| {
            let x = if x.is_nan() { 0.0 } else { x.trunc() };
            if x < 0.0 {
                (len_f + x).max(0.0)
            } else {
                x.min(len_f)
            }
        };
        let (s, e) = (clamp(start), clamp(end));
        (s as usize, (e as usize).max(s as usize))
    }

    /// The result object, `data` given.
    pub fn result(p: &Page, total: f64, data: Value) -> Value {
        let mut out = Map::new();
        out.insert("first".into(), js::number(p.first));
        out.insert("prev".into(), p.prev.map_or(Value::Null, js::number));
        out.insert("next".into(), p.next.map_or(Value::Null, js::number));
        out.insert("last".into(), js::number(p.last));
        out.insert("pages".into(), js::number(p.pages));
        out.insert(
            "items".into(),
            if total.is_nan() {
                js::undefined()
            } else {
                js::number(total)
            },
        );
        out.insert("data".into(), data);
        Value::Object(out)
    }
}

/// Port of `paginate` (`src/paginate.ts:11`). `items` is any JS value, as the function receives
/// it: arrays and strings slice, anything else throws what JS throws.
pub fn paginate(
    items: serde_json::Value,
    page: f64,
    per_page: f64,
) -> Result<serde_json::Value, String> {
    let (total, units): (f64, Option<Vec<u16>>) = match &items {
        Value::Null => return Err("Cannot read properties of null (reading 'length')".into()),
        v if js::is_undefined(v) => {
            return Err("Cannot read properties of undefined (reading 'length')".into());
        }
        Value::Array(a) => (a.len() as f64, None),
        v => match js::string_units(v) {
            Some(units) => (units.len() as f64, Some(units)),
            None => (f64::NAN, None),
        },
    };
    let p = page_of(total, page, per_page);
    let data = match (items, units) {
        (Value::Array(a), _) => {
            let (s, e) = slice_bounds(a.len(), p.start, p.end);
            Value::Array(a[s..e].to_vec())
        }
        (_, Some(units)) => {
            let (s, e) = slice_bounds(units.len(), p.start, p.end);
            js::utf16_string(&units[s..e])
        }
        _ => return Err("items.slice is not a function".into()),
    };
    Ok(result(&p, total, data))
}
