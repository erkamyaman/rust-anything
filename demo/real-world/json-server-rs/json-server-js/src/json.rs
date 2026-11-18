//! `JSON.parse` with V8's error messages, and `JSON.stringify(value, null, indent)`.

use serde_json::{Map, Value};

use crate::number::number_to_string;
use crate::value::{num, ordered_keys};

const CONTEXT: usize = 10;
const MIN_LENGTH_FOR_CONTEXT: usize = CONTEXT * 2 + 1;

enum Token {
    Eos,
    Number,
    Str,
    Other,
}

struct Parser<'a> {
    src: &'a [u16],
    pos: usize,
}

fn is_ws(c: u16) -> bool {
    matches!(c, 0x20 | 0x09 | 0x0A | 0x0D)
}

impl Parser<'_> {
    fn peek(&self) -> Option<u16> {
        self.src.get(self.pos).copied()
    }

    fn skip_ws(&mut self) {
        while self.peek().is_some_and(is_ws) {
            self.pos += 1;
        }
    }

    fn location(&self) -> (usize, usize) {
        let (cr, lf) = (u16::from(b'\r'), u16::from(b'\n'));
        let mut line = 1;
        let mut line_start = 0;
        let mut i = 0;
        while i < self.pos {
            if self.src[i] == cr && i + 1 < self.pos && self.src[i + 1] == lf {
                i += 1;
            }
            if self.src[i] == cr || self.src[i] == lf {
                line += 1;
                line_start = i + 1;
            }
            i += 1;
        }
        (line, i - line_start + 1)
    }

    fn at(&self, what: &str) -> String {
        self.at_raw(&format!("{what} in JSON"))
    }

    fn at_raw(&self, what: &str) -> String {
        let (line, column) = self.location();
        format!(
            "{what} at position {} (line {line} column {column})",
            self.pos
        )
    }

    fn token(&self) -> Token {
        match self.peek() {
            None => Token::Eos,
            Some(c) if c == u16::from(b'"') => Token::Str,
            Some(c) if c == u16::from(b'-') || (0x30..=0x39).contains(&c) => Token::Number,
            Some(_) => Token::Other,
        }
    }

    /// V8's message for an unexpected token with no more specific message.
    fn unexpected(&self) -> String {
        match self.token() {
            Token::Eos => "Unexpected end of JSON input".into(),
            Token::Number => self.at("Unexpected number"),
            Token::Str => self.at("Unexpected string"),
            Token::Other => {
                let whole = String::from_utf16_lossy(self.src);
                if matches!(
                    whole.as_str(),
                    "undefined" | "NaN" | "Infinity" | "[object Object]"
                ) {
                    return format!("\"{whole}\" is not valid JSON");
                }
                let token = String::from_utf16_lossy(&self.src[self.pos..self.pos + 1]);
                let len = self.src.len();
                let pos = self.pos;
                if len < MIN_LENGTH_FOR_CONTEXT {
                    return format!("Unexpected token '{token}', \"{whole}\" is not valid JSON");
                }
                let slice = |a: usize, b: usize| String::from_utf16_lossy(&self.src[a..b.min(len)]);
                if pos < CONTEXT {
                    format!(
                        "Unexpected token '{token}', \"{}\"... is not valid JSON",
                        slice(0, pos + CONTEXT)
                    )
                } else if pos < len - CONTEXT {
                    format!(
                        "Unexpected token '{token}', ...\"{}\"... is not valid JSON",
                        slice(pos - CONTEXT, pos + CONTEXT)
                    )
                } else {
                    format!(
                        "Unexpected token '{token}', ...\"{}\" is not valid JSON",
                        slice(pos - CONTEXT, len)
                    )
                }
            }
        }
    }

    fn expect(&mut self, c: u8, message: &str) -> Result<(), String> {
        self.skip_ws();
        if self.peek() == Some(u16::from(c)) {
            self.pos += 1;
            Ok(())
        } else {
            Err(self.at(message))
        }
    }

    fn literal(&mut self, word: &str, value: Value) -> Result<Value, String> {
        for b in word.bytes() {
            if self.peek() != Some(u16::from(b)) {
                return Err(self.unexpected());
            }
            self.pos += 1;
        }
        Ok(value)
    }

    fn digits(&mut self) -> usize {
        let start = self.pos;
        while self.peek().is_some_and(|c| (0x30..=0x39).contains(&c)) {
            self.pos += 1;
        }
        self.pos - start
    }

    fn number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        if self.peek() == Some(u16::from(b'-')) {
            self.pos += 1;
            if !self.peek().is_some_and(|c| (0x30..=0x39).contains(&c)) {
                return Err(self.at("No number after minus sign"));
            }
        }
        if self.peek() == Some(u16::from(b'0')) {
            self.pos += 1;
            if self.peek().is_some_and(|c| (0x30..=0x39).contains(&c)) {
                return Err(self.at("Unexpected number"));
            }
        } else {
            self.digits();
        }
        if self.peek() == Some(u16::from(b'.')) {
            self.pos += 1;
            if self.digits() == 0 {
                return Err(self.at("Unterminated fractional number"));
            }
        }
        if matches!(self.peek(), Some(c) if c == u16::from(b'e') || c == u16::from(b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(c) if c == u16::from(b'+') || c == u16::from(b'-')) {
                self.pos += 1;
            }
            if self.digits() == 0 {
                return Err(self.at("Exponent part is missing a number"));
            }
        }
        let text = String::from_utf16_lossy(&self.src[start..self.pos]);
        Ok(num(text.parse::<f64>().unwrap_or(f64::NAN)))
    }

    fn string(&mut self) -> Result<String, String> {
        self.pos += 1;
        let mut out: Vec<u16> = Vec::new();
        loop {
            let Some(c) = self.peek() else {
                return Err(self.at("Unterminated string"));
            };
            if c == u16::from(b'"') {
                self.pos += 1;
                return Ok(String::from_utf16_lossy(&out));
            }
            if c < 0x20 {
                return Err(self.at("Bad control character in string literal"));
            }
            if c != u16::from(b'\\') {
                out.push(c);
                self.pos += 1;
                continue;
            }
            self.pos += 1;
            let Some(e) = self.peek() else {
                return Err(self.at("Unterminated string"));
            };
            let simple = match u8::try_from(e).unwrap_or(0) {
                b'"' => Some(u16::from(b'"')),
                b'\\' => Some(u16::from(b'\\')),
                b'/' => Some(u16::from(b'/')),
                b'b' => Some(8),
                b'f' => Some(12),
                b'n' => Some(10),
                b'r' => Some(13),
                b't' => Some(9),
                _ => None,
            };
            if let Some(s) = simple {
                out.push(s);
                self.pos += 1;
                continue;
            }
            if e != u16::from(b'u') {
                return Err(self.at("Bad escaped character"));
            }
            self.pos += 1;
            let mut unit: u16 = 0;
            for _ in 0..4 {
                let digit = self
                    .peek()
                    .and_then(|h| char::from_u32(u32::from(h)))
                    .and_then(|h| h.to_digit(16));
                let Some(d) = digit else {
                    if self.peek().is_none() {
                        return Err(self.at("Unterminated string"));
                    }
                    return Err(self.at("Bad Unicode escape"));
                };
                unit = unit * 16 + d as u16;
                self.pos += 1;
            }
            out.push(unit);
        }
    }

    fn value(&mut self, depth: usize) -> Result<Value, String> {
        self.skip_ws();
        if depth > 4096 {
            return Err("Maximum call stack size exceeded".into());
        }
        match self.peek().and_then(|c| u8::try_from(c).ok()) {
            Some(b'"') => self.string().map(Value::String),
            Some(b'-' | b'0'..=b'9') => self.number(),
            Some(b't') => self.literal("true", Value::Bool(true)),
            Some(b'f') => self.literal("false", Value::Bool(false)),
            Some(b'n') => self.literal("null", Value::Null),
            Some(b'[') => {
                self.pos += 1;
                let mut items = Vec::new();
                self.skip_ws();
                if self.peek() == Some(u16::from(b']')) {
                    self.pos += 1;
                    return Ok(Value::Array(items));
                }
                loop {
                    items.push(self.value(depth + 1)?);
                    self.skip_ws();
                    if self.peek() == Some(u16::from(b',')) {
                        self.pos += 1;
                        continue;
                    }
                    self.expect(b']', "Expected ',' or ']' after array element")?;
                    return Ok(Value::Array(items));
                }
            }
            Some(b'{') => {
                self.pos += 1;
                let mut map = Map::new();
                self.skip_ws();
                if self.peek() == Some(u16::from(b'}')) {
                    self.pos += 1;
                    return Ok(Value::Object(map));
                }
                let mut message = "Expected property name or '}'";
                loop {
                    self.skip_ws();
                    if self.peek() != Some(u16::from(b'"')) {
                        return Err(self.at(message));
                    }
                    let key = self.string()?;
                    self.expect(b':', "Expected ':' after property name")?;
                    let value = self.value(depth + 1)?;
                    map.insert(key, value);
                    self.skip_ws();
                    if self.peek() == Some(u16::from(b',')) {
                        self.pos += 1;
                        message = "Expected double-quoted property name";
                        continue;
                    }
                    self.expect(b'}', "Expected ',' or '}' after property value")?;
                    return Ok(Value::Object(map));
                }
            }
            _ => Err(self.unexpected()),
        }
    }
}

