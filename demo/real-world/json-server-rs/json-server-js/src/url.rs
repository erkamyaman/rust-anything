//! URL decoding as `URLSearchParams`, Node's `querystring.parse`, `decodeURIComponent` and
//! `decodeURI` do it.

fn hex(b: u8) -> Option<u8> {
    (b as char).to_digit(16).map(|d| d as u8)
}

/// Percent-decodes every valid `%XX`, leaving the rest as it is, then decodes UTF-8 with
/// replacement characters (WHATWG percent-decode, and Node's `unescapeBuffer`).
pub fn percent_decode_lossy(s: &str) -> String {
    let b = s.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%'
            && i + 2 < b.len()
            && let (Some(h), Some(l)) = (hex(b[i + 1]), hex(b[i + 2]))
        {
            out.push(h * 16 + l);
            i += 3;
            continue;
        }
        out.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Node's `querystring.unescapeBuffer(s).toString()`: valid `%XX` become bytes, every other
/// UTF-16 unit keeps only its low byte, then UTF-8 decoding with replacement characters.
fn unescape_buffer(s: &str) -> String {
    let units: Vec<u16> = s.encode_utf16().collect();
    let unhex = |u: u16| char::from_u32(u32::from(u)).and_then(|c| c.to_digit(16));
    let max = units.len().saturating_sub(2);
    let mut out = Vec::with_capacity(units.len());
    let mut i = 0;
    while i < units.len() {
        let mut c = units[i];
        if c == u16::from(b'%') && i < max {
            i += 1;
            c = units[i];
            let Some(high) = unhex(c) else {
                out.push(b'%');
                continue;
            };
            i += 1;
            match unhex(units[i]) {
                Some(low) => c = (high * 16 + low) as u16,
                None => {
                    out.push(b'%');
                    i -= 1;
                }
            }
        }
        out.push(c as u8);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// Node's `querystring.unescape`.
fn node_unescape(s: &str) -> String {
    decode_uri_component(s).unwrap_or_else(|_| unescape_buffer(s))
}

/// Tracks whether a run of characters holds a valid `%XX`, as Node's parsers do.
#[derive(Default)]
struct EncodeCheck {
    state: u8,
}

impl EncodeCheck {
    fn feed(&mut self, c: char) -> bool {
        if c == '%' {
            self.state = 1;
        } else if self.state > 0 {
            if c.is_ascii_hexdigit() {
                self.state += 1;
                if self.state == 3 {
                    return true;
                }
            } else {
                self.state = 0;
            }
        }
        false
    }
}

/// `new URLSearchParams(query)` as Node parses it: name/value pairs in order.
pub fn search_params(query: &str) -> Vec<(String, String)> {
    let qs = query.strip_prefix('?').unwrap_or(query);
    let mut out: Vec<String> = Vec::new();
    let mut seen_sep = false;
    let mut buf = String::new();
    let mut encoded = false;
    let mut check = EncodeCheck::default();
    let mut pair_start = 0;
    let mut last = 0;
    let finish = |buf: &mut String, encoded: bool, out: &mut Vec<String>| {
        let piece = std::mem::take(buf);
        out.push(if encoded {
            node_unescape(&piece)
        } else {
            piece
        });
    };
    for (i, c) in qs.char_indices() {
        if c == '&' {
            if pair_start == i {
                pair_start = i + 1;
                last = i + 1;
                continue;
            }
            buf.push_str(&qs[last..i]);
            finish(&mut buf, encoded, &mut out);
            if !seen_sep {
                out.push(String::new());
            }
            seen_sep = false;
            encoded = false;
            check = EncodeCheck::default();
            pair_start = i + 1;
            last = i + 1;
            continue;
        }
        if !seen_sep && c == '=' {
            buf.push_str(&qs[last..i]);
            finish(&mut buf, encoded, &mut out);
            seen_sep = true;
            encoded = false;
            check = EncodeCheck::default();
            last = i + 1;
            continue;
        }
        if c == '+' {
            buf.push_str(&qs[last..i]);
            buf.push(' ');
            last = i + 1;
        } else if !encoded && check.feed(c) {
            encoded = true;
        }
    }
    if pair_start < qs.len() {
        buf.push_str(&qs[last..]);
        finish(&mut buf, encoded, &mut out);
        if !seen_sep {
            out.push(String::new());
        }
    }
    out.chunks(2)
        .map(|kv| (kv[0].clone(), kv.get(1).cloned().unwrap_or_default()))
        .collect()
}

fn form_encode(s: &str, out: &mut String) {
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'*' | b'-' | b'.' | b'_' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
}

/// `URLSearchParams.prototype.toString()`.
pub fn serialize_search_params(pairs: &[(String, String)]) -> String {
    let mut out = String::new();
    for (i, (name, value)) in pairs.iter().enumerate() {
        if i > 0 {
            out.push('&');
        }
        form_encode(name, &mut out);
        out.push('=');
        form_encode(value, &mut out);
    }
    out
}

/// One value of Node's `querystring.parse`: a string, or every value of a repeated key.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryValue {
    One(String),
    Many(Vec<String>),
}

impl QueryValue {
    /// `ensureArray`: the values as a list.
    pub fn to_vec(&self) -> Vec<String> {
        match self {
            QueryValue::One(s) => vec![s.clone()],
            QueryValue::Many(v) => v.clone(),
        }
    }
}

fn add_key_value(
    out: &mut Vec<(String, QueryValue)>,
    key: String,
    value: String,
    key_encoded: bool,
    value_encoded: bool,
) {
    let key = if !key.is_empty() && key_encoded {
        node_unescape(&key)
    } else {
        key
    };
    let value = if !value.is_empty() && value_encoded {
        node_unescape(&value)
    } else {
        value
    };
    match out.iter().position(|(k, _)| *k == key) {
        Some(idx) => {
            let slot = &mut out[idx].1;
            match slot {
                QueryValue::One(first) => {
                    let first = std::mem::take(first);
                    *slot = QueryValue::Many(vec![first, value]);
                }
                QueryValue::Many(all) => all.push(value),
            }
        }
        None => out.push((key, QueryValue::One(value))),
    }
}

/// Node's `querystring.parse(str)`, its loop ported as it is (at most 1000 keys).
pub fn querystring(qs: &str) -> Vec<(String, QueryValue)> {
    let mut out: Vec<(String, QueryValue)> = Vec::new();
    if qs.is_empty() {
        return out;
    }
    let mut pairs = 1000;
    let (mut key, mut value) = (String::new(), String::new());
    let (mut key_encoded, mut value_encoded) = (false, false);
    let mut check = EncodeCheck::default();
    let mut last = 0;
    let mut seen_eq = false;
    for (i, c) in qs.char_indices() {
        if c == '&' {
            if !seen_eq {
                if last < i {
                    key.push_str(&qs[last..i]);
                } else if key.is_empty() {
                    pairs -= 1;
                    if pairs == 0 {
                        return out;
                    }
                    last = i + 1;
                    continue;
                }
            } else if last < i {
                value.push_str(&qs[last..i]);
            }
            add_key_value(
                &mut out,
                std::mem::take(&mut key),
                std::mem::take(&mut value),
                key_encoded,
                value_encoded,
            );
            pairs -= 1;
            if pairs == 0 {
                return out;
            }
            key_encoded = false;
            value_encoded = false;
            check = EncodeCheck::default();
            last = i + 1;
            seen_eq = false;
            continue;
        }
        if !seen_eq {
            if c == '=' {
                if last < i {
                    key.push_str(&qs[last..i]);
                }
                check = EncodeCheck::default();
                last = i + 1;
                seen_eq = true;
                continue;
            }
            if !key_encoded {
                if c == '%' {
                    check.state = 1;
                    continue;
                } else if check.state > 0 {
                    if c.is_ascii_hexdigit() {
                        check.state += 1;
                        if check.state == 3 {
                            key_encoded = true;
                        }
                        continue;
                    }
                    check.state = 0;
                }
            }
            if c == '+' {
                if last < i {
                    key.push_str(&qs[last..i]);
                }
                key.push(' ');
                last = i + 1;
                continue;
            }
        }
        if c == '+' {
            if last < i {
                value.push_str(&qs[last..i]);
            }
            value.push(' ');
            last = i + 1;
        } else if !value_encoded && check.feed(c) {
            value_encoded = true;
        }
    }
    if last < qs.len() {
        if !seen_eq {
            key.push_str(&qs[last..]);
        } else {
            value.push_str(&qs[last..]);
        }
    } else if !seen_eq && key.is_empty() {
        return out;
    }
    add_key_value(&mut out, key, value, key_encoded, value_encoded);
    out
}

/// Looks a key up in a parsed query string.
pub fn query_get<'a>(query: &'a [(String, QueryValue)], key: &str) -> Option<&'a QueryValue> {
    query.iter().find(|(k, _)| k == key).map(|(_, v)| v)
}

