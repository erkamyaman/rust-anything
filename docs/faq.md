# FAQ

Questions people ask about rust-anything. Back to the [README](../README.md).

## FAQ

**Does it rewrite my code automatically?** No. It finds what to move, generates the bridge,
records what the original does and checks the Rust against it. You or your agent write the Rust
in `port.rs`; `diff` and `gate` decide when it is right.

**Does it compare side effects, not just the output?** Stdout, stderr, the exit code, the files
the app writes and HTTP responses always. Outgoing HTTP calls (webhooks, APIs), email and the
database when you ask for them: `app record --effects http,smtp` runs local fakes and records what
reached them per case, in order; `--db-reset` and `--db-dump` reset the database before each case
and record its state after. Replay compares those too, so a rewrite that returns the same JSON but
sends an email twice, or writes different rows, fails the case. See
[Side effects](whole-apps.md#side-effects-email-webhooks-and-databases).

**What happens when the Rust code fails?** It comes back as a normal error in the caller's
language, never a crash. A port returns `Result`, and `Err("TypeName: message")` raises that type
where the language allows it:

| Language | `TypeName` is | Looked up | Otherwise |
| --- | --- | --- | --- |
| Python | a dotted class path | the module is imported (`mypkg.errors.ParseError`) | `ValueError` |
| Ruby | a constant or `::`-separated class path | `Object` down the `::` path (`Errors::ParseError`) | `RuntimeError` |
| JavaScript | a built-in error name | `globalThis` (`TypeError`, `RangeError`, …) | `Error` |
| Go | not a type: the string is the `error` value | — | — |
| Java, Kotlin | a bare or package-qualified class | the JDK packages, then its own | `RuntimeException` |
| C# | a full .NET type name | every loaded assembly (`Acme.Errors.ParseError`, a nested type as `Acme.Parser+ParseException`) | `InvalidOperationException` |
| PHP | a class name with `\` separators | `class_exists`, and it must be a `Throwable` | `RuntimeException` |
| PowerShell | a .NET type name | `-as [type]`, and it must be an `Exception` | `RuntimeException` |

The names matter beyond the message: several drivers compare the error type as well as the text,
so `Err("ParseError: bad")` against an original that raises `Acme.Errors.ParseError` fails those
cases even though the text matches. If the Rust code panics anyway, the bridge catches it at the boundary and returns an error
("Rust panic: ..."). `gate` fails a port that uses `unwrap`, `expect` or `panic!` where the original
throws, and needs a recorded failing case for every error path, so errors are checked against the
original like any other output.

**Will my app's API change?** No. Callers import the same functions (or the shadow module, which
runs both and returns the original's result while you build confidence).

**Is Rust always faster?** No, and rust-anything says so. A small function called once per item
is often slower in Rust, because each call crosses a bridge; V8, the JVM and Go run such functions
in nanoseconds. `profile` finds the loop around it, `--batch` moves the whole loop, and `bench`
gives a verdict before you ship. Code already compiled by numba, Cython or JAX runs as machine
code too: `profile` and `analyze` flag it, since porting it buys little speed.

**Can I point it at all my tools and get them all in Rust?** Not in one step. `go` moves the one
function that takes the time in a command you run, and stops there; run it again for the next
one. A whole app moves piece by piece behind the front door ([Port a whole app](whole-apps.md)),
each piece checked against recordings before it switches on. Nothing is rewritten that a recording
can't check.

**Will it speed up a tool that runs shell commands (PowerShell, bash)?** Usually not, and the
reason is rarely the language. When a tool starts `powershell.exe` or `bash` for every command,
the time goes to starting the process, often more than the command itself, and the same command
in Rust still waits for that start. Keep one shell open and send it the commands, batch them into
one call, or call the API the command wraps (the file system, the registry, HTTP) from your code
directly. On an Apple M4, `pwsh -NoProfile -c 1` takes 0.24 to 0.45 s, longer than most commands
it would run.
Shell scripts (bash) are not a supported language. PowerShell modules and scripts are: their
functions can be bridged, recorded and switched on like any other unit, but that pays off only for
functions whose own logic is slow, since each call into Rust costs about 0.1 ms or more of
PowerShell-side encoding. `profile` does not run PowerShell workloads yet ([issue #208](https://github.com/erkamyaman/rust-anything/issues/208)
adds child-process detection, so it can say when start-up is the cost). The code around the calls
(an Electron main process, a Node or Python tool runner) can be ported as usual.

**My code is numeric; will the floats match exactly?** Often not: operation order, fused
multiply-add and the math library differ between compilers. Record with `--float-tolerance 1e-9`
(`record` or `app record`), and floats match within that share of the larger value (absolute below
1). Integers, text and structure still have to match exactly. A file of raw floats the app writes
(numpy `tofile`, `.f32` exports) is compared value by value too when you name it:
`app record --float-file out.f32=f32le` (or `f64le`).

**Can it port an app I only have as an APK, a JAR or an .exe (or Windows itself)?** No. It ports source code: it
parses the functions, records what each one does, and checks the Rust against the running
original. A compiled app has none of that to work with, and `analyze` says so. Writing a new Rust
app from a binary's visible behavior is a different project.

**Should I port battle-tested C?** Often not, and `analyze` tells you where. C that touches
hardware registers, relies on its compiler's type sizes or memory layout, or uses inline assembly
(`volatile`, bitfields, packed structs, `__asm__`, integers cast to addresses) is marked "keep in
C": a recording on your machine proves nothing about the target, and stable driver or firmware
code gains little from a rewrite. The portable logic around it (parsing, protocols, data handling)
is what moves well.

**Can it port my Electron app?** The logic, yes; the windows stay web pages. `analyze
--whole-app` spots Electron and plans the main process piece by piece while the renderer stays as
it is. Moving the UI to Tauri (the same web UI on a Rust backend) is a separate step once the logic
is Rust.

**What about C#, Kotlin or Swift?** C# is supported for whole apps, and Kotlin for Gradle modules of
top-level functions and `@JvmStatic` methods (see [Languages](../README.md#languages)); Swift is not
supported yet. Each language needs a parser, a recorder, a bridge and a replay
driver, all tested end to end. Open an issue (or upvote one) for the language you need.

**Won't the Rust be much bigger than the original?** It shouldn't be. `gate` compares the port's
lines with the original's and fails above 3x (plus 50 lines for imports and signatures): that usually means hand-writing what a crate already
does, or handling cases one by one. `analyze` lists the crates to use.

**Doesn't porting piece by piece lock the Rust into the old app's design?** Only while both run
side by side. The recordings check behavior at the app's edges (a CLI's output, a server's
responses, a module's exported functions), not its internals. Mirror the old structure while
units still cross the bridge; once `app lift` has removed the original, restructure the Rust
freely, and `app replay` still proves the behavior. Nothing calls back into the original either:
the front door runs it as a separate process (or forwards HTTP to it), bridged units are called
by the original and never the other way round, and callbacks stay in the source language.

**Does my code leave my machine?** No: rust-anything runs locally and sends none of your code
anywhere. Cargo downloads crates as usual, and your agent's model provider sees what your agent
sends it, under its own policy.

**What does it need installed?** Rust, plus the runtime of the language you port, and a few extras
that only matter for some languages, which `rust-anything doctor` lists with the command that
installs each one:

| Piece | Needed for |
| --- | --- |
| Ruby 3.0+ (not macOS's 2.6), Java 17+ | the Ruby and Java bridges; magnus and JNI do not build on older ones |
| `kotlinc` | Kotlin units, and `record`/`diff` on a host JVM |
| Android NDK, `adb`, `d8`, `kotlinc` and the three Rust Android targets | `build`/`record`/`diff --profile android` |
| `wasm32-unknown-unknown` and the matching `wasm-bindgen` CLI | `bridge --kind wasm`, and the wasm demo |
| A Node that allows `--cpu-prof` in `NODE_OPTIONS` (22.23 and 24 do, 22.14 does not) | `profile` through `npx` or `npm run` (22.6+ runs everything else, `profile -- node ...` included) |
| PHP, the .NET SDK | the PHP and C# whole-app paths |

A missing optional runtime skips its tests instead of failing them; `RUST_ANYTHING_REQUIRE` makes
them fail. See [Install](install.md#install).

**A command was stopped for running too long. Which limit is it?** Two environment variables set
the limits, in seconds (a number above zero; anything else is ignored and the default applies):

- `RUST_ANYTHING_COMMAND_TIMEOUT` limits each command rust-anything runs for you: your test
  runner, your build and the quick helper commands. The defaults are 30 minutes for tests and
  builds and 60 seconds for the quick ones. When a command passes the limit it is stopped and the
  error names this variable. A test runner in watch mode never ends, so run it once (for example
  `vitest run`) rather than raising the limit.
- `RUST_ANYTHING_CASE_TIMEOUT` limits one recorded case while the original runs, 10 seconds by
  default. An input that makes the original loop or allocate for ever is stopped and left out of
  the recording, and the others carry on; the output lists the inputs it left out.

```sh
RUST_ANYTHING_COMMAND_TIMEOUT=3600 rust-anything gate        # allow a slow test suite an hour
RUST_ANYTHING_CASE_TIMEOUT=60 rust-anything record           # allow slower cases 60 s each
```

**Something broke.** Open an issue with the command, its output and your platform.
