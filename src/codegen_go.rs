//! Go: the cgo wrapper package (the original API over the Rust library), and the driver that
//! calls a package's functions by name for record, diff and bench.

use std::fmt::Write as _;
use std::path::Path;

use crate::bridge::Manifest;
use crate::ts::exports::ExportedFn;

/// The result types of a Go function, without the trailing `error`, and whether it has one.
pub fn results(f: &ExportedFn) -> (Vec<String>, bool) {
    let text = f.native_return.as_deref().unwrap_or("()");
    let inner = text.trim().trim_start_matches('(').trim_end_matches(')');
    let mut parts = split_top(inner);
    let fallible = parts.last().is_some_and(|p| p == "error");
    if fallible {
        parts.pop();
    }
    (parts, fallible)
}

/// The port type of a Go type whose range `i64` cannot hold (`uint64`, `uint`, `uintptr`), alone
/// or as the items of a slice; `None` for every other type.
pub fn unsigned_port_type(go: &str) -> Option<String> {
    match go.trim() {
        "uint64" | "uint" => Some("u64".into()),
        "uintptr" => Some("usize".into()),
        go => go
            .strip_prefix("[]")
            .and_then(unsigned_port_type)
            .map(|item| format!("Vec<{item}>")),
    }
}

fn split_top(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut current = String::new();
    for c in text.chars() {
        match c {
            '[' | '(' | '{' => depth += 1,
            ']' | ')' | '}' => depth -= 1,
            ',' if depth == 0 => {
                out.push(current.trim().to_string());
                current.clear();
                continue;
            }
            _ => {}
        }
        current.push(c);
    }
    if !current.trim().is_empty() {
        out.push(current.trim().to_string());
    }
    out
}

/// Exported identifiers in a Go type: the package's own named types.
fn local_types(t: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut word = String::new();
    let mut prev = ' ';
    for c in t.chars().chain(std::iter::once(' ')) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if !word.is_empty() {
            if word.starts_with(char::is_uppercase) && prev != '.' {
                out.push(std::mem::take(&mut word));
            }
            word.clear();
        }
        prev = c;
    }
    out
}

/// `t` with the package's named types qualified as `pkg.Name`.
fn qualify(t: &str) -> String {
    let mut out = String::new();
    let mut word = String::new();
    let mut prev = ' ';
    for c in t.chars().chain(std::iter::once('\0')) {
        if c.is_alphanumeric() || c == '_' {
            word.push(c);
            continue;
        }
        if !word.is_empty() {
            if word.starts_with(char::is_uppercase) && prev != '.' {
                out.push_str("pkg.");
            }
            out.push_str(&word);
            word.clear();
        }
        if c != '\0' {
            out.push(c);
        }
        prev = c;
    }
    out
}

/// Every named type the exported signatures use.
pub fn used_types(m: &Manifest) -> Vec<String> {
    let mut names: Vec<String> = m
        .exports
        .iter()
        .flat_map(|f| {
            f.params
                .iter()
                .filter_map(|p| p.native_type.clone())
                .chain(results(f).0)
                .collect::<Vec<_>>()
        })
        .flat_map(|t| local_types(&t))
        .collect();
    names.sort();
    names.dedup();
    names
}

/// The Go sources of the unit (not tests).
pub fn unit_sources(m: &Manifest) -> Vec<String> {
    let dir = m.root.join(m.unit.trim_end_matches('/'));
    let dir = if dir.is_file() {
        dir.parent().map(Path::to_path_buf).unwrap_or(dir)
    } else {
        dir
    };
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "go") && !p.to_string_lossy().ends_with("_test.go")
        })
        .collect();
    files.sort();
    files
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .collect()
}

pub fn package_name(m: &Manifest) -> String {
    unit_sources(m)
        .iter()
        .find_map(|s| crate::lang::go::package_name(s))
        .unwrap_or_else(|| "port".into())
}

/// The Go sources of the unit selected for the active Go build target (excludes other-OS/arch
/// files such as `errors_windows.go` on Linux so generated sentinel references always compile).
fn sentinel_sources(m: &Manifest) -> Vec<String> {
    let dir = m.root.join(m.unit.trim_end_matches('/'));
    let dir = if dir.is_file() {
        dir.parent().map(Path::to_path_buf).unwrap_or(dir)
    } else {
        dir
    };
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().is_some_and(|e| e == "go")
                && !p.to_string_lossy().ends_with("_test.go")
                && go_file_matches_host(p)
        })
        .collect();
    files.sort();
    files
        .iter()
        .filter_map(|p| std::fs::read_to_string(p).ok())
        .filter(|src| go_build_tag_matches_host(src))
        .collect()
}

fn host_goos() -> &'static str {
    match std::env::consts::OS {
        "macos" => "darwin",
        other => other,
    }
}

fn host_goarch() -> &'static str {
    match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        "x86" => "386",
        other => other,
    }
}

fn go_file_matches_host(path: &Path) -> bool {
    const KNOWN_GOOS: &[&str] = &[
        "aix",
        "android",
        "darwin",
        "dragonfly",
        "freebsd",
        "hurd",
        "illumos",
        "ios",
        "js",
        "linux",
        "netbsd",
        "openbsd",
        "plan9",
        "solaris",
        "wasip1",
        "windows",
    ];
    const KNOWN_GOARCH: &[&str] = &[
        "386", "amd64", "arm", "arm64", "loong64", "mips", "mips64", "mips64le", "mipsle", "ppc64",
        "ppc64le", "riscv64", "s390x", "wasm",
    ];
    let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
        return true;
    };
    let mut parts: Vec<&str> = stem.split('_').collect();
    if parts.len() < 2 {
        return true;
    }
    if let Some(&last) = parts.last()
        && KNOWN_GOARCH.contains(&last)
    {
        if last != host_goarch() {
            return false;
        }
        parts.pop();
    }
    if parts.len() >= 2
        && let Some(&last) = parts.last()
        && KNOWN_GOOS.contains(&last)
        && last != host_goos()
    {
        return false;
    }
    true
}

fn go_build_tag_matches_host(source: &str) -> bool {
    let host_os = host_goos();
    let is_unix = matches!(
        host_os,
        "darwin" | "dragonfly" | "freebsd" | "linux" | "netbsd" | "openbsd" | "solaris"
    );
    for line in source.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("package ") {
            break;
        }
        if let Some(expr) = trimmed.strip_prefix("//go:build ") {
            let expr = expr.trim();
            if matches!(
                expr,
                "linux" | "windows" | "darwin" | "freebsd" | "openbsd" | "netbsd" | "plan9"
            ) {
                return expr == host_os;
            }
            if expr == "unix" {
                return is_unix;
            }
            if let Some(neg) = expr.strip_prefix('!') {
                let neg = neg.trim();
                if matches!(
                    neg,
                    "linux" | "windows" | "darwin" | "freebsd" | "openbsd" | "netbsd"
                ) {
                    return neg != host_os;
                }
                if neg == "unix" {
                    return !is_unix;
                }
            }
        }
    }
    true
}

/// Re-declares exported sentinel errors and emits `rustAnythingToErr` so `errors.Is` and `==`
/// match the package's own error values.
fn emit_sentinel_lookup(out: &mut String, sentinels: &[crate::lang::go::SentinelError]) {
    if sentinels.is_empty() {
        out.push_str("\nfunc rustAnythingToErr(msg string) error {\n\treturn errors.New(msg)\n}\n");
        return;
    }
    out.push('\n');
    for s in sentinels {
        if let Some(target) = &s.alias_of {
            let _ = writeln!(out, "var {} = {target}", s.name);
        } else {
            let text = s.message.as_deref().unwrap_or(&s.name);
            let quoted = serde_json::to_string(text).unwrap_or_else(|_| format!("{text:?}"));
            let _ = writeln!(out, "var {} = errors.New({quoted})", s.name);
        }
    }
    out.push_str(
        "\ntype rustAnythingWrappedErr struct {\n\tsentinel error\n\tmsg      string\n}\n\n\
         func (e *rustAnythingWrappedErr) Error() string { return e.msg }\n\
         func (e *rustAnythingWrappedErr) Unwrap() error { return e.sentinel }\n\n\
         type rustAnythingSentinel struct {\n\tname    string\n\ttext    string\n\thasText bool\n\terr     error\n}\n\n\
         var rustAnythingSentinels = []rustAnythingSentinel{\n",
    );
    for s in sentinels {
        let name_q = serde_json::to_string(&s.name).unwrap_or_else(|_| format!("{:?}", s.name));
        let (has_text, text_q) = match &s.message {
            Some(t) => (
                "true",
                serde_json::to_string(t).unwrap_or_else(|_| format!("{t:?}")),
            ),
            None => ("false", "\"\"".into()),
        };
        let _ = writeln!(
            out,
            "\t{{name: {name_q}, text: {text_q}, hasText: {has_text}, err: {}}},",
            s.name
        );
    }
    out.push_str(
        r#"}

func rustAnythingToErr(msg string) error {
	for _, s := range rustAnythingSentinels {
		if msg == s.name || (s.hasText && msg == s.text) {
			return s.err
		}
	}
	for _, s := range rustAnythingSentinels {
		if prefix := s.name + ": "; strings.HasPrefix(msg, prefix) {
			rest := strings.TrimPrefix(msg, prefix)
			if !s.hasText {
				return &rustAnythingWrappedErr{sentinel: s.err, msg: rest}
			}
			return fmt.Errorf("%w: %s", s.err, rest)
		}
		if s.hasText && s.text != "" {
			if prefix := s.text + ": "; strings.HasPrefix(msg, prefix) {
				return fmt.Errorf("%w: %s", s.err, strings.TrimPrefix(msg, prefix))
			}
			if suffix := ": " + s.text; strings.HasSuffix(msg, suffix) {
				return fmt.Errorf("%s: %w", strings.TrimSuffix(msg, suffix), s.err)
			}
		}
	}
	return errors.New(msg)
}
"#,
    );
}