/// JS's `URIError: URI malformed`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UriError;

fn decode_with(s: &str, keep: &str) -> Result<String, UriError> {
    let b = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(b.len());
    let mut i = 0;
    let byte_at = |i: usize| -> Result<u8, UriError> {
        if b.get(i) != Some(&b'%') {
            return Err(UriError);
        }
        let (h, l) = (
            b.get(i + 1).copied().and_then(hex),
            b.get(i + 2).copied().and_then(hex),
        );
        match (h, l) {
            (Some(h), Some(l)) => Ok(h * 16 + l),
            _ => Err(UriError),
        }
    };
    while i < b.len() {
        if b[i] != b'%' {
            out.push(b[i]);
            i += 1;
            continue;
        }
        let first = byte_at(i)?;
        if first < 0x80 {
            if keep.as_bytes().contains(&first) {
                out.extend_from_slice(&b[i..i + 3]);
            } else {
                out.push(first);
            }
            i += 3;
            continue;
        }
        let len = match first {
            0xC0..=0xDF => 2,
            0xE0..=0xEF => 3,
            0xF0..=0xF7 => 4,
            _ => return Err(UriError),
        };
        let mut bytes = vec![first];
        for k in 1..len {
            let next = byte_at(i + 3 * k)?;
            if next & 0xC0 != 0x80 {
                return Err(UriError);
            }
            bytes.push(next);
        }
        let text = std::str::from_utf8(&bytes).map_err(|_| UriError)?;
        out.extend_from_slice(text.as_bytes());
        i += 3 * len;
    }
    String::from_utf8(out).map_err(|_| UriError)
}

