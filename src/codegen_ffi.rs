//! The JSON bridge for Go, Ruby and Java: the port is plain Rust over serde types, and one
//! dispatcher takes a function name and its arguments as JSON and answers
//! `{"ok": value}`, `{"error": message}` or `{"panic": message}`.
//!
//! Go and Ruby reach the dispatcher through two C functions, `rust_anything_call` and
//! `rust_anything_free`; Java through one JNI method. The language side is a generated wrapper with
//! the original names and signatures, so callers do not change.

use crate::MIN_RUST;
use std::fmt::Write as _;

use crate::bridge::Manifest;
use crate::codegen::{PortApi, PortFn};
use crate::lang::Lang;

/// How a wrapper class or package names the JNI entry point: `rust-anything.<Lib>Native`.
pub fn java_native_class(m: &Manifest) -> String {
    let mut out = String::new();
    for part in m.lib_name().split('_') {
        let mut chars = part.chars();
        if let Some(first) = chars.next() {
            out.extend(first.to_uppercase());
            out.push_str(chars.as_str());
        }
    }
    format!("{out}Native")
}

fn arg_expr(f: &PortFn, i: usize) -> String {
    match f.params[i].1.as_str() {
        "f64" => format!("num(args, {i})?"),
        "Option<f64>" => format!("opt_num(args, {i})?"),
        "Vec<f64>" => format!("num_vec(args, {i})?"),
        "Option<Vec<f64>>" => format!("opt_num_vec(args, {i})?"),
        _ => format!("arg(args, {i})?"),
    }
}

fn out_expr(f: &PortFn) -> &'static str {
    match f.ret.as_str() {
        "f64" => "out_num(out)",
        "Option<f64>" => "out.map_or(Value::Null, out_num)",
        "Vec<f64>" => "Value::Array(out.into_iter().map(out_num).collect())",
        "()" => "Value::Null",
        _ => "serde_json::to_value(out).map_err(|e| e.to_string())?",
    }
}

