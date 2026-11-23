use std::path::Path;

use rust_anything::bridge;
use rust_anything::lang::{Lang, OVERLOADED};

/// Bridges `file` (holding `source`) in a fresh folder and returns the bridged export names and
/// the names kept in the original with their reasons.
fn bridge_one(
    lang: Lang,
    file: &str,
    source: &str,
    keep: &[&str],
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let root = std::env::temp_dir().join(format!(
        "rust-anything-overloads-{}-{}-{}",
        file.replace('.', "-"),
        std::process::id(),
        NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root)?;
    std::fs::write(root.join(file), source)?;
    let out = root.join("port");
    let result = bridge::bridge(&bridge::Options {
        root: &root,
        unit: file,
        kind: None,
        out: Some(out.clone()),
        name: Some("sample-rs".into()),
        force: false,
        keep: keep.iter().map(|k| k.to_string()).collect(),
        lang: Some(lang),
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })
    .and_then(|outcome| {
        let manifest = bridge::Manifest::load(Path::new(&out))?;
        Ok((
            outcome.exports,
            manifest
                .js_only
                .into_iter()
                .map(|j| format!("{}: {}", j.name, j.reason))
                .collect(),
        ))
    });
    std::fs::remove_dir_all(&root)?;
    result
}

#[test]
fn a_java_overload_stays_in_java_and_the_rest_is_bridged() {
    let (exports, kept) = bridge_one(
        Lang::Java,
        "Pad.java",
        "package com.example;\npublic final class Pad {\n\
         public static String pad(String s, int n) { return String.format(\"%-\" + n + \"s\", s); }\n\
         public static String pad(String s) { return pad(s, 8); }\n\
         public static int twice(int n) { return n * 2; }\n}\n",
        &[],
    )
    .unwrap();
    assert_eq!(exports, ["twice"]);
    assert_eq!(kept, [format!("pad: {OVERLOADED}")]);
}

#[test]
fn a_csharp_overload_stays_in_csharp_and_the_rest_is_bridged() {
    let (exports, kept) = bridge_one(
        Lang::Csharp,
        "Pad.cs",
        "namespace Acme;\npublic static class Pad\n{\n\
         public static string Fill(string s, int n) { return s.PadRight(n); }\n\
         public static string Fill(string s) { return Fill(s, 8); }\n\
         public static int Twice(int n) { return n * 2; }\n}\n",
        &[],
    )
    .unwrap();
    assert_eq!(exports, ["Twice"]);
    assert_eq!(kept, [format!("Fill: {OVERLOADED}")]);
}

#[test]
fn a_kotlin_overload_stays_in_kotlin_and_the_rest_is_bridged() {
    let (exports, kept) = bridge_one(
        Lang::Kotlin,
        "Pad.kt",
        "package demo\n\nfun pad(s: String, n: Int): String = s.padEnd(n)\n\n\
         fun pad(s: String): String = pad(s, 8)\n\nfun twice(n: Int): Int = n * 2\n",
        &[],
    )
    .unwrap();
    assert_eq!(exports, ["twice"]);
    assert_eq!(kept, [format!("pad: {OVERLOADED}")]);
}

#[test]
fn two_kept_functions_with_one_name_that_are_not_overloads_still_clash() {
    let err = bridge_one(
        Lang::Ruby,
        "parsers.rb",
        "module A\n  def self.parse(x, &b)\n    b.call(x)\n  end\nend\n\n\
         module B\n  def self.parse(x, &b)\n    b.call(x)\n  end\nend\n\n\
         def self.twice(n)\n  n * 2\nend\n",
        &[],
    )
    .unwrap_err()
    .to_string();
    assert!(err.contains("`parse` is defined twice"), "{err}");
}

#[test]
fn an_overload_kept_with_keep_is_still_one_name() {
    let (exports, kept) = bridge_one(
        Lang::Java,
        "Pad.java",
        "package com.example;\npublic final class Pad {\n\
         public static String pad(String s, int n) { return s; }\n\
         public static String pad(String s) { return s; }\n\
         public static int twice(int n) { return n * 2; }\n}\n",
        &["pad"],
    )
    .unwrap();
    assert_eq!(exports, ["twice"]);
    assert_eq!(kept, ["pad: kept on request"]);
}

#[test]
fn an_overload_set_never_absorbs_another_kept_function_whatever_the_order() {
    let xml = "public static class Xml { public static int Parse(object o) { return 1; } }\n";
    let json = "public static class Json {\n\
                public static int Parse(string s) { return 1; }\n\
                public static int Parse(string s, int n) { return n; }\n}\n";
    for source in [
        format!("namespace Acme;\n{xml}{json}"),
        format!("namespace Acme;\n{json}{xml}"),
    ] {
        let err = bridge_one(Lang::Csharp, "Parsers.cs", &source, &[])
            .unwrap_err()
            .to_string();
        assert!(err.contains("`Parse` is defined twice"), "{err}");
    }
}
