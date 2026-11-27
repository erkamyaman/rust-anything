## Languages

| Language | Unit | Bridge | Recorded and replayed by |
| --- | --- | --- | --- |
| <img src="https://rustanything.ai/langs/typescript.svg" width="16" height="16" alt=""> TypeScript | folder or file | napi-rs or wasm-bindgen with a generated binary wire; classes and async included | Node, running the `.ts` sources |
| <img src="https://rustanything.ai/langs/javascript.svg" width="16" height="16" alt=""> JavaScript | folder or file | napi-rs or wasm-bindgen with a generated binary wire; classes and async included | Node, running the `.js` sources |
| <img src="https://rustanything.ai/langs/python.svg" width="16" height="16" alt=""> Python | package or module | PyO3 (abi3), direct conversion, typed exceptions | Python, importing the original |
| <img src="https://rustanything.ai/langs/c.svg" width="16" height="16" alt=""> C | `.c` file | `extern "C"` functions with the original names and a header | the original compiled to a shared library, called through ctypes, one fork per case |
| <img src="https://rustanything.ai/langs/go.svg" width="16" height="16" alt=""> Go | package | typed cgo calls and a generated binary wire, in a package with the original API | a generated Go driver, built as its own module |
| <img src="https://rustanything.ai/langs/ruby.svg" width="16" height="16" alt=""> Ruby | file or folder of module functions | a magnus extension under the original modules | a generated Ruby driver |
| <img src="https://rustanything.ai/langs/java.svg" width="16" height="16" alt=""> Java | package of `public static` methods | typed JNI and a generated binary wire, in wrapper classes with the original methods | a generated reflection driver |
| <img src="https://rustanything.ai/langs/kotlin.svg" width="16" height="16" alt=""> Kotlin | Gradle module of top-level functions and `@JvmStatic` methods | the Java bridge: typed JNI in Java wrapper classes calling the compiled Kotlin statics | a generated reflection driver, with the Kotlin sources compiled by `kotlinc` |
| <img src="https://rustanything.ai/langs/php.svg" width="16" height="16" alt=""> PHP | file or folder of top-level functions (`public static` methods stay in PHP) | a JSON dispatcher over `ext-ffi`, in a file that redeclares the functions with the original parameters | a generated PHP driver; whole apps with `app record`, `app replay` and the front door, for CLIs and servers (`php -S`) |
| <img src="https://rustanything.ai/langs/powershell.svg" width="16" height="16" alt=""> PowerShell | `.psm1` module or `.ps1` script of functions with `param()` blocks (exported per `Export-ModuleMember`) | a JSON dispatcher over P/Invoke, declared at load time with Reflection.Emit, in a `.ps1` that redeclares each function with its original `param()` block | a generated `pwsh` driver (PowerShell 7.4+) |
| <img src="https://rustanything.ai/langs/csharp.svg" width="16" height="16" alt=""> C# | file or folder of `public static` methods | a JSON dispatcher over P/Invoke, in `<namespace>.Rust` classes with the original methods | a generated reflection driver, compiled with the project's sources by `dotnet build`; whole apps with `app record`, `app replay` and the front door, for CLIs and ASP.NET servers (`dotnet <app>.dll`) |

Every language except C, PHP, C# and PowerShell also gets batch classes (`--batch`): the items cross once and stay in
Rust, and each call runs over all of them on all cores.

The language is detected from the repository (`go.mod`, `pom.xml`, `build.gradle`/`build.gradle.kts`,
a `.csproj` or `.sln`, `pyproject.toml`, `Gemfile`, `composer.json`, `package.json`, else the most
common source files); a Gradle module with `.kt` sources and no `.java` sources is Kotlin, mixed
modules stay Java. `--lang` overrides it. Types come from the source: TS types, Python hints, C, Go,
Java, Kotlin and C# signatures, Ruby YARD tags, PHP types and phpdoc; untyped parameters used as
numbers are inferred as numbers. [Types](types.md) lists what each one becomes in the port,
and which keep a function in its language.