/// `JSON.parse(text)`, failing with the message V8 gives.
pub fn parse(text: &str) -> Result<Value, String> {
    let src: Vec<u16> = text.encode_utf16().collect();
    let mut p = Parser { src: &src, pos: 0 };
    let value = p.value(0)?;
    p.skip_ws();
    if p.pos < src.len() {
        return Err(p.at_raw("Unexpected non-whitespace character after JSON"));
    }
    Ok(value)
}

/// A JSON string literal, as `JSON.stringify` writes it.
pub fn escape(s: &str, out: &mut String) {
    out.push('"');
    let bytes = s.as_bytes();
    let mut start = 0;
    for (i, &b) in bytes.iter().enumerate() {
        let replacement = match b {
            b'"' => "\\\"",
            b'\\' => "\\\\",
            b'\n' => "\\n",
            b'\r' => "\\r",
            b'\t' => "\\t",
            0x08 => "\\b",
            0x0c => "\\f",
            0..0x20 => "",
            _ => continue,
        };
        out.push_str(&s[start..i]);
        if replacement.is_empty() {
            out.push_str(&format!("\\u{b:04x}"));
        } else {
            out.push_str(replacement);
        }
        start = i + 1;
    }
    out.push_str(&s[start..]);
    out.push('"');
}

fn newline(out: &mut String, indent: usize, level: usize) {
    const SPACES: &str = "                                                                ";
    if indent > 0 {
        out.push('\n');
        let mut n = indent * level;
        while n > 0 {
            let take = n.min(SPACES.len());
            out.push_str(&SPACES[..take]);
            n -= take;
        }
    }
}

