# Contributing to rust-anything

Thanks for helping. Bug reports, fixes, new language support and write-ups of real ports are all
welcome, and you don't need to ask before opening a pull request for a bug fix.

## Before you start

- **A bug:** open an [issue](https://github.com/erkamyaman/rust-anything/issues/new/choose) with
  the command you ran, its output, your platform and the rust-anything version
  (`rust-anything --version`). A tiny repro project helps most.
- **A bigger change** (a new language, a new command, a change to what a unit or a recording is):
  open an issue first so we can agree on the shape before you write it. The reasons behind past
  choices are in [DECISIONS.md](./DECISIONS.md).
- **A security problem:** don't open a public issue; see [SECURITY.md](./SECURITY.md).

## Set up

You need Rust (stable, 1.97 or newer). The tests also run the language each part handles, and
skip a part whose runtime is missing, so install the ones you touch:

| For | Install |
| --- | --- |
| TypeScript/JavaScript | Node 24 (22.6+ works except for `profile`) |
| Python | Python 3.9+ |
| C | a C compiler (`cc`) |
| Go | Go 1.22+ |
| Ruby | Ruby 3.x |
| Java | a JDK (17+) |
| PHP | PHP 8 |
| C# | the .NET SDK |
| PowerShell | PowerShell 7.4+ (`pwsh`) |
| wasm bridges | `rustup target add wasm32-unknown-unknown` and the matching `wasm-bindgen-cli` |

```sh
git clone https://github.com/erkamyaman/rust-anything
cd rust-anything
git config core.hooksPath .githooks
cargo test
```

With [mise](https://mise.jdx.dev), `mise install` gets the same Rust, Node, Python, Go, Ruby and
Java versions CI uses, and installs the [hk](https://hk.jdx.dev) hooks in `hk.pkl` (the secrets
check above, plus `cargo fmt` and `cargo clippy` before each commit). Use it instead of the
`core.hooksPath` line.

`cargo run -- doctor` lists which of these runtimes you have, their versions, and the command
that installs or updates each one. To make a missing runtime fail its tests instead of skipping
them (CI does this), name it in `RUST_ANYTHING_REQUIRE`, e.g.
`RUST_ANYTHING_REQUIRE=node,python3,go,ruby,java cargo test`.

The hook runs `scripts/check-secrets.sh`, which refuses keys, tokens and personal home-folder
paths in what you commit. CI runs it on every file.

## Make the change

- **Every fix comes with a test that fails without it.** Run the test against the old code once to
  be sure it does.
- **Where tests go:**
  - `tests/analyze.rs`: language detection, units, risks, dependencies, `--whole-app`.
  - `tests/loop.rs`: bridge, record, diff and shadow per language, with stand-ins instead of builds.
  - `tests/native.rs`: bridges built for real, then loaded and replayed.
  - `tests/app.rs` and `tests/http.rs`: whole-app record, replay, front door and lift, for CLIs and
    servers.
  - `tests/app_fresh.rs`: `app record --fresh`, where each case starts from a snapshot of a project
    folder.
  - `tests/library.rs`: whole-library ports.
  - `tests/effects.rs`: side effects (mail sent, rows written) that make a replay fail even when
    the output matches.
  - `tests/fuzz.rs`: `fuzz` and `record --pin-clock`.
  - `tests/switch.rs`: `enable` and `disable` on real source files.
  - `tests/package.rs`: `package` layouts and an installed napi port.
  - `tests/doctor.rs`: `doctor --json` and its exit code.
  - `tests/report.rs`: what the plan says about units you cannot start with.
  - `tests/port_map.rs`: the port map, upstream changes and unreadable manifests.
  - `tests/overloads.rs`: Java, C# and Kotlin overloads kept in the original while the rest is
    bridged.
  - `tests/tools.rs`: the MCP tools and the CLI expose the same options, and `docs/tools.md`
    matches the tool list.
  - `tests/types_doc.rs`: every row of `docs/types.md` matches what the type mappings give. Update
    the table there when a mapping changes.
  - `tests/plugin.rs`: the plugin ships the same skill and MCP server, at the crate version.
  - `tests/recording.rs`, `tests/record_cwd.rs` and `tests/record_timeout.rs`: `record` flags a
    stale recording, leaves the working directory alone, and stops a case that never returns.
  - `tests/float_files.rs`: float files compared within the tolerance.
  - `tests/c_pointers.rs` and `tests/c_structs.rs`: C buffers and out-pointers, and structs passed
    or returned by value.
  - `tests/go_internal_packages.rs`, `tests/go_mod_directives.rs`, `tests/go_module_paths.rs` and
    `tests/go_record_cgo.rs`: recording Go modules (internal packages, `go.mod` directives,
    versioned paths with spaces, cgo).
  - `tests/go_unsigned.rs`: Go `uint64`, `uint` and `uintptr` keep their range in the port.
  - `tests/java_package_declaration.rs` and `tests/java_replay.rs`: Java recording with comments in
    the package declaration, and replay that tells `null` from a rejected argument.
  - `tests/ruby_recording.rs`: Ruby results that cannot be encoded.
  - `tests/powershell.rs`: a PowerShell module bridged, recorded, diffed, gated and enabled;
    skipped without `pwsh` 7.4+.
  - `tests/untyped_js_types.rs`: types for untyped JavaScript from JSDoc or a sibling declaration
    file.
  - `tests/android_device.rs`: on-device recording over adb; skipped without a device.
  - `tests/diff_missing_folder.rs` and `tests/missing_crate.rs`: commands run on a crate folder
  that is missing (e.g. `diff` not suggesting `record` when the crate folder has no manifest,
  and `disable`, `report` and `shadow` on a non-existent crate folder naming the folder and
  suggesting `bridge` first).
  - Unit tests live next to the code.
- **Fixtures** are small apps under `tests/fixtures/`. The CLI and HTTP fixtures implement the same
  commands and routes in every language (`tcli upper|len|div|words|copy|now`, `/hello`, `/upper`,
  `/div/{a}/{b}`, `/now`), so keep a new one identical in behavior to the others.
- **Generated code** (the bridges, drivers and front doors rust-anything writes) has to pass
  `clippy -D warnings` too, since `gate` runs it on users' crates.
- **Style:** match the code around you. Keep comments for what the code can't say on its own, and
  put the reasoning for a change in the pull request instead.

## Add a crate mapping

`analyze` names the Rust crate for each library the code imports from `src/crates.toml`. To add
one, append an entry under its language: `names` (exact package names) or `prefixes` (the start of
a name, for Java and C#), `crates`, `fit` (`good`, `partial` or `none`) and a `note` when the fit
needs explaining. The first matching entry wins, so put specific ones before broad prefixes. A
library missing from the table is reported as `unknown`; never guess a crate.

## Add a language

PHP and C# were added in two commits worth reading first:
[e8ac243](https://github.com/erkamyaman/rust-anything/commit/e8ac243) (PHP) and
[caea59a](https://github.com/erkamyaman/rust-anything/commit/caea59a) (C#). A new language starts
the same way:

1. A plugin in `src/lang/<name>.rs`: discovery, scan (imports and risks), resolve and exports,
   plus its libraries' crates as `[[<name>]]` entries in `src/crates.toml`.
2. Detection and dispatch in `src/lang/mod.rs`.
3. Whole-app support in `src/whole_app.rs`: entry points, servers, declared dependencies.
4. The CLI and HTTP fixtures, a library fixture, and tests in `tests/analyze.rs`, `tests/app.rs`
   and `tests/http.rs`.
5. `bridge` refuses the language's units until a bridge exists, with a clear message.

## Translate the README

Translations are welcome and community-maintained. Say which language you are taking in
[#425](https://github.com/erkamyaman/rust-anything/issues/425) so two people don't do the same one. A new one:

1. Lives in `docs/README_<CODE>.md` (for example `docs/README_PT.md`); the root `README.md` stays
   English and is never renamed or replaced.
2. Uses paths that work from `docs/`: `assets/logo.png`, `../LICENSE`, `../README.md`.
3. Starts with the same language line as the other translations, with its own language as plain
   text, followed by the note that it is community-maintained and may lag behind the English
   README (copy it from `docs/README_DE.md` and translate it).
4. Goes at the **end** of the language line, in `README.md` and in every translation. The line
   always starts English, Türkçe, Українська; the order of the languages already listed does not
   change.

## Open the pull request

Before you push:

```sh
cargo fmt
cargo clippy --all-targets -- -D warnings
cargo test
```

- Say what was wrong, how to reproduce it, and how the change fixes it.
- Link the issue (`Fixes #123`).
- Note anything you couldn't test (another OS, a runtime you don't have).
- Keep it to one change: two fixes are easier to review as two pull requests.

CI runs on Ubuntu, macOS and Windows. The Ubuntu `check` job has Node and cargo
(`RUST_ANYTHING_REQUIRE=node,cargo`); macOS adds Python, Go, Ruby, Java and the wasm-bindgen CLI, and
Windows runs the native bridge integration tests. The `powershell` job runs the PowerShell tests on
Ubuntu and Windows, and the Ubuntu and macOS images have `pwsh` too. PHP, .NET and Kotlin are installed nowhere, so their
tests skip there: run `cargo test` locally with those on `PATH` before a change that touches them
(`docs/STATUS.md`, "Where the tests run", has the same split). For a first-time contributor, GitHub
waits for a maintainer to approve the CI run, so expect a short delay before the checks start.

Pull requests are merged with their commits kept, so your history stays yours. If a change needs
small adjustments, a maintainer may add a commit on top rather than ask for another round.

## Share a real port

Ports of real projects are the best evidence of what works and what breaks. If you used
rust-anything on something real, an issue or a pull request adding it to
[docs/real-world.md](./docs/real-world.md) is very welcome. Include the project, the commands you
ran, the case counts, and anything that surprised you.

## Code of conduct

Everyone taking part is expected to follow the [code of conduct](./CODE_OF_CONDUCT.md).