pub fn lib_rs(m: &Manifest, api: &PortApi) -> String {
    let mut arms = String::new();
    for f in &api.functions {
        let args: Vec<String> = (0..f.params.len()).map(|i| arg_expr(f, i)).collect();
        let call = format!("port::{}({})", f.rust_name, args.join(", "));
        let call = if f.fallible { format!("{call}?") } else { call };
        let binding = if f.ret == "()" {
            "let () ="
        } else {
            "let out ="
        };
        let _ = writeln!(
            arms,
            "        \"{name}\" => {{\n            {binding} {call};\n            Ok({out})\n        }}",
            name = f.js_name,
            out = out_expr(f),
        );
    }
    let entry = if matches!(m.language, Lang::Java | Lang::Kotlin) {
        format!(
            r#"
/// `static native String call(String name, String args)` of `rustanything.{class}`.
// SAFETY: the JNI symbol name of that method; nothing else defines it.
#[unsafe(no_mangle)]
pub extern "system" fn Java_rustanything_{mangled}_call<'local>(
    mut env: jni::JNIEnv<'local>,
    _class: jni::objects::JClass<'local>,
    name: jni::objects::JString<'local>,
    args: jni::objects::JString<'local>,
) -> jni::sys::jstring {{
    let read = |env: &mut jni::JNIEnv<'local>, s: &jni::objects::JString<'local>| -> String {{
        env.get_string(s).map(Into::into).unwrap_or_default()
    }};
    let name = read(&mut env, &name);
    let args = read(&mut env, &args);
    env.new_string(call_json(&name, &args))
        .map_or(std::ptr::null_mut(), jni::objects::JString::into_raw)
}}
"#,
            class = java_native_class(m),
            mangled = crate::codegen_java::mangle_jni_component(&java_native_class(m)),
        )
    } else {
        r#"
/// The C entry point: `char *rust_anything_call(const char *name, const char *args)`. Free the
/// answer with `rust_anything_free`.
///
/// # Safety
///
/// `name` and `args` must be NUL-terminated strings, valid for the call.
// SAFETY: exported unmangled under a name only rust-anything defines.
#[unsafe(no_mangle)]
// SAFETY: callers uphold the contract under `# Safety`.
pub unsafe extern "C" fn rust_anything_call(name: *const c_char, args: *const c_char) -> *mut c_char {
    let text = |ptr: *const c_char| -> String {
        if ptr.is_null() {
            return String::new();
        }
        // SAFETY: the caller passes NUL-terminated strings that live for the call (see above).
        unsafe { CStr::from_ptr(ptr) }.to_string_lossy().into_owned()
    };
    let answer = call_json(&text(name), &text(args));
    // A JSON text never contains NUL: serde_json escapes it as \u0000.
    CString::new(answer).map_or(std::ptr::null_mut(), CString::into_raw)
}

/// Frees an answer of `rust_anything_call`.
///
/// # Safety
///
/// `ptr` must come from `rust_anything_call` and not be freed before; NULL is ignored.
// SAFETY: exported unmangled under a name only rust-anything defines.
#[unsafe(no_mangle)]
// SAFETY: callers uphold the contract under `# Safety`.
pub unsafe extern "C" fn rust_anything_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: `ptr` was made by `CString::into_raw` in `rust_anything_call` (see above).
        drop(unsafe { CString::from_raw(ptr) });
    }
}
"#
        .to_string()
    };
    let imports = if matches!(m.language, Lang::Java | Lang::Kotlin) {
        "use serde_json::{Value, json};\n"
    } else {
        "use std::ffi::{CStr, CString, c_char};\n\nuse serde_json::{Value, json};\n"
    };
    format!(
        r#"//! The {lang} bridge for `{unit}`, generated by rust-anything from `src/port.rs`: a JSON
//! dispatcher over the port, and its entry point. Regenerate with `rust-anything refresh`.

{imports}
pub mod port;

/// Argument `i`, decoded from JSON into the port's parameter type.
fn arg<T: serde::de::DeserializeOwned>(args: &[Value], i: usize) -> Result<T, String> {{
    serde_json::from_value(args.get(i).cloned().unwrap_or(Value::Null))
        .map_err(|e| format!("argument {{i}}: {{e}}"))
}}

/// A float argument: JSON has no NaN or infinities, so they arrive as `{{"$num": "NaN"}}`.
#[allow(dead_code)]
fn num(args: &[Value], i: usize) -> Result<f64, String> {{
    match args.get(i) {{
        Some(Value::Object(o)) => match o.get("$num").and_then(Value::as_str) {{
            Some("NaN") => Ok(f64::NAN),
            Some("Infinity") => Ok(f64::INFINITY),
            Some("-Infinity") => Ok(f64::NEG_INFINITY),
            Some("-0") => Ok(-0.0),
            _ => Err(format!("argument {{i}}: not a number")),
        }},
        _ => arg(args, i),
    }}
}}

#[allow(dead_code)]
fn num_vec(args: &[Value], i: usize) -> Result<Vec<f64>, String> {{
    match args.get(i) {{
        Some(Value::Array(items)) => (0..items.len()).map(|j| num(items, j)).collect(),
        _ => arg(args, i),
    }}
}}

#[allow(dead_code)]
fn opt_num_vec(args: &[Value], i: usize) -> Result<Option<Vec<f64>>, String> {{
    match args.get(i) {{
        None | Some(Value::Null) => Ok(None),
        Some(_) => num_vec(args, i).map(Some),
    }}
}}

#[allow(dead_code)]
fn opt_num(args: &[Value], i: usize) -> Result<Option<f64>, String> {{
    match args.get(i) {{
        None | Some(Value::Null) => Ok(None),
        Some(_) => num(args, i).map(Some),
    }}
}}

#[allow(dead_code)]
fn out_num(x: f64) -> Value {{
    if x.is_nan() {{
        json!({{"$num": "NaN"}})
    }} else if x.is_infinite() {{
        json!({{"$num": if x > 0.0 {{ "Infinity" }} else {{ "-Infinity" }}}})
    }} else if x == 0.0 && x.is_sign_negative() {{
        json!({{"$num": "-0"}})
    }} else {{
        json!(x)
    }}
}}

fn dispatch(name: &str, args: &[Value]) -> Result<Value, String> {{
    match name {{
{arms}        _ => Err(format!("no function `{{name}}`")),
    }}
}}

/// Runs `name` on the JSON array `args`; never unwinds into the caller.
pub fn call_json(name: &str, args: &str) -> String {{
    let result = std::panic::catch_unwind(|| {{
        let args: Vec<Value> = serde_json::from_str(args).map_err(|e| e.to_string())?;
        dispatch(name, &args)
    }});
    match result {{
        Ok(Ok(value)) => json!({{"ok": value}}),
        Ok(Err(message)) => json!({{"error": message}}),
        Err(panic) => {{
            let message = panic
                .downcast_ref::<&str>()
                .map(ToString::to_string)
                .or_else(|| panic.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic".into());
            json!({{"panic": message}})
        }}
    }}
    .to_string()
}}
{entry}"#,
        lang = m.language.name(),
        unit = m.unit,
    )
}