/// JSON helpers shared by the wrapper and the driver.
const HELPERS_GO: &str = r#"
// rustAnythingDecode decodes JSON into target; floats may be {"$num": "NaN" | "Infinity" | ...}.
func rustAnythingDecode(raw json.RawMessage, target any) error {
	if f, ok := target.(*float64); ok && len(raw) > 0 && raw[0] == '{' {
		var marker map[string]string
		if err := json.Unmarshal(raw, &marker); err != nil {
			return err
		}
		switch marker["$num"] {
		case "NaN":
			*f = math.NaN()
		case "Infinity":
			*f = math.Inf(1)
		case "-Infinity":
			*f = math.Inf(-1)
		case "-0":
			*f = math.Copysign(0, -1)
		default:
			return errors.New("not a number")
		}
		return nil
	}
	if len(raw) == 0 {
		raw = json.RawMessage("null")
	}
	return json.Unmarshal(raw, target)
}

// rustAnythingEncode marks the floats JSON cannot carry.
func rustAnythingEncode(v any) any {
	var f float64
	switch x := v.(type) {
	case float64:
		f = x
	case float32:
		f = float64(x)
	default:
		return v
	}
	switch {
	case math.IsNaN(f):
		return map[string]string{"$num": "NaN"}
	case math.IsInf(f, 1):
		return map[string]string{"$num": "Infinity"}
	case math.IsInf(f, -1):
		return map[string]string{"$num": "-Infinity"}
	case f == 0 && math.Signbit(f):
		return map[string]string{"$num": "-0"}
	}
	return v
}
"#;

/// The wrapper package: the original package's API, each function calling the Rust library.
pub fn wrapper(m: &Manifest, api: &crate::codegen::PortApi) -> String {
    let lib = m.lib_name();
    let sources = unit_sources(m);
    let types = crate::lang::go::type_decls(&sources, &used_types(m));
    let sentinels = crate::lang::go::sentinel_errors(&sentinel_sources(m));
    let extra_imports = if sentinels.is_empty() {
        ""
    } else {
        "\t\"fmt\"\n\t\"strings\"\n"
    };
    let mut out = format!(
        "// Package {pkg} is the Rust port of `{unit}`, generated by rust-anything: the same API, each\n\
         // function calling the Rust library. Regenerate with `rust-anything refresh`.\n\
         package {pkg}\n\n\
         /*\n#cgo LDFLAGS: -L${{SRCDIR}}/../target/release -l{lib} -Wl,-rpath,${{SRCDIR}}/../target/release\n\
         #include <stdbool.h>\n#include <stdint.h>\n#include <stdlib.h>\n\
         char *rust_anything_call(const char *name, const char *args);\n\
         void rust_anything_free(char *ptr);\n{typed_c}*/\nimport \"C\"\n\n\
         import (\n\t\"encoding/binary\"\n\t\"encoding/json\"\n\t\"errors\"\n{extra_imports}\t\"math\"\n\t\"runtime\"\n\t\"unsafe\"\n)\n\nvar _ = binary.LittleEndian\nvar _ = runtime.KeepAlive\n",
        pkg = package_name(m),
        unit = m.unit,
        typed_c = typed_prototypes(m, api),
    );
    for t in types {
        let _ = write!(out, "\n{t}\n");
    }
    emit_sentinel_lookup(&mut out, &sentinels);
    out.push_str(HELPERS_GO);
    out.push_str(
        r#"
type rustAnythingAnswer struct {
	Ok    json.RawMessage `json:"ok"`
	Error *string         `json:"error"`
	Panic *string         `json:"panic"`
}

func rustAnythingCall(name string, args ...any) (json.RawMessage, error) {
	for i, a := range args {
		args[i] = rustAnythingEncode(a)
	}
	encoded, err := json.Marshal(args)
	if err != nil {
		panic(err)
	}
	cName := C.CString(name)
	defer C.free(unsafe.Pointer(cName))
	cArgs := C.CString(string(encoded))
	defer C.free(unsafe.Pointer(cArgs))
	answer := C.rust_anything_call(cName, cArgs)
	defer C.rust_anything_free(answer)
	var a rustAnythingAnswer
	if err := json.Unmarshal([]byte(C.GoString(answer)), &a); err != nil {
		panic(err)
	}
	if a.Panic != nil {
		panic(*a.Panic)
	}
	if a.Error != nil {
		return nil, rustAnythingToErr(*a.Error)
	}
	return a.Ok, nil
}

func rustAnythingMust(err error) {
	if err != nil {
		panic(err)
	}
}
"#,
    );
    let typed_fns: Vec<&str> = api
        .functions
        .iter()
        .filter(|f| typed(m, f).is_some())
        .map(|f| f.js_name.as_str())
        .collect();
    let wired = wire_fns(m, api);
    if !typed_fns.is_empty() || !wired.is_empty() || !m.batches.is_empty() {
        out.push_str(TYPED_HELPERS_GO);
    }
    if !wired.is_empty() {
        out.push_str(WIRE_HELPERS_GO);
    }
    for f in &m.exports {
        if typed_fns.contains(&f.name.as_str()) {
            out.push_str(&typed_go(f));
            continue;
        }
        if let Some((e, params, result, _)) = wired.iter().find(|(e, ..)| e.name == f.name) {
            out.push_str(&wire_go(e, params, result.as_ref()));
            continue;
        }
        let (results, fallible) = results(f);
        let params = wrapper_params(f);
        let mut all_results = results.clone();
        if fallible {
            all_results.push("error".into());
        }
        let ret = match all_results.len() {
            0 => String::new(),
            1 => format!(" {}", all_results[0]),
            _ => format!(" ({})", all_results.join(", ")),
        };
        let args: Vec<String> = (0..f.params.len()).map(wrapper_arg).collect();
        let call_args = if args.is_empty() {
            String::new()
        } else {
            format!(", {}", args.join(", "))
        };
        let _ = writeln!(
            out,
            "\n// {} is the Rust port of `{}` (`{}:{}`).\nfunc {}({}){ret} {{",
            f.name,
            f.prototype.as_deref().unwrap_or(&f.name),
            f.file,
            f.line,
            f.name,
            params
        );
        let names: Vec<String> = (0..results.len()).map(|i| format!("r{i}")).collect();
        let decls: String = results
            .iter()
            .enumerate()
            .map(|(i, t)| format!("\tvar r{i} {t}\n"))
            .collect();
        let raw = if results.is_empty() { "_" } else { "raw" };
        let _ = writeln!(
            out,
            "\t{raw}, err := rustAnythingCall(\"{}\"{call_args})",
            f.name
        );
        if fallible {
            let _ = writeln!(out, "\tif err != nil {{\n{decls}\t\treturn {}\n\t}}", {
                let mut r = names.clone();
                r.push("err".into());
                r.join(", ")
            });
        } else {
            out.push_str("\trustAnythingMust(err)\n");
        }
        out.push_str(&decls);
        match results.len() {
            0 => {}
            1 => out.push_str("\trustAnythingMust(rustAnythingDecode(raw, &r0))\n"),
            n => {
                let _ = writeln!(
                    out,
                    "\tvar parts []json.RawMessage\n\trustAnythingMust(json.Unmarshal(raw, &parts))\n\
                     \tif len(parts) != {n} {{\n\t\tpanic(\"rust-anything: expected {n} results\")\n\t}}"
                );
                for i in 0..n {
                    let _ = writeln!(
                        out,
                        "\trustAnythingMust(rustAnythingDecode(parts[{i}], &r{i}))"
                    );
                }
            }
        }
        let mut r = names;
        if fallible {
            r.push("nil".into());
        }
        if r.is_empty() {
            out.push_str("}\n");
        } else {
            let _ = writeln!(out, "\treturn {}\n}}", r.join(", "));
        }
    }
    // Validated with the Rust side in `ffi_bindings`.
    out.push_str(&crate::codegen_batch_native::go_wrapper(m, api).unwrap_or_default());
    out
}

// ---- typed calls: numbers, booleans and strings cross cgo without JSON ----------------------

/// A Go type that crosses cgo directly: its C type, the port type it must match, the Rust
/// parameter type, and how Go converts it back from C.
pub(crate) struct Typed {
    pub c: &'static str,
    pub port: &'static str,
    pub rust: &'static str,
}