/// `decodeURIComponent`: `Err` where JS throws a `URIError`.
pub fn decode_uri_component(s: &str) -> Result<String, UriError> {
    decode_with(s, "")
}

/// `decodeURI`: like `decodeURIComponent`, but escapes of `;/?:@&=+$,#` stay escaped.
pub fn decode_uri(s: &str) -> Result<String, UriError> {
    decode_with(s, ";/?:@&=+$,#")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decoding_matches_javascript() {
        assert_eq!(
            search_params("a=1&&b=x+y%20z&c&=v&d=%zz%E0"),
            [
                ("a".into(), "1".into()),
                ("b".into(), "x y z".into()),
                ("c".into(), String::new()),
                (String::new(), "v".into()),
                ("d".into(), "%zz\u{FFFD}".into()),
            ]
        );
        assert_eq!(
            serialize_search_params(&[("a b".into(), "é*~".into())]),
            "a+b=%C3%A9*%7E"
        );
        assert_eq!(decode_uri_component("%F0%9F%91%8D"), Ok("👍".into()));
        assert!(decode_uri_component("%E0").is_err());
        assert!(decode_uri_component("%ED%A0%80").is_err());
        assert_eq!(decode_uri("%2Fa%20b"), Ok("%2Fa b".into()));
        let q = querystring("_embed=a&_embed=b&x=%E0&y=1+2");
        assert_eq!(
            query_get(&q, "_embed"),
            Some(&QueryValue::Many(vec!["a".into(), "b".into()]))
        );
        assert_eq!(
            query_get(&q, "x"),
            Some(&QueryValue::One("\u{FFFD}".into()))
        );
        assert_eq!(query_get(&q, "y"), Some(&QueryValue::One("1 2".into())));
    }
}
