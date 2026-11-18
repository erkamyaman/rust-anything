//! sirv 3.0.2 (MIT) in dev mode, as json-server's `createApp` mounts it: files under a folder, looked up on
//! every request, with `Cache-Control: no-store` and byte ranges.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::mime;

/// A file's headers, as sirv computes them.
pub type Headers = Vec<(String, String)>;

/// What sirv answers: the status, the headers it sets, the body.
#[derive(Debug, Clone, PartialEq)]
pub struct Served {
    pub status: u16,
    pub headers: Headers,
    pub body: Vec<u8>,
    /// `res.writeHead` was called: Node adds no Content-Length of its own.
    pub head_written: bool,
}

/// A folder sirv serves, as an absolute path ending in `/`.
#[derive(Debug, Clone)]
pub struct Folder {
    root: String,
}

/// Node's `path.normalize` for POSIX paths.
pub fn normalize(path: &str) -> String {
    if path.is_empty() {
        return ".".into();
    }
    let absolute = path.starts_with('/');
    let trailing = path.ends_with('/');
    let mut parts: Vec<&str> = Vec::new();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                if parts.last().is_some_and(|p| *p != "..") {
                    parts.pop();
                } else if !absolute {
                    parts.push("..");
                }
            }
            s => parts.push(s),
        }
    }
    let mut out = parts.join("/");
    if absolute {
        out.insert(0, '/');
    }
    if out.is_empty() {
        out.push('.');
    }
    if trailing && !out.ends_with('/') {
        out.push('/');
    }
    out
}

impl Folder {
    /// `sirv(dir)`: the folder resolved against the working directory.
    pub fn new(dir: &Path) -> Folder {
        let abs = if dir.is_absolute() {
            dir.to_path_buf()
        } else {
            std::env::current_dir()
                .unwrap_or_else(|_| PathBuf::from("/"))
                .join(dir)
        };
        let mut root = normalize(&abs.to_string_lossy());
        if root.len() > 1 && root.ends_with('/') {
            root.pop();
        }
        root.push('/');
        Folder { root }
    }

    fn candidates(uri: &str) -> Vec<String> {
        let uri = uri.strip_suffix('/').unwrap_or(uri);
        let index = format!("{uri}/index");
        let mut out = Vec::new();
        for ext in ["", ".html", ".htm"] {
            if !uri.is_empty() {
                out.push(format!("{uri}{ext}"));
            }
            out.push(format!("{index}{ext}"));
        }
        out
    }

    /// The file a pathname names, with its sirv headers.
    pub fn lookup(&self, pathname: &str) -> Option<(PathBuf, Headers, u64)> {
        for name in Self::candidates(pathname) {
            let abs = normalize(&format!("{}{}", self.root, name));
            if !abs.starts_with(&self.root) || abs.contains('\0') {
                continue;
            }
            let Ok(meta) = std::fs::metadata(&abs) else {
                continue;
            };
            if meta.is_dir() {
                continue;
            }
            let encoding = match name.get(name.len().saturating_sub(3)..) {
                Some(".br") => Some("br"),
                Some(".gz") => Some("gzip"),
                _ => None,
            };
            let typed = if encoding.is_some() {
                &name[..name.len() - 3]
            } else {
                name.as_str()
            };
            let mut ctype = content_type(typed).unwrap_or("").to_string();
            if ctype == "text/html" {
                ctype += ";charset=utf-8";
            }
            let modified = meta.modified().map(http_date).unwrap_or_default();
            let mut headers = vec![
                ("Content-Length".to_string(), meta.len().to_string()),
                ("Content-Type".to_string(), ctype),
                ("Last-Modified".to_string(), modified),
            ];
            if let Some(e) = encoding {
                headers.push(("Content-Encoding".into(), e.into()));
            }
            headers.push(("Cache-Control".into(), "no-store".into()));
            return Some((PathBuf::from(abs), headers, meta.len()));
        }
        None
    }
}