pub(crate) fn typed_type(go: &str) -> Option<Typed> {
    Some(match go.trim() {
        "int" | "int8" | "int16" | "int32" | "int64" | "uint8" | "uint16" | "uint32" | "byte"
        | "rune" => Typed {
            c: "int64_t",
            port: "i64",
            rust: "i64",
        },
        "float64" | "float32" => Typed {
            c: "double",
            port: "f64",
            rust: "f64",
        },
        "bool" => Typed {
            c: "bool",
            port: "bool",
            rust: "bool",
        },
        "string" => Typed {
            c: "string",
            port: "String",
            rust: "String",
        },
        _ => return None,
    })
}

/// Whether a function crosses as a typed cgo call: typed parameters, and no result, one typed
/// result, or one with a trailing `error`; the port kept the matching Rust types.
fn typed<'m>(m: &'m Manifest, f: &crate::codegen::PortFn) -> Option<&'m ExportedFn> {
    let e = m.exports.iter().find(|e| e.name == f.js_name)?;
    if e.params.len() != f.params.len() {
        return None;
    }
    for (p, (_, port)) in e.params.iter().zip(&f.params) {
        if typed_type(p.native_type.as_deref()?)?.port != port {
            return None;
        }
    }
    let (results, _) = results(e);
    match results.as_slice() {
        [] => (f.ret == "()").then_some(e),
        [r] => (typed_type(r)?.port == f.ret).then_some(e),
        _ => None,
    }
}

/// C prototypes of the typed functions, for the cgo preamble.
fn typed_prototypes(m: &Manifest, api: &crate::codegen::PortApi) -> String {
    let mut out = String::new();
    for f in &api.functions {
        let Some(e) = typed(m, f) else { continue };
        let mut params: Vec<String> = Vec::new();
        for (index, p) in e.params.iter().enumerate() {
            let name = format!("ra_arg{index}");
            let t = typed_type(p.native_type.as_deref().unwrap_or_default()).expect("typed");
            if t.c == "string" {
                params.push(format!("const uint8_t *{name}_ptr, size_t {name}_len"));
            } else {
                params.push(format!("{} {name}", t.c));
            }
        }
        match results(e).0.first().and_then(|r| typed_type(r)) {
            Some(t) if t.c == "string" => {
                params.push("uint8_t *out, size_t out_cap, size_t *out_len".into())
            }
            Some(t) => params.push(format!("{} *out", t.c)),
            None => {}
        }
        params.push("char **err".into());
        let _ = writeln!(
            out,
            "int rust_anything_t_{}({});",
            f.js_name,
            params.join(", ")
        );
    }
    out.push_str(&crate::codegen_batch_native::go_prototypes(m, api));
    for (e, ..) in wire_fns(m, api) {
        let _ = writeln!(
            out,
            "int rust_anything_w_{}(const uint8_t *in, size_t in_len, uint8_t *out, size_t out_cap, size_t *out_len, char **err);",
            e.name
        );
    }
    out
}

const TYPED_HELPERS_GO: &str = r#"
// rustAnythingStr is a string's bytes for C, without copying; nil for "".
func rustAnythingStr(s string) *C.uint8_t {
	if len(s) == 0 {
		return nil
	}
	return (*C.uint8_t)(unsafe.Pointer(unsafe.StringData(s)))
}

// rustAnythingErr takes the error message the Rust side allocated.
func rustAnythingErr(p *C.char) string {
	msg := C.GoString(p)
	C.rust_anything_free(p)
	return msg
}
"#;

fn wrapper_arg(index: usize) -> String {
    format!("raArg{index}")
}

fn wrapper_params(export: &ExportedFn) -> String {
    export
        .params
        .iter()
        .enumerate()
        .map(|(index, param)| {
            format!(
                "{} {}",
                wrapper_arg(index),
                param.native_type.as_deref().unwrap_or("any")
            )
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// The Go wrapper function of a typed export: one cgo call, retried once with a bigger buffer
/// when a string result does not fit.
fn typed_go(e: &ExportedFn) -> String {
    let (results, fallible) = results(e);
    let result = results.first().cloned();
    let params = wrapper_params(e);
    let mut sig_results: Vec<String> = result.iter().cloned().collect();
    if fallible {
        sig_results.push("error".into());
    }
    let ret = match sig_results.len() {
        0 => String::new(),
        1 => format!(" {}", sig_results[0]),
        _ => format!(" ({})", sig_results.join(", ")),
    };
    let mut args: Vec<String> = Vec::new();
    for (index, p) in e.params.iter().enumerate() {
        let name = wrapper_arg(index);
        let t = typed_type(p.native_type.as_deref().unwrap_or_default()).expect("typed");
        match t.c {
            "string" => {
                args.push(format!("rustAnythingStr({name})"));
                args.push(format!("C.size_t(len({name}))"));
            }
            "int64_t" => args.push(format!("C.int64_t({name})")),
            "double" => args.push(format!("C.double({name})")),
            _ => args.push(format!("C.bool({name})")),
        }
    }
    let rt = result.as_deref().and_then(typed_type);
    let mut body = String::new();
    let zero = match (&result, rt.as_ref().map(|t| t.c)) {
        (_, Some("string")) => "\"\"".to_string(),
        (_, Some("bool")) => "false".to_string(),
        (Some(_), _) => "0".to_string(),
        _ => String::new(),
    };
    let fail = |body: &mut String| {
        if fallible {
            let ret = if zero.is_empty() {
                "rustAnythingToErr(rustAnythingErr(errp))".to_string()
            } else {
                format!("{zero}, rustAnythingToErr(rustAnythingErr(errp))")
            };
            let _ = writeln!(body, "\tif status == 1 {{\n\t\treturn {ret}\n\t}}");
        } else {
            let _ = writeln!(
                body,
                "\tif status == 1 {{\n\t\tpanic(rustAnythingErr(errp))\n\t}}"
            );
        }
    };
    body.push_str("\tvar errp *C.char\n");
    let ok_tail = if fallible { ", nil" } else { "" };
    match rt.as_ref().map(|t| t.c) {
        Some("string") => {
            let call = |buf: &str| {
                let mut a = args.clone();
                a.push(format!("(*C.uint8_t)(unsafe.Pointer(&{buf}[0]))"));
                a.push(format!("C.size_t(len({buf}))"));
                a.push("&n".into());
                a.push("&errp".into());
                format!("C.rust_anything_t_{}({})", e.name, a.join(", "))
            };
            let _ = writeln!(
                body,
                "\tvar stack [256]byte\n\tbuf := stack[:]\n\tvar n C.size_t\n\tstatus := {}",
                call("buf")
            );
            let _ = writeln!(
                body,
                "\tif status == 2 {{\n\t\tbuf = make([]byte, int(n))\n\t\tstatus = {}\n\t}}",
                call("buf")
            );
            fail(&mut body);
            let _ = writeln!(body, "\treturn string(buf[:n]){ok_tail}");
        }
        Some(c) => {
            let ty = match c {
                "int64_t" => "C.int64_t",
                "double" => "C.double",
                _ => "C.bool",
            };
            let mut a = args.clone();
            a.push("&out".into());
            a.push("&errp".into());
            let _ = writeln!(
                body,
                "\tvar out {ty}\n\tstatus := C.rust_anything_t_{}({})",
                e.name,
                a.join(", ")
            );
            fail(&mut body);
            let _ = writeln!(
                body,
                "\treturn {}(out){ok_tail}",
                result.as_deref().unwrap_or_default()
            );
        }
        None => {
            let mut a = args.clone();
            a.push("&errp".into());
            let _ = writeln!(
                body,
                "\tstatus := C.rust_anything_t_{}({})",
                e.name,
                a.join(", ")
            );
            fail(&mut body);
            if fallible {
                body.push_str("\treturn nil\n");
            }
        }
    }
    format!(
        "\n// {} is the Rust port of `{}` (`{}:{}`): a typed call, no JSON.\nfunc {}({}){ret} {{\n{body}}}\n",
        e.name,
        e.prototype.as_deref().unwrap_or(&e.name),
        e.file,
        e.line,
        e.name,
        params
    )
}

/// The typed `extern "C"` functions for `lib.rs`. Status: 0 ok, 1 error (message in `err`,
/// freed with `rust_anything_free`), 2 the string result needs `*out_len` bytes.
pub fn typed_rs(m: &Manifest, api: &crate::codegen::PortApi) -> String {
    let mut out = String::from(
        r#"
/// The port's result, or its error message; a panic becomes an error.
#[allow(dead_code)]
fn run<T>(f: impl FnOnce() -> Result<T, String>) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)).unwrap_or_else(|panic| {
        let text = panic
            .downcast_ref::<&str>()
            .map(ToString::to_string)
            .or_else(|| panic.downcast_ref::<String>().cloned())
            .unwrap_or_default();
        Err(format!("Rust panic: {text}"))
    })
}

/// A Go string argument.
///
/// # Safety
///
/// `ptr` points at `len` readable bytes, or `len` is 0.
#[allow(dead_code)]
// SAFETY: unsafe to call; callers uphold the contract above.
unsafe fn go_str(ptr: *const u8, len: usize) -> String {
    if len == 0 {
        return String::new();
    }
    // SAFETY: the caller passes a Go string's bytes and its length (see above).
    String::from_utf8_lossy(unsafe { std::slice::from_raw_parts(ptr, len) }).into_owned()
}

/// Reports an error through `err`.
///
/// # Safety
///
/// `err` is a valid pointer to write to.
#[allow(dead_code)]
// SAFETY: unsafe to call; callers uphold the contract above.
unsafe fn fail(err: *mut *mut c_char, message: String) -> i32 {
    let message = CString::new(message.replace('\0', " ")).unwrap_or_default();
    // SAFETY: `err` is a valid out-pointer (see above).
    unsafe { *err = message.into_raw() };
    1
}
"#,
    );
    for f in &api.functions {
        let Some(e) = typed(m, f) else { continue };
        let mut params = Vec::new();
        let mut convert = String::new();
        let mut args = Vec::new();
        for (index, p) in e.params.iter().enumerate() {
            let name = format!("ra_arg{index}");
            let t = typed_type(p.native_type.as_deref().unwrap_or_default()).expect("typed");
            if t.c == "string" {
                params.push(format!("{name}_ptr: *const u8, {name}_len: usize"));
                let _ = writeln!(
                    convert,
                    "    // SAFETY: Go passes the string's bytes and length.\n    let {name} = unsafe {{ go_str({name}_ptr, {name}_len) }};"
                );
            } else {
                params.push(format!("{name}: {}", t.rust));
            }
            args.push(name);
        }
        let rt = results(e).0.first().and_then(|r| typed_type(r));
        match rt.as_ref().map(|t| t.c) {
            Some("string") => {
                params.push("out: *mut u8, out_cap: usize, out_len: *mut usize".into())
            }
            Some(_) => params.push(format!(
                "out: *mut {}",
                rt.as_ref().map_or("i64", |t| t.rust)
            )),
            None => {}
        }
        params.push("err: *mut *mut c_char".into());
        let call = format!("port::{}({})", f.rust_name, args.join(", "));
        let run_call = if args.is_empty() && f.fallible {
            format!("run(port::{})", f.rust_name)
        } else if f.fallible {
            format!("run(|| {call})")
        } else {
            format!("run(|| Ok({call}))")
        };
        let ok = match rt.as_ref().map(|t| t.c) {
            Some("string") => "{\n            let bytes = value.as_bytes();\n            \
                // SAFETY: Go passes a buffer of `out_cap` bytes and a valid `out_len`.\n            \
                unsafe {\n                *out_len = bytes.len();\n                if bytes.len() > out_cap {\n                    \
                return 2;\n                }\n                std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len());\n            }\n            0\n        }".to_string(),
            Some(_) => "{\n            // SAFETY: Go passes a valid `out`.\n            unsafe { *out = value };\n            0\n        }".to_string(),
            None => "{\n            let () = value;\n            0\n        }".to_string(),
        };
        let _ = writeln!(
            out,
            "\n/// `{proto}`: a typed call.\n///\n/// # Safety\n///\n/// Called by the generated Go wrapper with valid pointers.\n\
             // SAFETY: exported unmangled under a name only rust-anything defines.\n#[unsafe(no_mangle)]\n\
             // SAFETY: callers uphold the contract under `# Safety`.\n\
             pub unsafe extern \"C\" fn rust_anything_t_{name}({params}) -> i32 {{\n{convert}    \
             match {run_call} {{\n        Ok(value) => {ok}\n        \
             // SAFETY: Go passes a valid `err`.\n        Err(message) => unsafe {{ fail(err, message) }},\n    }}\n}}",
            proto = e.prototype.as_deref().unwrap_or(&e.name),
            name = f.js_name,
            params = params.join(", "),
        );
    }
    out
}

