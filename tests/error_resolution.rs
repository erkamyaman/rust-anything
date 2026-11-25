mod common;

use std::{path::PathBuf, process::Command};

use rust_anything::{
    bridge::{Kind, Manifest},
    codegen_csharp, codegen_php,
    lang::Lang,
};

#[test]
fn csharp_error_resolution_matches_qualified_and_nested_exceptions_and_falls_back() {
    if !common::have("dotnet") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-cs-err-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();

    let version_out = Command::new("dotnet")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .arg("--version")
        .output()
        .unwrap();
    assert!(version_out.status.success());
    let version = String::from_utf8_lossy(&version_out.stdout);
    let major: u32 = version.trim().split('.').next().unwrap().parse().unwrap();
    let framework = format!("net{major}.0");

    std::fs::write(root.join("Driver.cs"), codegen_csharp::driver()).unwrap();
    std::fs::write(
        root.join("CheckErrors.csproj"),
        format!(
            "<Project Sdk=\"Microsoft.NET.Sdk\">\n\
             \x20 <PropertyGroup>\n\
             \x20   <OutputType>Exe</OutputType>\n\
             \x20   <TargetFramework>{framework}</TargetFramework>\n\
             \x20   <StartupObject>RustAnything.Driver.CheckErrors</StartupObject>\n\
             \x20   <EnableDefaultCompileItems>false</EnableDefaultCompileItems>\n\
             \x20   <Nullable>enable</Nullable>\n\
             \x20   <LangVersion>latest</LangVersion>\n\
             \x20 </PropertyGroup>\n\
             \x20 <ItemGroup>\n\
             \x20   <Compile Include=\"Driver.cs\" />\n\
             \x20   <Compile Include=\"CheckErrors.cs\" />\n\
             \x20 </ItemGroup>\n\
             </Project>\n"
        ),
    )
    .unwrap();
    std::fs::write(
        root.join("CheckErrors.cs"),
        r#"#nullable enable

namespace Acme.Errors
{
    public sealed class ParseException : global::System.Exception
    {
        public ParseException(string message) : base(message) { }
    }
}

namespace Acme.Geo
{
    public static class Geometry
    {
        public sealed class ParseException : global::System.Exception
        {
            public ParseException(string message) : base(message) { }
        }
    }

    public sealed class NotAnException
    {
        public NotAnException(string message) { }
    }

    public sealed class MultiArgException : global::System.Exception
    {
        public MultiArgException(string a, string b) : base(a) { }
    }
}

namespace RustAnything.Driver
{
    public static class CheckErrors
    {
        public static void Main()
        {
            var cases = new (string Input, string ExpectedType, string ExpectedMessage)[]
            {
                ("System.ArgumentException: bad", "System.ArgumentException", "bad"),
                ("Acme.Errors.ParseException: bad", "Acme.Errors.ParseException", "bad"),
                ("Acme.Geo.Geometry+ParseException: not a digit", "Acme.Geo.Geometry+ParseException", "not a digit"),
                ("ArgumentException: bad", "System.InvalidOperationException", "ArgumentException: bad"),
                ("Acme.Geo.NotAnException: bad", "System.InvalidOperationException", "Acme.Geo.NotAnException: bad"),
                ("Acme.Geo.MultiArgException: bad", "System.InvalidOperationException", "Acme.Geo.MultiArgException: bad"),
                ("Acme.Geo.MissingException: bad", "System.InvalidOperationException", "Acme.Geo.MissingException: bad"),
                ("Acme.Geo.Geometry+: bad", "System.InvalidOperationException", "Acme.Geo.Geometry+: bad"),
                ("Acme.Geo..Bad: bad", "System.InvalidOperationException", "Acme.Geo..Bad: bad"),
                ("plain error", "System.InvalidOperationException", "plain error"),
            };
            foreach (var (input, expectedType, expectedMessage) in cases)
            {
                var err = DriverCodec.Error(input);
                var actualType = err.GetType().FullName;
                if (actualType != expectedType || err.Message != expectedMessage)
                {
                    throw new global::System.Exception(
                        $"mismatch for {input}: got {actualType} ({err.Message}), expected {expectedType} ({expectedMessage})");
                }
            }
        }
    }
}
"#,
    )
    .unwrap();

    let out = Command::new("dotnet")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .current_dir(&root)
        .args(["run", "--project", "CheckErrors.csproj", "--nologo"])
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        out.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

#[test]
fn php_error_resolution_matches_namespaced_throwables_and_falls_back() {
    if !common::have("php") {
        return;
    }

    let root = std::env::temp_dir().join(format!("rust-anything-php-err-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();

    let manifest = Manifest {
        schema: String::new(),
        root: PathBuf::new(),
        root_written: None,
        unit: "src/numbers.php".into(),
        language: Lang::Php,
        kind: Kind::Ffi,
        crate_name: "err-rs".into(),
        exports: Vec::new(),
        classes: Vec::new(),
        js_only: Vec::new(),
        batches: Vec::new(),
        allocator: Default::default(),
        float_tolerance: None,
        ignore_key_order: false,
    };
    std::fs::write(root.join("wrapper.php"), codegen_php::wrapper(&manifest)).unwrap();
    std::fs::write(
        root.join("check_errors.php"),
        r#"<?php

namespace Acme\Numbers {
    final class ParseException extends \RuntimeException {}
    final class NotThrowable {}
    final class TwoArgException extends \RuntimeException
    {
        public function __construct(string $a, string $b)
        {
            parent::__construct($a);
        }
    }
}

namespace {
    require_once __DIR__ . '/wrapper.php';
    use RustAnything\Codec;

    $cases = [
        ["InvalidArgumentException: bad", "InvalidArgumentException", "bad"],
        ["\\InvalidArgumentException: bad", "InvalidArgumentException", "bad"],
        ["Acme\\Numbers\\ParseException: bad", "Acme\\Numbers\\ParseException", "bad"],
        ["\\Acme\\Numbers\\ParseException: line1\nline2", "Acme\\Numbers\\ParseException", "line1\nline2"],
        ["ParseException: bad", "RuntimeException", "ParseException: bad"],
        ["Acme\\Numbers\\MissingException: bad", "RuntimeException", "Acme\\Numbers\\MissingException: bad"],
        ["Acme\\Numbers\\NotThrowable: bad", "RuntimeException", "Acme\\Numbers\\NotThrowable: bad"],
        ["Acme\\Numbers\\TwoArgException: bad", "RuntimeException", "Acme\\Numbers\\TwoArgException: bad"],
        ["Acme\\\\Numbers\\ParseException: bad", "RuntimeException", "Acme\\\\Numbers\\ParseException: bad"],
        ["bad name: msg", "RuntimeException", "bad name: msg"],
        ["plain error", "RuntimeException", "plain error"],
    ];

    foreach ($cases as [$input, $expectedType, $expectedMsg]) {
        $err = Codec::error($input);
        $actualType = \get_class($err);
        $actualMsg = $err->getMessage();
        if ($actualType !== $expectedType || $actualMsg !== $expectedMsg) {
            \fwrite(\STDERR, "mismatch for " . \var_export($input, true) . ": got $actualType (" . \var_export($actualMsg, true) . "), expected $expectedType (" . \var_export($expectedMsg, true) . ")\n");
            exit(1);
        }
    }
}
"#,
    )
    .unwrap();

    let out = Command::new("php")
        .arg(root.join("check_errors.php"))
        .output()
        .unwrap();
    let _ = std::fs::remove_dir_all(&root);
    assert!(
        out.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