fn write_number(x: f64, out: &mut String) {
    use std::fmt::Write as _;
    if !x.is_finite() {
        out.push_str("null");
    } else if x.fract() == 0.0 && x.abs() < 9007199254740992.0 {
        let _ = write!(out, "{}", x as i64);
    } else {
        out.push_str(&number_to_string(x));
    }
}

/// `JSON.stringify` of `v` nested `level` deep, appended to `out`.
pub fn write(v: &Value, indent: usize, level: usize, out: &mut String) {
    match v {
        Value::Null => out.push_str("null"),
        Value::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Value::Number(n) => write_number(n.as_f64().unwrap_or(f64::NAN), out),
        Value::String(s) => escape(s, out),
        Value::Array(items) => {
            if items.is_empty() {
                out.push_str("[]");
                return;
            }
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                newline(out, indent, level + 1);
                write(item, indent, level + 1, out);
            }
            newline(out, indent, level);
            out.push(']');
        }
        Value::Object(map) => write_map(map, indent, level, out),
    }
}

/// `JSON.stringify` of an object, nested `level` deep, appended to `out`.
pub fn write_map(map: &Map<String, Value>, indent: usize, level: usize, out: &mut String) {
    if map.is_empty() {
        out.push_str("{}");
        return;
    }
    out.push('{');
    let entry = |i: usize, key: &str, value: &Value, out: &mut String| {
        if i > 0 {
            out.push(',');
        }
        newline(out, indent, level + 1);
        escape(key, out);
        out.push(':');
        if indent > 0 {
            out.push(' ');
        }
        write(value, indent, level + 1, out);
    };
    if map.keys().any(|k| crate::value::is_array_index(k)) {
        for (i, key) in ordered_keys(map).into_iter().enumerate() {
            entry(i, key, &map[key.as_str()], out);
        }
    } else {
        for (i, (key, value)) in map.iter().enumerate() {
            entry(i, key, value, out);
        }
    }
    newline(out, indent, level);
    out.push('}');
}

