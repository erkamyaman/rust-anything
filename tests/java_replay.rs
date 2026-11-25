mod common;

use std::{path::Path, process::Command};

use rust_anything::{
    bridge,
    codegen_java::{DRIVER_JAVA, ERROR_JAVA, JSON_JAVA, WIRE_JAVA},
    harness::DiffOutcome,
};

#[test]
fn java_replay_distinguishes_null_from_a_rejected_argument() {
    if !common::have("java") || !common::have("javac") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-java-null-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let result = replay(&root);
    std::fs::remove_dir_all(root).unwrap();

    let diff = result.unwrap();
    assert_eq!((diff.passed, diff.failed), (1, 1));
    assert_eq!(diff.mismatches[0]["actual"]["invalid"], true);
}

#[test]
fn java_replay_and_wrapper_handle_extends_wildcard_parameters() {
    if !common::have("java") || !common::have("javac") {
        return;
    }

    let root = std::env::temp_dir().join(format!(
        "rust-anything-java-wildcard-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let result = replay_wildcards(&root);
    let _ = std::fs::remove_dir_all(&root);
    result.unwrap();
}

fn replay_wildcards(root: &Path) -> anyhow::Result<()> {
    let src = r#"package acme;
import java.util.List;
import java.util.Map;
import java.util.Optional;
public class Wildcards {
    public static int count(List<? extends CharSequence> xs) {
        int total = 0;
        for (CharSequence s : xs) total += s.length();
        return total;
    }
    public static int sumValues(Map<String, ? extends Integer> m) {
        int total = 0;
        for (Integer v : m.values()) total += v;
        return total;
    }
    public static int firstLen(Optional<? extends CharSequence> opt) {
        return opt.map(CharSequence::length).orElse(-1);
    }
    public static int nested(List<? extends List<? extends CharSequence>> rows) {
        int total = 0;
        for (List<? extends CharSequence> row : rows) total += row.size();
        return total;
    }
    public static int nestedGenericArray(List<? extends List<String>[][]> grids) {
        List<String>[][] first = grids.get(0);
        return grids.size() + first.length + first[0].length;
    }
    public static Map<CharSequence, Integer> echoCharSeqMap(Map<CharSequence, Integer> m) {
        return m;
    }
}
"#;
    std::fs::create_dir_all(root.join("acme"))?;
    std::fs::write(root.join("acme/Wildcards.java"), src)?;

    // Bridge the class and compile the generated Java wrapper + wire classes with javac.
    let out = root.join("port");
    bridge::bridge(&bridge::Options {
        root,
        unit: "acme/Wildcards.java",
        kind: None,
        out: Some(out.clone()),
        name: Some("wildcards-rs".into()),
        force: false,
        keep: Vec::new(),
        lang: Some(rust_anything::lang::Lang::Java),
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })?;

    let java_dir = out.join("java");
    let compile_dir = root.join("wrapper-classes");
    std::fs::create_dir_all(&compile_dir)?;
    let mut javac = Command::new("javac");
    javac.args(["-d", compile_dir.to_str().unwrap()]);
    for rel in [
        "rustanything/RustAnythingJson.java",
        "rustanything/RustAnythingWire.java",
        "rustanything/RustAnythingError.java",
        "rustanything/WildcardsRsNative.java",
        "acme/Wildcards.java",
    ] {
        javac.arg(java_dir.join(rel));
    }
    run(&mut javac)?;

    // Compile the original class with the driver and run diff across valid and invalid inputs.
    let run_dir = root.join("driver-run");
    std::fs::create_dir_all(run_dir.join("acme"))?;
    std::fs::write(run_dir.join("RustAnythingDriver.java"), DRIVER_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingJson.java"), JSON_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingWire.java"), WIRE_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingError.java"), ERROR_JAVA)?;
    std::fs::write(run_dir.join("acme/Wildcards.java"), src)?;
    std::fs::write(
        run_dir.join("targets.json"),
        r#"{"count":["acme.Wildcards","count"],"sumValues":["acme.Wildcards","sumValues"],"firstLen":["acme.Wildcards","firstLen"],"nested":["acme.Wildcards","nested"],"nestedGenericArray":["acme.Wildcards","nestedGenericArray"],"echoCharSeqMap":["acme.Wildcards","echoCharSeqMap"]}"#,
    )?;
    std::fs::write(
        run_dir.join("cases.jsonl"),
        concat!(
            r#"{"fn":"count","args":[["hi","there"]],"ok":7}"#,
            "\n",
            r#"{"fn":"sumValues","args":[{"a":2,"b":5}],"ok":7}"#,
            "\n",
            r#"{"fn":"firstLen","args":["rust"],"ok":4}"#,
            "\n",
            r#"{"fn":"firstLen","args":[null],"ok":-1}"#,
            "\n",
            r#"{"fn":"nested","args":[[["a","b"],["c"]]],"ok":3}"#,
            "\n",
            r#"{"fn":"nestedGenericArray","args":[[[[["a"]]]]],"ok":3}"#,
            "\n",
            r#"{"fn":"echoCharSeqMap","args":[{"k":9}],"ok":{"k":9}}"#,
            "\n",
            r#"{"fn":"count","args":[[123]],"ok":0}"#,
            "\n",
        ),
    )?;

    run(Command::new("javac").current_dir(&run_dir).args([
        "-d",
        ".",
        "RustAnythingDriver.java",
        "RustAnythingJson.java",
        "RustAnythingWire.java",
        "RustAnythingError.java",
        "acme/Wildcards.java",
    ]))?;
    run(Command::new("java").current_dir(&run_dir).args([
        "-cp",
        ".",
        "rustanything.RustAnythingDriver",
        "diff",
        "targets.json",
        "cases.jsonl",
        "diff.json",
    ]))?;
    let diff: DiffOutcome = serde_json::from_slice(&std::fs::read(run_dir.join("diff.json"))?)?;
    assert_eq!((diff.passed, diff.failed), (7, 1), "{diff:?}");
    assert_eq!(diff.mismatches[0]["actual"]["invalid"], true, "{diff:?}");
    Ok(())
}

#[test]
fn java_replay_and_wrapper_handle_ordered_sets() {
    if !common::have("java") || !common::have("javac") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-java-sets-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let result = replay_ordered_sets(&root);
    let _ = std::fs::remove_dir_all(&root);
    result.unwrap();
}

fn replay_ordered_sets(root: &Path) -> anyhow::Result<()> {
    let src = r#"package acme;
import java.util.LinkedHashSet;
import java.util.NavigableSet;
import java.util.SortedSet;
import java.util.TreeSet;
public class OrderedSets {
    public static String joinSorted(SortedSet<String> words) {
        return String.join(",", words);
    }
    public static TreeSet<String> toTree(LinkedHashSet<String> words) {
        return new TreeSet<>(words);
    }
    public static LinkedHashSet<String> keepOrder(LinkedHashSet<String> words) {
        return words;
    }
    public static int maxNav(NavigableSet<Integer> nums) {
        return nums.isEmpty() ? -1 : nums.last();
    }
    @SuppressWarnings("unchecked")
    public static TreeSet<String>[] splitSets(int n) {
        return new TreeSet[n];
    }
    public static String[] toArray(LinkedHashSet<String> words) {
        return words.toArray(new String[0]);
    }
}
"#;
    std::fs::create_dir_all(root.join("acme"))?;
    std::fs::write(root.join("acme/OrderedSets.java"), src)?;

    let out = root.join("port");
    bridge::bridge(&bridge::Options {
        root,
        unit: "acme/OrderedSets.java",
        kind: None,
        out: Some(out.clone()),
        name: Some("ordered-sets-rs".into()),
        force: false,
        keep: Vec::new(),
        lang: Some(rust_anything::lang::Lang::Java),
        batch: Vec::new(),
        as_json: Vec::new(),
        allocator: Default::default(),
    })?;

    let java_dir = out.join("java");
    let compile_dir = root.join("wrapper-classes");
    std::fs::create_dir_all(&compile_dir)?;
    let mut javac = Command::new("javac");
    javac.args(["-d", compile_dir.to_str().unwrap()]);
    for rel in [
        "rustanything/RustAnythingJson.java",
        "rustanything/RustAnythingWire.java",
        "rustanything/RustAnythingError.java",
        "rustanything/OrderedSetsRsNative.java",
        "acme/OrderedSets.java",
    ] {
        javac.arg(java_dir.join(rel));
    }
    run(&mut javac)?;

    let run_dir = root.join("driver-run");
    std::fs::create_dir_all(run_dir.join("acme"))?;
    std::fs::write(run_dir.join("RustAnythingDriver.java"), DRIVER_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingJson.java"), JSON_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingWire.java"), WIRE_JAVA)?;
    std::fs::write(run_dir.join("RustAnythingError.java"), ERROR_JAVA)?;
    std::fs::write(run_dir.join("acme/OrderedSets.java"), src)?;
    std::fs::write(
        run_dir.join("targets.json"),
        r#"{"joinSorted":["acme.OrderedSets","joinSorted"],"toTree":["acme.OrderedSets","toTree"],"keepOrder":["acme.OrderedSets","keepOrder"],"maxNav":["acme.OrderedSets","maxNav"]}"#,
    )?;
    std::fs::write(
        run_dir.join("cases.jsonl"),
        concat!(
            r#"{"fn":"joinSorted","args":[["a","b"]],"ok":"a,b"}"#,
            "\n",
            r#"{"fn":"toTree","args":[["c","a","b"]],"ok":["a","b","c"]}"#,
            "\n",
            r#"{"fn":"keepOrder","args":[["c","a","b"]],"ok":["c","a","b"]}"#,
            "\n",
            r#"{"fn":"keepOrder","args":[["c","a","a"]],"ok":["c","a"]}"#,
            "\n",
            r#"{"fn":"joinSorted","args":[["a","a"]],"ok":"a"}"#,
            "\n",
            r#"{"fn":"maxNav","args":[[1,2,3,9]],"ok":9}"#,
            "\n",
            r#"{"fn":"joinSorted","args":[[null]],"ok":""}"#,
            "\n",
            r#"{"fn":"joinSorted","args":[["b","a"]],"ok":"b,a"}"#,
            "\n",
        ),
    )?;

    std::fs::write(
        run_dir.join("CheckTreeSetWire.java"),
        r#"import rustanything.RustAnythingWire;
import java.util.TreeSet;
public class CheckTreeSetWire {
    public static void main(String[] args) {
        RustAnythingWire.Writer ok = new RustAnythingWire.Writer();
        ok.tag(1); ok.len(2); ok.str("a"); ok.str("b");
        RustAnythingWire.Reader okReader = new RustAnythingWire.Reader(ok.bytes());
        TreeSet<String> parsed = okReader.treeSet(okReader::str);
        if (!parsed.toString().equals("[a, b]")) {
            throw new AssertionError("unexpected sorted set: " + parsed);
        }
        RustAnythingWire.Writer dup = new RustAnythingWire.Writer();
        dup.tag(1); dup.len(2); dup.str("a"); dup.str("a");
        try {
            RustAnythingWire.Reader dupReader = new RustAnythingWire.Reader(dup.bytes());
            dupReader.linkedHashSet(dupReader::str);
            throw new AssertionError("expected a repeated element to be rejected");
        } catch (IllegalArgumentException expected) {
            if (!expected.getMessage().contains("duplicate")) {
                throw new AssertionError("unexpected error: " + expected);
            }
        }
        RustAnythingWire.Writer rev = new RustAnythingWire.Writer();
        rev.tag(1); rev.len(2); rev.str("b"); rev.str("a");
        try {
            RustAnythingWire.Reader revReader = new RustAnythingWire.Reader(rev.bytes());
            revReader.treeSet(revReader::str);
            throw new AssertionError("expected non-natural order to be rejected");
        } catch (IllegalArgumentException expected) {
            if (!expected.getMessage().contains("natural order")) {
                throw new AssertionError("unexpected error: " + expected);
            }
        }
    }
}
"#,
    )?;

    run(Command::new("javac").current_dir(&run_dir).args([
        "-d",
        ".",
        "RustAnythingDriver.java",
        "RustAnythingJson.java",
        "RustAnythingWire.java",
        "RustAnythingError.java",
        "CheckTreeSetWire.java",
        "acme/OrderedSets.java",
    ]))?;
    run(Command::new("java")
        .current_dir(&run_dir)
        .args(["-cp", ".", "CheckTreeSetWire"]))?;
    run(Command::new("java").current_dir(&run_dir).args([
        "-cp",
        ".",
        "rustanything.RustAnythingDriver",
        "diff",
        "targets.json",
        "cases.jsonl",
        "diff.json",
    ]))?;
    let diff: DiffOutcome = serde_json::from_slice(&std::fs::read(run_dir.join("diff.json"))?)?;
    assert_eq!((diff.passed, diff.failed), (4, 4), "{diff:?}");
    assert!(
        diff.mismatches
            .iter()
            .all(|m| m["actual"]["invalid"] == true),
        "{diff:?}"
    );
    Ok(())
}

#[test]
fn java_error_lookup_matches_across_json_typed_and_wire_paths() {
    if !common::have("java") || !common::have("javac") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-java-err-{}", std::process::id()));
    std::fs::create_dir_all(&root).unwrap();
    let result = check_error_lookup(&root);
    std::fs::remove_dir_all(root).unwrap();
    result.unwrap();
}

fn check_error_lookup(root: &Path) -> anyhow::Result<()> {
    std::fs::write(root.join("RustAnythingError.java"), ERROR_JAVA)?;
    std::fs::create_dir_all(root.join("com/example"))?;
    std::fs::write(
        root.join("com/example/TestException.java"),
        "package com.example;\n\
         public final class TestException extends RuntimeException {\n\
             public TestException(String message) { super(message); }\n\
         }\n",
    )?;
    std::fs::write(
        root.join("com/example/Outer.java"),
        "package com.example;\n\
         public final class Outer {\n\
             public static final class InnerException extends RuntimeException {\n\
                 public InnerException(String message) { super(message); }\n\
             }\n\
         }\n",
    )?;
    std::fs::write(
        root.join("com/example/NoMsgException.java"),
        "package com.example;\n\
         public final class NoMsgException extends RuntimeException {\n\
             public NoMsgException() { super(\"fixed\"); }\n\
         }\n",
    )?;
    std::fs::write(
        root.join("CheckErrors.java"),
        r#"import rustanything.RustAnythingError;

public class CheckErrors {
    private static void checkBoth(String type, String msg, Class<?> expected) {
        RuntimeException fromJson = RustAnythingError.from(type + ": " + msg);
        RuntimeException fromReal = new RustAnythingError(type, msg).real();
        RuntimeException fromSlot;
        try {
            String[] slot = RustAnythingError.slot();
            slot[0] = type;
            slot[1] = msg;
            RustAnythingError.check(slot);
            throw new AssertionError("expected exception for " + type);
        } catch (RuntimeException e) {
            fromSlot = e;
        }
        if (fromJson.getClass() != expected || fromReal.getClass() != expected || fromSlot.getClass() != expected) {
            throw new AssertionError("class mismatch for " + type + ": "
                + fromJson.getClass() + " / " + fromReal.getClass() + " / " + fromSlot.getClass()
                + " (expected " + expected + ")");
        }
        String expectedMessage = expected == RuntimeException.class ? type + ": " + msg : msg;
        if (!expectedMessage.equals(fromJson.getMessage())
            || !expectedMessage.equals(fromReal.getMessage())
            || !expectedMessage.equals(fromSlot.getMessage())) {
            throw new AssertionError("message mismatch for " + type + ": "
                + fromJson.getMessage() + " / " + fromReal.getMessage() + " / " + fromSlot.getMessage());
        }
    }

    public static void main(String[] args) {
        // Resolving: packaged, nested, and JDK exception packages with a (String) constructor.
        checkBoth("com.example.TestException", "bad", com.example.TestException.class);
        checkBoth("com.example.Outer$InnerException", "nested", com.example.Outer.InnerException.class);
        checkBoth("IllegalArgumentException", "bad", IllegalArgumentException.class);
        checkBoth("NoSuchElementException", "missing", java.util.NoSuchElementException.class);
        checkBoth("DateTimeException", "empty year", java.time.DateTimeException.class);
        checkBoth("java.time.DateTimeException", "empty year", java.time.DateTimeException.class);
        checkBoth("RejectedExecutionException", "busy", java.util.concurrent.RejectedExecutionException.class);

        // Non-resolving: checked, non-exception, missing (String) ctor, unknown, malformed.
        checkBoth("java.io.IOException", "checked", RuntimeException.class);
        checkBoth("java.lang.String", "not an exception", RuntimeException.class);
        checkBoth("com.example.NoMsgException", "no string ctor", RuntimeException.class);
        checkBoth("UncheckedIOException", "disk", RuntimeException.class);
        checkBoth("NoSuchException", "oops", RuntimeException.class);
        checkBoth("bad name", "msg", RuntimeException.class);
        checkBoth("com.example..Bad", "msg", RuntimeException.class);

        // No ": " separator.
        for (String raw : new String[] { "plain error", "IllegalArgumentException:bad", "bad input: 42" }) {
            RuntimeException plain = RustAnythingError.from(raw);
            if (plain.getClass() != RuntimeException.class || !raw.equals(plain.getMessage())) {
                throw new AssertionError("unexpected plain error for " + raw + ": " + plain);
            }
        }
    }
}
"#,
    )?;

    run(Command::new("javac").current_dir(root).args([
        "-d",
        ".",
        "RustAnythingError.java",
        "com/example/TestException.java",
        "com/example/Outer.java",
        "com/example/NoMsgException.java",
        "CheckErrors.java",
    ]))?;
    run(Command::new("java")
        .current_dir(root)
        .args(["-cp", ".", "CheckErrors"]))?;
    Ok(())
}

fn replay(root: &Path) -> anyhow::Result<DiffOutcome> {
    std::fs::write(root.join("RustAnythingDriver.java"), DRIVER_JAVA)?;
    std::fs::write(root.join("RustAnythingJson.java"), JSON_JAVA)?;
    std::fs::write(
        root.join("Port.java"),
        "public class Port { public static String value(int input) { return null; } }",
    )?;
    std::fs::write(root.join("targets.json"), r#"{"value":["Port","value"]}"#)?;
    std::fs::write(
        root.join("cases.jsonl"),
        r#"{"fn":"value","args":[1],"ok":null}
{"fn":"value","args":["previously accepted"],"ok":null}
"#,
    )?;

    run(Command::new("javac").current_dir(root).args([
        "-d",
        ".",
        "RustAnythingDriver.java",
        "RustAnythingJson.java",
        "Port.java",
    ]))?;
    run(Command::new("java").current_dir(root).args([
        "-cp",
        ".",
        "rustanything.RustAnythingDriver",
        "diff",
        "targets.json",
        "cases.jsonl",
        "diff.json",
    ]))?;
    Ok(serde_json::from_slice(&std::fs::read(
        root.join("diff.json"),
    )?)?)
}

fn run(cmd: &mut Command) -> anyhow::Result<()> {
    let output = cmd.output()?;
    anyhow::ensure!(
        output.status.success(),
        "{cmd:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(())
}