// ---- the binary wire: lists, maps, structs and several results, without JSON ----------------

use crate::wire::Ty;

/// The wire type of a Go type; `None` for types the wire does not carry.
fn go_ty(go: &str, sources: &[String], depth: usize) -> Option<Ty> {
    let go = go.trim();
    if depth > 8 {
        return None;
    }
    if let Some(t) = typed_type(go) {
        return Some(match t.c {
            "int64_t" => Ty::I64(go.into()),
            "double" => Ty::F64(go.into()),
            "bool" => Ty::Bool,
            _ => Ty::Str,
        });
    }
    if let Some(item) = go.strip_prefix("[]") {
        return Some(Ty::List {
            item: Box::new(go_ty(item, sources, depth + 1)?),
            native: go.into(),
            array: false,
        });
    }
    if let Some(rest) = go.strip_prefix("map[string]") {
        return Some(Ty::Map {
            value: Box::new(go_ty(rest, sources, depth + 1)?),
            native: go.into(),
        });
    }
    if go.starts_with(char::is_uppercase) && !go.contains(['.', '[', '*']) {
        let fields = crate::lang::go::struct_fields(sources, go)?;
        let fields = fields
            .into_iter()
            .map(|(f, t, key)| Some((f, key, go_ty(&t, sources, depth + 1)?)))
            .collect::<Option<Vec<_>>>()?;
        return Some(Ty::Struct {
            name: go.into(),
            fields,
        });
    }
    None
}

/// The parameter and result wire types of an export, when the wire carries all of them.
/// A wire export: the export, its parameter and result wire types, and its Rust code.
type WireFn<'m> = (&'m ExportedFn, Vec<(String, Ty)>, Option<Ty>, String);

/// Wire types of an export's parameters, and of its result.
type WireSig = (Vec<(String, Ty)>, Option<Ty>);

fn wire_sig(e: &ExportedFn, sources: &[String]) -> Option<WireSig> {
    let params = e
        .params
        .iter()
        .map(|p| {
            Some((
                p.name.clone(),
                go_ty(p.native_type.as_deref()?, sources, 0)?,
            ))
        })
        .collect::<Option<Vec<_>>>()?;
    let (results, _) = results(e);
    let result = match results.len() {
        0 => None,
        1 => Some(go_ty(&results[0], sources, 0)?),
        _ => Some(Ty::Tuple(
            results
                .iter()
                .map(|r| go_ty(r, sources, 0))
                .collect::<Option<Vec<_>>>()?,
        )),
    };
    Some((params, result))
}

/// Go statements appending `expr` of type `t` to the byte slice `b`.
fn go_enc(expr: &str, t: &Ty, depth: usize) -> String {
    let v = format!("v{depth}");
    match t {
        Ty::I64(_) => format!("cwb = cwI64(cwb, int64({expr}))\n"),
        Ty::F64(_) => format!("cwb = cwF64(cwb, float64({expr}))\n"),
        Ty::Bool => format!("cwb = cwBool(cwb, {expr})\n"),
        Ty::Str => format!("cwb = cwStr(cwb, {expr})\n"),
        Ty::List { item, .. } => format!(
            "if {expr} == nil {{\ncwb = append(cwb, 0)\n}} else {{\ncwb = append(cwb, 1)\ncwb = cwU32(cwb, len({expr}))\nfor _, {v} := range {expr} {{\n{}}}\n}}\n",
            go_enc(&v, item, depth + 1)
        ),
        Ty::Map { value, .. } => format!(
            "if {expr} == nil {{\ncwb = append(cwb, 0)\n}} else {{\ncwb = append(cwb, 1)\ncwb = cwU32(cwb, len({expr}))\nfor k{depth}, {v} := range {expr} {{\ncwb = cwStr(cwb, k{depth})\n{}}}\n}}\n",
            go_enc(&v, value, depth + 1)
        ),
        Ty::Struct { fields, .. } => {
            let mut out = String::from("cwb = append(cwb, 1)\n");
            for (field, _, ft) in fields {
                out.push_str(&go_enc(&format!("{expr}.{field}"), ft, depth + 1));
            }
            out
        }
        Ty::Opt { inner, .. } => go_enc(expr, inner, depth),
        // Tuples are handled by the caller; JS-only types never come from Go.
        _ => String::new(),
    }
}

/// Go statements decoding a value of type `t` from reader `r` into `target`.
fn go_dec(target: &str, t: &Ty, depth: usize) -> String {
    match t {
        Ty::I64(native) => format!("{target} = {native}(cwr.i64())\n"),
        Ty::F64(native) => format!("{target} = {native}(cwr.f64())\n"),
        Ty::Bool => format!("{target} = cwr.bool()\n"),
        Ty::Str => format!("{target} = cwr.str()\n"),
        Ty::List { item, native, .. } => format!(
            "if cwr.u8() == 1 {{\nn{depth} := cwr.u32()\n{target} = make({native}, n{depth})\nfor i{depth} := range {target} {{\n{}}}\n}}\n",
            go_dec(&format!("{target}[i{depth}]"), item, depth + 1)
        ),
        Ty::Map { value, native } => {
            let item = native.strip_prefix("map[string]").unwrap_or("any");
            format!(
                "if cwr.u8() == 1 {{\nn{depth} := cwr.u32()\n{target} = make({native}, n{depth})\nfor j{depth} := 0; j{depth} < n{depth}; j{depth}++ {{\nk{depth} := cwr.str()\nvar v{depth} {item}\n{}{target}[k{depth}] = v{depth}\n}}\n}}\n",
                go_dec(&format!("v{depth}"), value, depth + 1)
            )
        }
        Ty::Struct { fields, .. } => {
            let mut out = String::from("if cwr.u8() == 1 {\n");
            for (field, _, ft) in fields {
                out.push_str(&go_dec(&format!("{target}.{field}"), ft, depth + 1));
            }
            out.push_str("}\n");
            out
        }
        Ty::Opt { inner, .. } => go_dec(target, inner, depth),
        // Tuples are handled by the caller; JS-only types never come from Go.
        _ => String::new(),
    }
}

