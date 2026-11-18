//! The index page: `views/index.html` rendered as eta 4 renders it (`autoTrim: [false, 'nl']`:
//! the newline after each tag is dropped).

use std::sync::OnceLock;

use json_server_js::value::ordered_keys;
use serde_json::{Map, Value};

const TEMPLATE: &str = include_str!("index.html");

/// The text between the template's tags, in order.
fn pieces() -> &'static [String] {
    static PIECES: OnceLock<Vec<String>> = OnceLock::new();
    PIECES.get_or_init(|| {
        let mut out = Vec::new();
        let mut rest = TEMPLATE;
        let mut after_tag = false;
        loop {
            let (text, next) = match rest.find("<%") {
                Some(i) => (&rest[..i], Some(i)),
                None => (rest, None),
            };
            let text = if after_tag {
                text.strip_prefix("\r\n")
                    .or_else(|| text.strip_prefix('\n'))
                    .unwrap_or(text)
            } else {
                text
            };
            out.push(text.to_string());
            let Some(i) = next else { break };
            let close = rest[i..].find("%>").map_or(rest.len(), |j| i + j + 2);
            rest = &rest[close..];
            after_tag = true;
        }
        out
    })
}

/// eta's `XMLEscape`.
pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            c => out.push(c),
        }
    }
    out
}

/// The page for the database's data.
pub fn render(data: &Map<String, Value>) -> String {
    let s = pieces();
    let mut out = String::with_capacity(TEMPLATE.len() + 128 * data.len());
    out.push_str(&s[0]);
    out.push_str(&s[1]);
    if data.is_empty() {
        out.push_str(&s[2]);
    } else {
        out.push_str(&s[3]);
        for name in ordered_keys(data) {
            let name_html = escape(name);
            out.push_str(&s[4]);
            out.push_str(&name_html);
            out.push_str(&s[5]);
            out.push_str(&name_html);
            out.push_str(&s[6]);
            match &data[name.as_str()] {
                Value::Array(items) => {
                    out.push_str(&s[7]);
                    out.push_str(&items.len().to_string());
                    out.push_str(&s[8]);
                }
                _ => out.push_str(&s[9]),
            }
            out.push_str(&s[10]);
        }
        out.push_str(&s[11]);
    }
    out.push_str(&s[12]);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_template_has_the_tags_the_renderer_expects() {
        assert_eq!(pieces().len(), 13);
        let page = render(&Map::new());
        assert!(page.contains("No resources in db.json."));
    }
}