pub fn cargo_toml(m: &Manifest) -> String {
    let (crate_types, extra) = match m.language {
        Lang::Java | Lang::Kotlin => ("\"cdylib\", \"rlib\"", "jni = \"0.21\"\n"),
        Lang::Go => ("\"cdylib\", \"rlib\"", ""),
        _ => ("\"cdylib\", \"staticlib\", \"rlib\"", ""),
    };
    // Ruby hashes and Java's LinkedHashMap keep insertion order; Go's encoder sorts map keys.
    // `float_roundtrip`: the default float parser can be off by one ULP.
    let serde_json = if m.language == Lang::Go {
        "serde_json = { version = \"1\", features = [\"float_roundtrip\"] }\n"
    } else {
        "serde_json = { version = \"1\", features = [\"preserve_order\", \"float_roundtrip\"] }\n"
    };
    format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2024\"\nrust-version = \"{MIN_RUST}\"\npublish = false\n\
         description = \"Rust port of `{}`, generated by rust-anything\"\n\n\
         [lib]\ncrate-type = [{crate_types}]\n\n[dependencies]\nserde = {{ version = \"1\", features = [\"derive\"] }}\n\
         {serde_json}{extra}\n[profile.release]\nlto = true\n\n[lints.clippy]\nall = {{ level = \"warn\", priority = -1 }}\n\n[lints.rust]\nunused_lifetimes = \"warn\"\ntrivial_numeric_casts = \"warn\"\n",
        m.crate_name, m.unit
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest(crate_name: &str) -> Manifest {
        Manifest {
            schema: String::new(),
            root: std::path::PathBuf::new(),
            root_written: None,
            unit: "unit".into(),
            language: Lang::Java,
            kind: crate::bridge::Kind::Ffi,
            crate_name: crate_name.into(),
            exports: Vec::new(),
            classes: Vec::new(),
            js_only: Vec::new(),
            batches: Vec::new(),
            allocator: Default::default(),
            float_tolerance: None,
            ignore_key_order: false,
        }
    }

    fn port_fn(params: Vec<(&str, &str)>, ret: &str) -> PortFn {
        PortFn {
            js_name: "test".into(),
            rust_name: "test".into(),
            params: params
                .into_iter()
                .map(|(n, t)| (n.into(), t.into()))
                .collect(),
            ret: ret.into(),
            fallible: false,
            is_async: false,
        }
    }

    #[test]
    fn test_java_native_class() {
        // Multi-part name (hyphenated and underscored)
        let m1 = manifest("text-kit-rs");
        assert_eq!(java_native_class(&m1), "TextKitRsNative");

        let m2 = manifest("my_awesome_tool");
        assert_eq!(java_native_class(&m2), "MyAwesomeToolNative");

        // Doubled underscore (or doubled hyphen)
        let m3 = manifest("foo__bar");
        assert_eq!(java_native_class(&m3), "FooBarNative");

        let m4 = manifest("double--hyphen__lib");
        assert_eq!(java_native_class(&m4), "DoubleHyphenLibNative");

        // Single part
        let m5 = manifest("simple");
        assert_eq!(java_native_class(&m5), "SimpleNative");
    }

    #[test]
    fn test_arg_expr() {
        let f = port_fn(
            vec![
                ("a", "f64"),
                ("b", "Option<f64>"),
                ("c", "Vec<f64>"),
                ("d", "Option<Vec<f64>>"),
                ("e", "String"),
                ("f", "i32"),
            ],
            "()",
        );

        assert_eq!(arg_expr(&f, 0), "num(args, 0)?");
        assert_eq!(arg_expr(&f, 1), "opt_num(args, 1)?");
        assert_eq!(arg_expr(&f, 2), "num_vec(args, 2)?");
        assert_eq!(arg_expr(&f, 3), "opt_num_vec(args, 3)?");
        assert_eq!(arg_expr(&f, 4), "arg(args, 4)?");
        assert_eq!(arg_expr(&f, 5), "arg(args, 5)?");
    }

    #[test]
    fn test_out_expr() {
        let f_f64 = port_fn(vec![], "f64");
        assert_eq!(out_expr(&f_f64), "out_num(out)");

        let f_opt_f64 = port_fn(vec![], "Option<f64>");
        assert_eq!(out_expr(&f_opt_f64), "out.map_or(Value::Null, out_num)");

        let f_vec_f64 = port_fn(vec![], "Vec<f64>");
        assert_eq!(
            out_expr(&f_vec_f64),
            "Value::Array(out.into_iter().map(out_num).collect())"
        );

        let f_unit = port_fn(vec![], "()");
        assert_eq!(out_expr(&f_unit), "Value::Null");

        let f_string = port_fn(vec![], "String");
        assert_eq!(
            out_expr(&f_string),
            "serde_json::to_value(out).map_err(|e| e.to_string())?"
        );

        let f_custom = port_fn(vec![], "MyStruct");
        assert_eq!(
            out_expr(&f_custom),
            "serde_json::to_value(out).map_err(|e| e.to_string())?"
        );
    }
}