const WIRE_HELPERS_GO: &str = r#"
// The binary wire: little endian; strings, slices, maps and structs start with 0 for nil.
func cwU32(b []byte, n int) []byte { return binary.LittleEndian.AppendUint32(b, uint32(n)) }
func cwI64(b []byte, v int64) []byte { return binary.LittleEndian.AppendUint64(b, uint64(v)) }
func cwF64(b []byte, v float64) []byte {
	return binary.LittleEndian.AppendUint64(b, math.Float64bits(v))
}
func cwBool(b []byte, v bool) []byte {
	if v {
		return append(b, 1)
	}
	return append(b, 0)
}
func cwStr(b []byte, s string) []byte {
	b = append(b, 1)
	b = cwU32(b, len(s))
	return append(b, s...)
}

type cwReader struct {
	b []byte
	i int
}

func (r *cwReader) u8() byte {
	v := r.b[r.i]
	r.i++
	return v
}
func (r *cwReader) u32() int {
	v := binary.LittleEndian.Uint32(r.b[r.i:])
	r.i += 4
	return int(v)
}
func (r *cwReader) i64() int64 {
	v := binary.LittleEndian.Uint64(r.b[r.i:])
	r.i += 8
	return int64(v)
}
func (r *cwReader) f64() float64 { return math.Float64frombits(uint64(r.i64())) }
func (r *cwReader) bool() bool   { return r.u8() != 0 }
func (r *cwReader) str() string {
	if r.u8() == 0 {
		return ""
	}
	n := r.u32()
	s := string(r.b[r.i : r.i+n])
	r.i += n
	return s
}

func cwPtr(b []byte) *C.uint8_t {
	if len(b) == 0 {
		return nil
	}
	return (*C.uint8_t)(unsafe.Pointer(&b[0]))
}
"#;

/// The Go wrapper function of a wire export: encode the arguments, one cgo call, decode.
fn wire_go(e: &ExportedFn, params: &[(String, Ty)], result: Option<&Ty>) -> String {
    let (results, fallible) = results(e);
    let sig_params = wrapper_params(e);
    let mut sig_results = results.clone();
    if fallible {
        sig_results.push("error".into());
    }
    let ret = match sig_results.len() {
        0 => String::new(),
        1 => format!(" {}", sig_results[0]),
        _ => format!(" ({})", sig_results.join(", ")),
    };
    let mut body = String::from("\tcwb := make([]byte, 0, 128)\n");
    for (index, (_, t)) in params.iter().enumerate() {
        body.push_str(&go_enc(&wrapper_arg(index), t, 0));
    }
    let names: Vec<String> = (0..results.len()).map(|i| format!("r{i}")).collect();
    for (i, r) in results.iter().enumerate() {
        let _ = writeln!(body, "\tvar r{i} {r}");
    }
    let call = |buf: &str| {
        format!(
            "C.rust_anything_w_{}(cwPtr(cwb), C.size_t(len(cwb)), (*C.uint8_t)(unsafe.Pointer(&{buf}[0])), C.size_t(len({buf})), &n, &errp)",
            e.name
        )
    };
    let _ = writeln!(
        body,
        "\tvar cwStack [512]byte\n\tcwOut := cwStack[:]\n\tvar n C.size_t\n\tvar errp *C.char\n\tstatus := {}\n\
         \tif status == 2 {{\n\t\tcwOut = make([]byte, int(n))\n\t\tstatus = {}\n\t}}",
        call("cwOut"),
        call("cwOut")
    );
    let mut fail_ret: Vec<String> = names.clone();
    if fallible {
        fail_ret.push("rustAnythingToErr(rustAnythingErr(errp))".into());
        let _ = writeln!(
            body,
            "\tif status == 1 {{\n\t\treturn {}\n\t}}",
            fail_ret.join(", ")
        );
    } else {
        body.push_str("\tif status == 1 {\n\t\tpanic(rustAnythingErr(errp))\n\t}\n");
    }
    if !results.is_empty() {
        body.push_str("\tcwr := &cwReader{b: cwOut[:n]}\n");
        match result {
            Some(Ty::Tuple(items)) => {
                for (i, t) in items.iter().enumerate() {
                    body.push_str(&go_dec(&format!("r{i}"), t, 0));
                }
            }
            Some(t) => body.push_str(&go_dec("r0", t, 0)),
            None => {}
        }
    }
    let mut ok = names;
    if fallible {
        ok.push("nil".into());
    }
    if !ok.is_empty() {
        let _ = writeln!(body, "\treturn {}", ok.join(", "));
    }
    format!(
        "\n// {} is the Rust port of `{}` (`{}:{}`): binary wire, no JSON.\nfunc {}({}){ret} {{\n{body}}}\n",
        e.name,
        e.prototype.as_deref().unwrap_or(&e.name),
        e.file,
        e.line,
        e.name,
        sig_params
    )
}

/// The exports that cross the binary wire (not typed, but every type fits the wire and the port).
fn wire_fns<'m>(m: &'m Manifest, api: &crate::codegen::PortApi) -> Vec<WireFn<'m>> {
    let sources = unit_sources(m);
    let mut out = Vec::new();
    for f in &api.functions {
        if typed(m, f).is_some() {
            continue;
        }
        let Some(e) = m.exports.iter().find(|e| e.name == f.js_name) else {
            continue;
        };
        let Some((params, result)) = wire_sig(e, &sources) else {
            continue;
        };
        if params.len() != f.params.len() {
            continue;
        }
        if let Some(code) = wire_rs_fn(f, e, &params, result.as_ref()) {
            out.push((e, params, result, code));
        }
    }
    out
}

/// The Rust `extern "C"` function of a wire export, with its schema statics.
fn wire_rs_fn(
    f: &crate::codegen::PortFn,
    e: &ExportedFn,
    params: &[(String, Ty)],
    result: Option<&Ty>,
) -> Option<String> {
    let mut statics = String::new();
    let prefix = format!("S_{}", f.js_name.to_uppercase());
    let bindings: Vec<_> = f
        .params
        .iter()
        .enumerate()
        .map(|(index, (_, ty))| (format!("ra_arg{index}"), ty.clone()))
        .collect();
    let reads = crate::wire::rust_read_args(params, &bindings, &prefix, &mut statics)?;
    let args: Vec<&str> = bindings.iter().map(|(n, _)| n.as_str()).collect();
    let call = format!("port::{}({})", f.rust_name, args.join(", "));
    let call = if f.fallible { format!("{call}?") } else { call };
    let write = match result {
        None => {
            if f.ret != "()" {
                return None;
            }
            format!("        let () = {call};\n")
        }
        Some(t) => {
            let schema = format!("{prefix}_OUT");
            let w = crate::wire::rust_write(t, &f.ret, "out", &schema, 0)?;
            if w.contains(&schema) {
                let _ = writeln!(
                    statics,
                    "static {schema}: wire::Schema = {};",
                    crate::wire::schema_expr(t)
                );
            }
            format!("        let out = &{call};\n        {w}\n")
        }
    };
    Some(format!(
        "\n{statics}\n/// `{proto}`: binary wire.\n///\n/// # Safety\n///\n/// Called by the generated Go wrapper with valid pointers.\n\
         // SAFETY: exported unmangled under a name only rust-anything defines.\n#[unsafe(no_mangle)]\n\
         // SAFETY: callers uphold the contract under `# Safety`.\n\
         pub unsafe extern \"C\" fn rust_anything_w_{name}(in_ptr: *const u8, in_len: usize, out: *mut u8, out_cap: usize, out_len: *mut usize, err: *mut *mut c_char) -> i32 {{\n    \
         let input: &[u8] = if in_len == 0 {{\n        &[]\n    }} else {{\n        \
         // SAFETY: Go passes its buffer and length.\n        unsafe {{ std::slice::from_raw_parts(in_ptr, in_len) }}\n    }};\n    \
         let result = run(|| {{\n        let r = &mut wire::Reader::new(input);\n{reads}        let w = &mut wire::Writer::default();\n{write}        \
         Ok(std::mem::take(&mut w.0))\n    }});\n    \
         match result {{\n        Ok(bytes) => {{\n            \
         // SAFETY: Go passes a buffer of `out_cap` bytes and a valid `out_len`.\n            unsafe {{\n                \
         *out_len = bytes.len();\n                if bytes.len() > out_cap {{\n                    return 2;\n                }}\n                \
         std::ptr::copy_nonoverlapping(bytes.as_ptr(), out, bytes.len());\n            }}\n            0\n        }}\n        \
         // SAFETY: Go passes a valid `err`.\n        Err(message) => unsafe {{ fail(err, message) }},\n    }}\n}}",
        proto = e.prototype.as_deref().unwrap_or(&e.name),
        name = f.js_name,
    ))
}