/// `JSON.stringify` of an array of `items`, nested `level` deep, appended to `out`.
pub fn write_list<'a>(
    items: impl ExactSizeIterator<Item = &'a Value>,
    indent: usize,
    level: usize,
    out: &mut String,
) {
    if items.len() == 0 {
        out.push_str("[]");
        return;
    }
    out.push('[');
    for (i, item) in items.enumerate() {
        if i > 0 {
            out.push(',');
        }
        newline(out, indent, level + 1);
        write(item, indent, level + 1, out);
    }
    newline(out, indent, level);
    out.push(']');
}

/// `JSON.stringify(value, null, indent)`.
pub fn stringify(v: &Value, indent: usize) -> String {
    let mut out = String::with_capacity(1024);
    write(v, indent, 0, &mut out);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_errors_match_v8() {
        let cases = [
            (
                "{",
                "Expected property name or '}' in JSON at position 1 (line 1 column 2)",
            ),
            ("[", "Unexpected end of JSON input"),
            (
                "[1",
                "Expected ',' or ']' after array element in JSON at position 2 (line 1 column 3)",
            ),
            (
                "{\"a\":1,",
                "Expected double-quoted property name in JSON at position 7 (line 1 column 8)",
            ),
            ("tx", "Unexpected token 'x', \"tx\" is not valid JSON"),
            ("nul", "Unexpected end of JSON input"),
            (
                "01",
                "Unexpected number in JSON at position 1 (line 1 column 2)",
            ),
            (
                "1.e5",
                "Unterminated fractional number in JSON at position 2 (line 1 column 3)",
            ),
            (
                "aaaaaaaaaaaaaaaaaaaaa",
                "Unexpected token 'a', \"aaaaaaaaaa\"... is not valid JSON",
            ),
            (
                "\r\n\r\n1 x",
                "Unexpected non-whitespace character after JSON at position 6 (line 3 column 3)",
            ),
            ("NaN", "\"NaN\" is not valid JSON"),
            (
                "\"\\u00zz\"",
                "Bad Unicode escape in JSON at position 5 (line 1 column 6)",
            ),
        ];
        for (text, message) in cases {
            assert_eq!(parse(text).unwrap_err(), message, "{text:?}");
        }
    }

    #[test]
    fn stringify_matches_javascript() {
        let v = parse(r#"{"b":[1,{"10":0.1,"2":1e21}],"a":{},"c":[],"s":"\u0001\n"}"#).unwrap();
        assert_eq!(
            stringify(&v, 2),
            "{\n  \"b\": [\n    1,\n    {\n      \"2\": 1e+21,\n      \"10\": 0.1\n    }\n  ],\n  \"a\": {},\n  \"c\": [],\n  \"s\": \"\\u0001\\n\"\n}"
        );
        assert_eq!(
            stringify(&parse("9007199254740993").unwrap(), 2),
            "9007199254740992"
        );
    }
}