/// mrmime's `lookup`: the type of the text after the last dot (or of the whole name).
pub fn content_type(name: &str) -> Option<&'static str> {
    let lower = crate::number::trim(name).to_lowercase();
    let ext = match lower.rfind('.') {
        Some(i) => &lower[i + 1..],
        None => lower.as_str(),
    };
    mime::lookup(ext)
}

/// `Date.prototype.toUTCString()`.
pub fn http_date(t: SystemTime) -> String {
    let secs = match t.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(d) => d.as_secs() as i64,
        Err(e) => -(e.duration().as_secs() as i64),
    };
    let days = secs.div_euclid(86_400);
    let rem = secs.rem_euclid(86_400);
    let (h, m, s) = (rem / 3600, rem % 3600 / 60, rem % 60);
    let weekday = ["Thu", "Fri", "Sat", "Sun", "Mon", "Tue", "Wed"][days.rem_euclid(7) as usize];
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    let names = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!(
        "{weekday}, {d:02} {} {year} {h:02}:{m:02}:{s:02} GMT",
        names[(month - 1) as usize]
    )
}

/// sirv's `send`: the whole file, or the range a `Range` header asks for.
pub fn send(
    file: &Path,
    mut headers: Headers,
    size: u64,
    range: Option<&str>,
) -> Result<Served, String> {
    let mut code = 200;
    let mut span: Option<(u64, u64)> = None;
    if let Some(range) = range {
        code = 206;
        let spec = range.replacen("bytes=", "", 1);
        let mut parts = spec.split('-');
        let x = parts.next().unwrap_or("");
        let y = parts.next();
        let int = |s: Option<&str>| {
            let v = crate::number::parse_int(s.unwrap_or(""));
            (v.is_finite() && v != 0.0).then_some(v)
        };
        let mut end = int(y).unwrap_or(size as f64 - 1.0);
        let start = int(Some(x)).unwrap_or(0.0);
        if end >= size as f64 {
            end = size as f64 - 1.0;
        }
        if start >= size as f64 {
            return Ok(Served {
                status: 416,
                headers: vec![("Content-Range".into(), format!("bytes */{size}"))],
                body: Vec::new(),
                head_written: false,
            });
        }
        let set = |headers: &mut Headers, k: &str, v: String| match headers
            .iter_mut()
            .find(|(n, _)| n.eq_ignore_ascii_case(k))
        {
            Some(slot) => slot.1 = v,
            None => headers.push((k.to_string(), v)),
        };
        let n = crate::number::number_to_string;
        set(
            &mut headers,
            "Content-Range",
            format!("bytes {}-{}/{size}", n(start), n(end)),
        );
        set(&mut headers, "Content-Length", n(end - start + 1.0));
        set(&mut headers, "Accept-Ranges", "bytes".to_string());
        span = Some((start.max(0.0) as u64, end.max(0.0) as u64));
    }
    let bytes = std::fs::read(file).map_err(|e| e.to_string())?;
    let body = match span {
        Some((s, e)) => {
            let s = (s as usize).min(bytes.len());
            let e = ((e as usize) + 1).min(bytes.len()).max(s);
            bytes[s..e].to_vec()
        }
        None => bytes,
    };
    Ok(Served {
        status: code,
        headers,
        body,
        head_written: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_and_dates_follow_node() {
        assert_eq!(normalize("/a/public//../b/./c"), "/a/b/c");
        assert_eq!(normalize("/a/public/../../.."), "/");
        assert_eq!(
            http_date(SystemTime::UNIX_EPOCH + std::time::Duration::from_secs(1_794_918_649)),
            "Tue, 17 Nov 2026 12:30:49 GMT"
        );
        assert_eq!(content_type("/test.HTML"), Some("text/html"));
        assert_eq!(content_type("/test"), None);
        assert_eq!(
            Folder::candidates("/"),
            ["/index", "/index.html", "/index.htm"]
        );
    }
}