/// Rust code of every wire export, for `lib.rs`.
pub fn wire_rs(m: &Manifest, api: &crate::codegen::PortApi) -> String {
    wire_fns(m, api)
        .into_iter()
        .map(|(_, _, _, code)| code)
        .collect()
}

/// A `main` package that calls `import_path`'s functions by name: record, diff and bench.
pub fn driver(m: &Manifest, import_path: &str) -> String {
    let mut cases = String::new();
    let mut prepares = String::new();
    for f in &m.exports {
        let (results, fallible) = results(f);
        let _ = writeln!(cases, "\tcase \"{}\":", f.name);
        let mut args = Vec::new();
        for (i, p) in f.params.iter().enumerate() {
            let t = qualify(p.native_type.as_deref().unwrap_or("any"));
            let _ = writeln!(
                cases,
                "\t\tvar a{i} {t}\n\t\tif rustAnythingDecode(argAt(args, {i}), &a{i}) != nil {{\n\t\t\treturn invalid\n\t\t}}"
            );
            args.push(format!("a{i}"));
        }
        let _ = writeln!(prepares, "\tcase \"{}\":", f.name);
        for (i, p) in f.params.iter().enumerate() {
            let t = qualify(p.native_type.as_deref().unwrap_or("any"));
            let _ = writeln!(
                prepares,
                "\t\tvar a{i} {t}\n\t\tif rustAnythingDecode(argAt(args, {i}), &a{i}) != nil {{\n\t\t\treturn nil\n\t\t}}"
            );
        }
        let _ = writeln!(
            prepares,
            "\t\treturn func() {{ pkg.{}({}) }}",
            f.name,
            args.join(", ")
        );
        let mut lhs: Vec<String> = (0..results.len()).map(|i| format!("r{i}")).collect();
        if fallible {
            lhs.push("err".into());
        }
        let call = format!("pkg.{}({})", f.name, args.join(", "));
        if lhs.is_empty() {
            let _ = writeln!(cases, "\t\t{call}");
        } else {
            let _ = writeln!(cases, "\t\t{} := {call}", lhs.join(", "));
        }
        if fallible {
            cases.push_str(
                "\t\tif err != nil {\n\t\t\tres := map[string]any{\"error\": err.Error()}\n\t\t\tif s := rustAnythingSentinelName(err); s != \"\" {\n\t\t\t\tres[\"sentinel\"] = s\n\t\t\t}\n\t\t\treturn res\n\t\t}\n",
            );
        }
        let ok = match results.len() {
            0 => "nil".to_string(),
            1 => "rustAnythingEncode(r0)".to_string(),
            n => format!(
                "[]any{{{}}}",
                (0..n)
                    .map(|i| format!("rustAnythingEncode(r{i})"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        };
        let _ = writeln!(cases, "\t\treturn map[string]any{{\"ok\": {ok}}}");
    }
    let sentinels = crate::lang::go::sentinel_errors(&sentinel_sources(m));
    let mut sentinel_fn = String::from("func rustAnythingSentinelName(err error) string {\n");
    for s in &sentinels {
        let name_q = serde_json::to_string(&s.name).unwrap_or_else(|_| format!("{:?}", s.name));
        let _ = writeln!(
            sentinel_fn,
            "\tif errors.Is(err, pkg.{}) {{\n\t\treturn {name_q}\n\t}}",
            s.name
        );
    }
    sentinel_fn.push_str("\treturn \"\"\n}\n");
    DRIVER_GO
        .replace("IMPORT_PATH", import_path)
        .replace("// CASES\n", &cases)
        .replace("// PREPARES\n", &prepares)
        .replace("// SENTINELS\n", &sentinel_fn)
        .replace("// HELPERS\n", HELPERS_GO)
}

const DRIVER_GO: &str = r#"// Generated by rust-anything: calls the package's functions by name.
package main

import (
	"bufio"
	"bytes"
	"encoding/json"
	"errors"
	"fmt"
	"math"
	"os"
	"strings"
	"time"

	pkg "IMPORT_PATH"
)

var _ = errors.New

// SENTINELS

// HELPERS

var invalid = map[string]any{"invalid": true}

func argAt(args []json.RawMessage, i int) json.RawMessage {
	if i < len(args) {
		return args[i]
	}
	return nil
}

func call(name string, args []json.RawMessage) (out map[string]any) {
	defer func() {
		if r := recover(); r != nil {
			out = map[string]any{"panic": fmt.Sprint(r)}
		}
	}()
	switch name {
	// CASES
	}
	return map[string]any{"missing": name}
}

// prepare decodes a case's arguments once and returns the bare call; nil if they do not fit.
func prepare(name string, args []json.RawMessage) func() {
	switch name {
	// PREPARES
	}
	return nil
}

type input struct {
	Fn   string            `json:"fn"`
	Args []json.RawMessage `json:"args"`
}

// canon re-encodes a JSON value with exact numbers, so equal values compare equal as text.
func canon(v any) string {
	text, err := json.Marshal(v)
	if err != nil {
		return "unencodable: " + err.Error()
	}
	d := json.NewDecoder(bytes.NewReader(text))
	d.UseNumber()
	var x any
	if err := d.Decode(&x); err != nil {
		return string(text)
	}
	out, _ := json.Marshal(x)
	return string(out)
}

func record(inputs, out string) {
	data, err := os.ReadFile(inputs)
	if err != nil {
		panic(err)
	}
	var cases []input
	if err := json.Unmarshal(data, &cases); err != nil {
		panic(err)
	}
	f, err := os.Create(out)
	if err != nil {
		panic(err)
	}
	defer f.Close()
	w := bufio.NewWriter(f)
	defer w.Flush()
	progress := os.Getenv("RUST_ANYTHING_PROGRESS")
	for index, c := range cases {
		if progress != "" {
			if p, err := os.OpenFile(progress, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0o644); err == nil {
				fmt.Fprintln(p, index)
				p.Close()
			}
		}
		result := call(c.Fn, c.Args)
		if result["invalid"] != nil {
			continue
		}
		line := map[string]any{"fn": c.Fn, "args": c.Args}
		for k, v := range result {
			line[k] = v
		}
		text, err := json.Marshal(line)
		if err != nil {
			continue
		}
		w.Write(text)
		w.WriteByte('\n')
		w.Flush()
	}
}

func readCases(path string) []map[string]json.RawMessage {
	data, err := os.ReadFile(path)
	if err != nil {
		panic(err)
	}
	var out []map[string]json.RawMessage
	for _, line := range bytes.Split(data, []byte("\n")) {
		if len(bytes.TrimSpace(line)) == 0 {
			continue
		}
		var c map[string]json.RawMessage
		if err := json.Unmarshal(line, &c); err != nil {
			panic(err)
		}
		out = append(out, c)
	}
	return out
}

func argsOf(c map[string]json.RawMessage) (string, []json.RawMessage) {
	var name string
	var args []json.RawMessage
	json.Unmarshal(c["fn"], &name)
	json.Unmarshal(c["args"], &args)
	return name, args
}

func diff(cases, out string) {
	summary := map[string]any{"total": 0, "passed": 0, "failed": 0}
	missing := []string{}
	mismatches := []any{}
	total, passed, failed := 0, 0, 0
	for _, c := range readCases(cases) {
		total++
		name, args := argsOf(c)
		got := call(name, args)
		if got["missing"] != nil {
			failed++
			missing = append(missing, name)
			continue
		}
		expected := map[string]any{}
		for _, k := range []string{"ok", "error", "sentinel", "panic"} {
			if raw, ok := c[k]; ok {
				expected[k] = raw
			}
		}
		if _, ok := c["sentinel"]; !ok {
			delete(got, "sentinel")
		}
		same := false
		if p, ok := got["panic"]; ok && strings.Contains(fmt.Sprint(p), "Rust panic:") {
			same = false
		} else if _, ok := expected["panic"]; ok {
			// Panic messages differ between Go and Rust; that it panicked is the behavior.
			_, same = got["panic"]
		} else {
			same = canon(expected) == canon(got)
		}
		if same {
			passed++
		} else {
			failed++
			mismatches = append(mismatches, map[string]any{"fn": name, "args": args, "expected": expected, "actual": got})
		}
	}
	summary["total"], summary["passed"], summary["failed"] = total, passed, failed
	summary["missing"], summary["mismatches"] = missing, mismatches
	text, _ := json.Marshal(summary)
	os.WriteFile(out, text, 0o644)
}

func bench(cases, out string, minMs int) {
	groups := map[string][][]json.RawMessage{}
	order := []string{}
	for _, c := range readCases(cases) {
		name, args := argsOf(c)
		if _, ok := groups[name]; !ok {
			order = append(order, name)
		}
		groups[name] = append(groups[name], args)
	}
	result := map[string]any{}
	for _, name := range order {
		// Arguments are decoded once: only the calls are timed.
		var calls []func()
		for _, args := range groups[name] {
			if f := prepare(name, args); f != nil {
				calls = append(calls, f)
			}
		}
		if len(calls) == 0 {
			continue
		}
		once := func() {
			for _, f := range calls {
				func() {
					defer func() { recover() }()
					f()
				}()
			}
		}
		once()
		passes, start := 0, time.Now()
		for {
			once()
			passes++
			if elapsed := time.Since(start); elapsed.Milliseconds() >= int64(minMs) {
				result[name] = map[string]any{"cases": len(calls), "ns": float64(elapsed.Nanoseconds()) / float64(passes*len(calls))}
				break
			}
		}
	}
	text, _ := json.Marshal(result)
	os.WriteFile(out, text, 0o644)
}

func main() {
	switch os.Args[1] {
	case "record":
		record(os.Args[2], os.Args[3])
	case "diff":
		diff(os.Args[2], os.Args[3])
	case "bench":
		var ms int
		fmt.Sscan(os.Args[4], &ms)
		bench(os.Args[2], os.Args[3], ms)
	}
}
"#;

/// A shadow package with the original's API: each call runs the original and the Rust port
/// (through the wrapper), returns the original's results and logs mismatches.
/// `RUST_ANYTHING_MODE=old` runs the original only, `new` the port only.
pub fn shadow(m: &Manifest, original: &str) -> String {
    let wrapper = crate::harness_go::wrapper_module(m);
    let sentinels = crate::lang::go::sentinel_errors(&sentinel_sources(m));
    let remap_sentinels = !sentinels.is_empty() && m.exports.iter().any(|f| results(f).1);
    let extra_imports = if remap_sentinels {
        "\t\"errors\"\n\t\"fmt\"\n\t\"strings\"\n"
    } else {
        ""
    };
    let mut out = format!(
        "// Package {pkg} runs the Go original and the Rust port of `{unit}` side by side. Generated\n\
         // by rust-anything: import it in place of {original}. RUST_ANYTHING_MODE=old|shadow|new.\n\
         package {pkg}\n\n\
         import (\n\t\"encoding/json\"\n{extra_imports}\t\"log\"\n\t\"os\"\n\t\"reflect\"\n\t\"sync/atomic\"\n\n\
         \torig \"{original}\"\n\tport \"{wrapper}\"\n)\n\n\
         var mode = func() string {{\n\tif m := os.Getenv(\"RUST_ANYTHING_MODE\"); m != \"\" {{\n\t\treturn m\n\t}}\n\treturn \"shadow\"\n}}()\n\n\
         // Calls and Mismatches count shadowed calls.\nvar Calls, Mismatches atomic.Int64\n\n\
         // convert moves a value between the original's and the wrapper's identical types.\n\
         func convert[T any](v any) T {{\n\tvar out T\n\tif text, err := json.Marshal(v); err == nil {{\n\t\t_ = json.Unmarshal(text, &out)\n\t}}\n\treturn out\n}}\n\n\
         func compare(name string, expected, actual []any) {{\n\tCalls.Add(1)\n\tif !reflect.DeepEqual(expected, actual) {{\n\
         \t\tif Mismatches.Add(1) <= 10 {{\n\t\t\tlog.Printf(\"[rust-anything] mismatch in %s: expected %v, got %v\", name, expected, actual)\n\t\t}}\n\t}}\n}}\n",
        pkg = package_name(m),
        unit = m.unit,
    );
    for t in used_types(m) {
        let _ = write!(
            out,
            "\n// {t} is the original's type.\ntype {t} = orig.{t}\n"
        );
    }
    for s in &sentinels {
        let _ = write!(
            out,
            "\n// {} is the original's sentinel error.\nvar {} = orig.{}\n",
            s.name, s.name, s.name
        );
    }
    if remap_sentinels {
        out.push_str(
            "\nfunc remapErr(err error) error {\n\tif err == nil {\n\t\treturn nil\n\t}\n",
        );
        for s in &sentinels {
            let n = &s.name;
            let _ = write!(
                out,
                "\tif err == port.{n} {{\n\t\treturn orig.{n}\n\t}}\n\
                 \tif errors.Is(err, port.{n}) {{\n\
                 \t\tmsg := err.Error()\n\
                 \t\ttext := port.{n}.Error()\n\
                 \t\tif prefix := text + \": \"; strings.HasPrefix(msg, prefix) {{\n\
                 \t\t\treturn fmt.Errorf(\"%w: %s\", orig.{n}, strings.TrimPrefix(msg, prefix))\n\
                 \t\t}}\n\
                 \t\tif suffix := \": \" + text; strings.HasSuffix(msg, suffix) {{\n\
                 \t\t\treturn fmt.Errorf(\"%s: %w\", strings.TrimSuffix(msg, suffix), orig.{n})\n\
                 \t\t}}\n\
                 \t\treturn orig.{n}\n\
                 \t}}\n"
            );
        }
        out.push_str("\treturn err\n}\n");
    }
    for f in &m.exports {
        let (results, fallible) = results(f);
        let params: Vec<String> = f
            .params
            .iter()
            .map(|p| format!("{} {}", p.name, p.native_type.as_deref().unwrap_or("any")))
            .collect();
        let mut all: Vec<String> = results.clone();
        if fallible {
            all.push("error".into());
        }
        let ret = match all.len() {
            0 => String::new(),
            1 => format!(" {}", all[0]),
            _ => format!(" ({})", all.join(", ")),
        };
        let names: Vec<String> = (0..all.len()).map(|i| format!("o{i}")).collect();
        let rust_names: Vec<String> = (0..all.len()).map(|i| format!("n{i}")).collect();
        let args: Vec<&str> = f.params.iter().map(|p| p.name.as_str()).collect();
        let port_args: Vec<String> = f
            .params
            .iter()
            .map(|p| {
                let t = p.native_type.as_deref().unwrap_or("any");
                if local_types(t).is_empty() {
                    p.name.clone()
                } else {
                    format!(
                        "convert[{}]({})",
                        qualify(t).replace("pkg.", "port."),
                        p.name
                    )
                }
            })
            .collect();
        let lhs = |v: &[String]| {
            if v.is_empty() {
                String::new()
            } else {
                format!("{} := ", v.join(", "))
            }
        };
        let back: Vec<String> = all
            .iter()
            .zip(&rust_names)
            .map(|(t, n)| {
                if t == "error" && remap_sentinels {
                    format!("remapErr({n})")
                } else if local_types(t).is_empty() {
                    n.clone()
                } else {
                    format!("convert[{t}]({n})")
                }
            })
            .collect();
        let errors_text = |v: &[String]| -> Vec<String> {
            v.iter()
                .zip(&all)
                .map(|(n, t)| {
                    if t == "error" {
                        format!("errText({n})")
                    } else {
                        n.clone()
                    }
                })
                .collect()
        };
        let _ = writeln!(
            out,
            "\n// {} shadows the original `{}`.\nfunc {}({}){ret} {{",
            f.name,
            f.name,
            f.name,
            params.join(", ")
        );
        let ret_new = if all.is_empty() {
            String::new()
        } else {
            format!("return {}", back.join(", "))
        };
        let _ = writeln!(
            out,
            "\tif mode == \"new\" {{\n\t\t{}port.{}({})\n\t\t{ret_new}\n\t}}",
            lhs(&rust_names),
            f.name,
            port_args.join(", ")
        );
        let _ = writeln!(out, "\t{}orig.{}({})", lhs(&names), f.name, args.join(", "));
        let ret_old = if all.is_empty() {
            "return".to_string()
        } else {
            format!("return {}", names.join(", "))
        };
        let _ = writeln!(
            out,
            "\tif mode != \"old\" {{\n\t\tfunc() {{\n\t\t\tdefer func() {{\n\t\t\t\tif r := recover(); r != nil {{\n\
             \t\t\t\t\tcompare(\"{name}\", []any{{\"no panic\"}}, []any{{r}})\n\t\t\t\t}}\n\t\t\t}}()\n\
             \t\t\t{}port.{name}({})\n\t\t\tcompare(\"{name}\", []any{{{}}}, []any{{{}}})\n\t\t}}()\n\t}}\n\t{ret_old}\n}}",
            lhs(&rust_names),
            port_args.join(", "),
            errors_text(&names).join(", "),
            errors_text(&back).join(", "),
            name = f.name,
        );
    }
    out.push_str("\nfunc errText(err error) any {\n\tif err == nil {\n\t\treturn nil\n\t}\n\treturn err.Error()\n}\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn go_codegen_reserves_its_own_bindings() {
        let exports = crate::lang::go::exports(
            "names.go",
            r#"package names
func Typed(out int, C string) int { return out + len(C) }
func Wire(cwb []int, r []int) []int { return append(cwb, r...) }
func Json(raw uint64) uint64 { return raw }"#,
        );
        let manifest = Manifest {
            schema: String::new(),
            root: std::path::PathBuf::new(),
            root_written: None,
            unit: "names.go".into(),
            language: crate::lang::Lang::Go,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "names-rs".into(),
            exports,
            classes: vec![],
            js_only: vec![],
            batches: vec![],
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        };
        let mut api = crate::codegen::parse_port(&crate::codegen::port_rs(&manifest));
        crate::bridge::match_export_names(&manifest, &mut api);
        let go = wrapper(&manifest, &api);
        assert!(go.contains("func Typed(raArg0 int, raArg1 string) int {"));
        assert!(go.contains("C.rust_anything_t_Typed(C.int64_t(raArg0),"));
        assert!(go.contains("int64_t ra_arg0, const uint8_t *ra_arg1_ptr"));
        assert!(go.contains("func Wire(raArg0 []int, raArg1 []int) []int {"));
        assert!(go.contains("C.rust_anything_w_Wire(cwPtr(cwb)"));
        assert!(go.contains("func Json(raArg0 uint64) uint64 {"));
        assert!(go.contains(r#"raw, err := rustAnythingCall("Json", raArg0)"#));
        let typed = typed_rs(&manifest, &api);
        assert!(typed.contains("port::typed(ra_arg0, ra_arg1)"));
        let wire = wire_rs(&manifest, &api);
        assert!(wire.contains("port::wire(ra_arg0, ra_arg1)"));
    }

    #[test]
    fn qualifies_local_types() {
        assert_eq!(qualify("[]Point"), "[]pkg.Point");
        assert_eq!(qualify("map[string]Point"), "map[string]pkg.Point");
        assert_eq!(qualify("time.Duration"), "time.Duration");
        assert_eq!(local_types("map[string][]Point"), ["Point"]);
    }

    #[test]
    fn maps_sentinel_errors_in_wrapper_driver_and_shadow() {
        let dir =
            std::env::temp_dir().join(format!("rust-anything-go-sentinels-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("store")).unwrap();
        let src = r#"package store

import "errors"

var (
	ErrNotFound = errors.New("not found")
	ErrAlias    error = ErrNotFound
	ErrBadInput = errors.New("bad input")
)

func Lookup(id int) (string, error) {
	if id < 0 {
		return "", ErrBadInput
	}
	return "", ErrNotFound
}
"#;
        std::fs::write(dir.join("store/store.go"), src).unwrap();
        // Platform-specific file for another OS must be excluded from sentinel discovery.
        let other_os = if host_goos() == "windows" {
            "linux"
        } else {
            "windows"
        };
        std::fs::write(
            dir.join(format!("store/errors_{other_os}.go")),
            "package store\nimport \"errors\"\nvar ErrOtherOS = errors.New(\"other\")\n",
        )
        .unwrap();
        let exports = crate::lang::go::exports("store/store.go", src);
        let manifest = Manifest {
            schema: String::new(),
            root: dir.clone(),
            root_written: None,
            unit: "store/".into(),
            language: crate::lang::Lang::Go,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "store-rs".into(),
            exports,
            classes: vec![],
            js_only: vec![],
            batches: vec![],
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        };
        let mut api = crate::codegen::parse_port(&crate::codegen::port_rs(&manifest));
        crate::bridge::match_export_names(&manifest, &mut api);

        let w = wrapper(&manifest, &api);
        assert!(
            w.contains("var ErrNotFound = errors.New(\"not found\")"),
            "{w}"
        );
        assert!(w.contains("var ErrAlias = ErrNotFound"), "{w}");
        assert!(
            w.contains("var ErrBadInput = errors.New(\"bad input\")"),
            "{w}"
        );
        assert!(!w.contains("ErrOtherOS"), "{w}");
        assert!(w.contains("hasText: true"), "{w}");
        assert!(
            w.contains("func rustAnythingToErr(msg string) error"),
            "{w}"
        );
        assert!(w.contains("fmt.Errorf(\"%w: %s\""), "{w}");
        assert!(
            w.contains("rustAnythingToErr(rustAnythingErr(errp))"),
            "{w}"
        );
        assert!(!w.contains("errors.New(rustAnythingErr(errp))"), "{w}");

        let d = driver(&manifest, "example.com/app/store");
        assert!(d.contains("errors.Is(err, pkg.ErrNotFound)"), "{d}");
        assert!(!d.contains("ErrOtherOS"), "{d}");
        assert!(d.contains("res[\"sentinel\"] = s"), "{d}");
        assert!(d.contains("delete(got, \"sentinel\")"), "{d}");

        let s = shadow(&manifest, "example.com/app/store");
        assert!(s.contains("var ErrNotFound = orig.ErrNotFound"), "{s}");
        assert!(s.contains("func remapErr(err error) error"), "{s}");
        assert!(s.contains("remapErr(n1)"), "{s}");

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn diff_handles_legacy_cases_without_sentinel_and_rejects_plain_error() {
        if std::process::Command::new("go")
            .arg("version")
            .output()
            .is_err()
        {
            return;
        }
        let dir = std::env::temp_dir().join(format!(
            "rust-anything-go-sentinel-diff-{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("store")).unwrap();
        std::fs::create_dir_all(dir.join("drv")).unwrap();

        let src = r#"package store

import "errors"

var ErrNotFound = errors.New("not found")

func Lookup(id int) (string, error) {
	if id == 1 {
		return "", ErrNotFound
	}
	return "", errors.New("not found")
}
"#;
        std::fs::write(dir.join("go.mod"), "module example.com/app\n\ngo 1.22\n").unwrap();
        std::fs::write(dir.join("store/store.go"), src).unwrap();

        let exports = crate::lang::go::exports("store/store.go", src);
        let manifest = Manifest {
            schema: String::new(),
            root: dir.clone(),
            root_written: None,
            unit: "store/".into(),
            language: crate::lang::Lang::Go,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "store-rs".into(),
            exports,
            classes: vec![],
            js_only: vec![],
            batches: vec![],
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        };
        let drv = driver(&manifest, "example.com/app/store");
        std::fs::write(dir.join("drv/main.go"), drv).unwrap();

        // 1) Legacy case (no "sentinel" key) passes even though Lookup(1) returns ErrNotFound.
        // 2) Recorded sentinel case passes when Lookup(1) returns ErrNotFound.
        // 3) Recorded sentinel case fails when Lookup(2) returns a plain error with the same text.
        let cases_path = dir.join("cases.jsonl");
        let out_path = dir.join("out.json");
        std::fs::write(
            &cases_path,
            concat!(
                "{\"fn\":\"Lookup\",\"args\":[1],\"error\":\"not found\"}\n",
                "{\"fn\":\"Lookup\",\"args\":[1],\"error\":\"not found\",\"sentinel\":\"ErrNotFound\"}\n",
                "{\"fn\":\"Lookup\",\"args\":[2],\"error\":\"not found\",\"sentinel\":\"ErrNotFound\"}\n",
            ),
        )
        .unwrap();

        let status = std::process::Command::new("go")
            .current_dir(&dir)
            .args([
                "run",
                "./drv",
                "diff",
                cases_path.to_str().unwrap(),
                out_path.to_str().unwrap(),
            ])
            .status()
            .unwrap();
        assert!(status.success(), "go driver diff failed to run");

        let summary: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&out_path).unwrap()).unwrap();
        assert_eq!(summary["total"], 3, "{summary}");
        assert_eq!(summary["passed"], 2, "{summary}");
        assert_eq!(summary["failed"], 1, "{summary}");
        let mismatches = summary["mismatches"].as_array().unwrap();
        assert_eq!(mismatches.len(), 1, "{summary}");
        assert_eq!(mismatches[0]["args"], serde_json::json!([2]), "{summary}");
        assert_eq!(
            mismatches[0]["expected"]["sentinel"], "ErrNotFound",
            "{summary}"
        );
        assert!(
            mismatches[0]["actual"].get("sentinel").is_none(),
            "{summary}"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn typed_rs_avoids_redundant_closure_for_zero_arg_fallible_functions() {
        let src = r#"package ping

func Pong() (string, error) { return "pong", nil }
func PurePong() string { return "pong" }
func Echo(s string) (string, error) { return s, nil }
"#;
        let exports = crate::lang::go::exports("ping.go", src);
        let manifest = Manifest {
            schema: String::new(),
            root: std::path::PathBuf::new(),
            root_written: None,
            unit: "ping".into(),
            language: crate::lang::Lang::Go,
            kind: crate::bridge::Kind::Ffi,
            crate_name: "ping-rs".into(),
            exports,
            classes: vec![],
            js_only: vec![],
            batches: vec![],
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
            end_state: None,
        };
        let mut api = crate::codegen::parse_port(
            "pub fn pong() -> Result<String, String> { Ok(\"pong\".into()) }\n\
             pub fn pure_pong() -> String { \"pong\".into() }\n\
             pub fn echo(s: String) -> Result<String, String> { Ok(s) }\n",
        );
        crate::bridge::match_export_names(&manifest, &mut api);
        let rs = typed_rs(&manifest, &api);
        assert!(rs.contains("match run(port::pong)"), "{rs}");
        assert!(!rs.contains("run(|| port::pong())"), "{rs}");
        assert!(rs.contains("match run(|| Ok(port::pure_pong()))"), "{rs}");
        assert!(rs.contains("match run(|| port::echo(ra_arg0))"), "{rs}");
    }
}