A Java unit is compiled with the rest of its source tree. When those sources import classes from
outside it (the module's libraries, or another module), set `CLASSPATH` to the jars and class
folders the project builds against before `record`, `diff` and `bench`; `javac` and the driver both
use it. With Gradle, print it from the module's compile classpath
(`configurations.compileClasspath.asPath`, or `<variant>CompileClasspath` on Android); with Maven,
`mvn dependency:build-classpath`. Without it, `javac` fails on the missing types and the error says
to set it.

For Android, `rust-anything build <crate> --profile android` builds the same JNI library with the
NDK for `arm64-v8a`, `armeabi-v7a` and `x86_64` (or the `--abi` values you pass) and installs it
into `<crate>/jniLibs/<abi>/`; `--api` sets the API level (minimum 26, which the generated Java
runtime needs). It finds the NDK from `ANDROID_NDK_HOME`/`ANDROID_NDK_ROOT` or the SDK, needs the
Rust targets (`rustup target add aarch64-linux-android armv7-linux-androideabi
x86_64-linux-android`), links 64-bit libraries with 16 KB page alignment, and checks that every
JNI entry point is exported. Gradle looks in `src/main/jniLibs` by default, so pass
`--jni-libs app/src/main/jniLibs` or add `<crate>/jniLibs` to `sourceSets.main.jniLibs.srcDirs`.
`rust-anything record <crate> --profile android` runs the original Kotlin unit under ART on a
connected device (adb), dexes the generated driver with `d8` from the SDK's build-tools, and pulls
the results back as the same `cases.jsonl`; `diff <crate> --profile android` replays them against
the `.so` built for the device's ABI. Both need `adb`, a device, `kotlinc`, an SDK with build-tools
and a platform, and `--device <serial>` when more than one device is connected. The profile covers
logic units: Activity/Compose/XML UI, lifecycle and SDK-only classes stay in Kotlin, and each call
crosses JNI, so prefer the generated batch wrappers for hot loops. On a host, `record` and `diff`
keep using the desktop JVM.

For PHP, `analyze` plans a PHP app (Composer packages and PSR-4 autoloading, risks
such as loose comparison and integer overflow into floats, Composer packages mapped to crates),
and a PHP CLI or server can be recorded, replayed and put behind a Rust front door. A unit's
top-level functions with scalar, string, list and assoc-array parameters are bridged through PHP's
FFI extension (`php` with `ext-ffi`, on by default for the CLI): once `diff` passes, require the
generated `php/<crate>.php` (the name is the crate with `-` written as `_`, so `foo-rs` is
`php/foo_rs.php`) where the app required the original, and leave both files where they are: the
wrapper loads its native library relative to its own folder, so moving it breaks the load. The wrapper
declares each bridged function unconditionally, with no `function_exists` guard, so it replaces the
original file rather than joining it. Whatever else that file holds cannot come along: classes,
`public static` methods, functions the bridge keeps in PHP (a `callable` or `Closure` parameter, say),
and top-level statements such as `define('FEATURE_ENABLED', true)` are not in the wrapper, and requiring
both files redeclares the bridged functions. Keep those in a file that stays required.
Each
call crosses as JSON, about 1 µs, so `bench` shows small functions slower than PHP; there is no `shadow` for PHP yet
([issue #200](https://github.com/erkamyaman/rust-anything/issues/200)).

C# works the same way: `analyze` reads `.csproj` projects and their NuGet packages
(mapped to crates), resolves `using` directives to folders, and flags what C# does differently
(`decimal`, `dynamic`, LINQ's deferred queries, `Task`, `lock`, reflection). A C# CLI or ASP.NET
server can be recorded, replayed and put behind a Rust front door. A unit's public static methods
with scalar, string, array, list, dictionary and record or plain-class parameters are bridged
through P/Invoke (`dotnet` on PATH to record and diff): once `diff` passes, compile the generated
`cs/` wrapper (one file named after the crate: `foo-rs` gives `cs/FooRs.cs`) into the project and keep
the built native library next to the app, then call
the generated classes in the `<namespace>.Rust` namespace (`Acme.Text.Slug.Slugify` becomes
`Acme.Text.Rust.Slug.Slugify`), or alias them with `using`. Each call crosses as JSON, about 1-3 µs,
so `bench` shows small methods slower than .NET; there is no `shadow` for C# yet
([issue #200](https://github.com/erkamyaman/rust-anything/issues/200)).

For PowerShell, `analyze` reads `.psm1` modules, `.ps1` scripts and `.psd1` manifests
(`RequiredModules`), follows `Import-Module`, `using module` and dot-sourced files, and flags what
PowerShell does differently: implicit output and array unrolling, case-insensitive `-eq`, `-match`
and hashtable keys, `$null`, and implicit conversions (`5 / 2` is 2.5, `[int]2.5` is 2). A unit's
functions with `[string]`, integer, `[double]`, `[bool]`/`[switch]`, array, `[hashtable]` or untyped
parameters are bridged through P/Invoke from a generated `ps/<crate>.ps1`; filters, `begin`/`process`
blocks, pipeline input, `[scriptblock]`, `[ref]` and other .NET types stay in PowerShell with the
reason. Once `diff` passes, `rust-anything enable <crate>` dot-sources the wrapper at the end of the
original file. Each call crosses as JSON encoded by PowerShell code: about 0.1 ms for a few
scalars on an Apple M4, more for arrays and hashtables, so only functions that do more work than
that per call get faster (`bench` measures it for yours). Most slow PowerShell in tools is the
start-up of a new `pwsh` or `powershell.exe` per command (0.24 to 0.45 s for
`pwsh -NoProfile -c 1` on the same M4), which no port fixes: keep one shell open and send it the
commands (see the [FAQ](faq.md)).

