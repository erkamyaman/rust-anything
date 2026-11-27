# Changelog

## Unreleased

- `record`, `diff` and `bench` on a Java unit use the `CLASSPATH` variable for classes from outside
  the source tree (a Gradle or Maven module's libraries). `javac` already read it, but the driver
  ran with its own `-cp`, so every case recorded `NoClassDefFoundError` instead of a result. When
  `javac` fails on missing packages or symbols, the error now says to set `CLASSPATH`
  (#744, #765 by @Layfate).
- `record` on Windows reports how much of a TS/JS original the cases ran, as it does elsewhere.
  V8 names files `file:///C:/...` and the unit's files were compared as `\\?\C:\...`, so no file
  matched and the coverage line was missing (#763 by @Layfate).
- The repository checks text files out with `\n` line endings on every OS (`.gitattributes`).
  Git for Windows turns on `core.autocrlf` by default, which gave `docs/tools.md` and
  `skills/rust-anything/SKILL.md` `\r\n` endings, so `tests/tools.rs` and `tests/skill.rs` failed
  on a fresh Windows clone (#761 by @Layfate).
- `map` names a crate folder whose `rust-anything.json` does not parse with `/` between folders
  on Windows too (`rust/util-rs`), as it names every readable port, instead of `rust\util-rs`
  (#760 by @Layfate).
- `profile` on a Java workload keeps JFR's "Started recording" banner off stdout
  (`-Xlog:jfr+startup=error`). Every JVM the workload starts inherits the JFR options, so a build
  tool that runs `java` to read a JDK's version, as Gradle does for its toolchains, failed on the
  banner with `Unable to determine version for JDK` before the build began
  (#742, #759 by @Layfate).
- `profile` on a Java workload reads all of `jfr print`'s output. It kept only the last MiB, so a
  recording of more than a second or two failed with `jfr output: expected value` (#759).
- `go` says "captured 1 real call" / "N real calls", "recorded N cases (N threw in the original)",
  and "gate: 1 check fails" / "N checks fail" instead of "(s)" and "throw"
  (#732, #757 by @Papito084).
- `go` retry prompt says "1 distinct difference" for one difference and "N distinct differences"
  for more, instead of "N distinct difference(s)", and "1 case" instead of "1 case(s)"
  (#731, #756 by @Papito084).
- `record --from-tests` says "Captured 1 call from the tests" for one call and
  "Captured N calls from the tests" for more, instead of "Captured N call(s)"
  (#733, #755 by @Papito084).
- `record` says "Left out 1 input the original ran too long on" for one timed-out
  input and "Left out N inputs ..." for more, instead of "Left out N input(s)"
  (#734, #754 by @Papito084).
- `fuzz --keep` says "Kept 1 case in ..." for one kept case and "Kept N cases in ..."
  for more, instead of "Kept N case(s)" (#735, #753 by @Papito084).
- `enable --only` says "unknown export in ..." for one unknown name and "unknown exports in ..."
  for more, instead of "unknown export(s)", and "requested function does not match" for one
  failing function and "requested functions do not match" for more (#736, #752 by @Papito084).
- `app record --no-fresh` keeps the `files/` snapshot when a case that stays reads it through a
  folder above it (`fs.base = "."`), not only through one inside it (#739, #749 by @XiaoBinGan).
- `analyze`'s empty-plan warnings say "1 TypeScript/JavaScript file under ... is excluded" and
  "only 1 compiled or packaged file" for one file, instead of "1 ... files ... are"
  (#738, #746 by @brixtonpham).
- `npx rust-anything@latest setup` now leaves agents that keep working: a copy run from npx's
  cache sets each agent to start `npx -y rust-anything@<version> mcp` (`cmd /c npx ...` on
  Windows) instead of the cached binary, which npm may delete. `doctor` reads that entry as
  current.
- A GitHub Action runs `rust-anything gate` in CI on every push:
  `uses: erkamyaman/rust-anything@main` with `crates:` listing the crate folders. It installs
  rust-anything from npm and fails the job when any crate fails the gate.
- `analyze` also counts `Number.parseInt` and `Number.parseFloat` calls as `Number semantics`,
  alongside global `parseInt` and `parseFloat` (#730 by @brixtonpham).
- `--db-dump` output is recorded with `\n` line endings: on Windows, Python's `print` and `sqlite3`
  end lines with `\r\n`, so `.effects` mixed both and a recording made there failed replay on Linux
  and macOS. Recordings made on Windows with `--db-dump` before need a fresh `app record`
  (#729 by @Layfate).
- The Android 16 KB page-size check says "1 LOAD segment is not 16 KB aligned" for one
  misaligned segment and "2 LOAD segments are not 16 KB aligned" for more, instead of
  "LOAD segment(s) are" (#718, #728 by @MannXo).
- `app record --fresh` writes the files a case removed (`<case>.removed`) with `/` between folders
  on Windows too, so a recording made on Windows replays on Linux and macOS instead of failing on
  `stale\old.txt`. Recordings made on Windows before need a fresh `app record`
  (#727 by @Layfate).
- `gate` starts a failed check's extra detail lines and its `fix:` line in the same column as the
  first line of its detail, instead of a fixed 12 spaces that no longer lined up after #707
  (#713, #725 by @Layfate).
- `gate`'s `ported` check says "1 place in port.rs" for one stub and "N places" for more,
  instead of "1 place(s)" (#712, #724 by @Layfate).
- `analyze` also counts `toLocaleString`, `toLocaleDateString` and `toLocaleTimeString` calls as
  `Intl and locale-aware strings`: they format numbers and dates with the same ICU data as
  `localeCompare` (#714, #722 by @chuenchen309).
- `analyze` no longer flags `new Date(-1)` (a negative number literal) as `Date parsing`
  (#715, #721 by @brixtonpham).
- `app record` no longer keeps a separator that two runs' random values happen to share: request
  ids `k2Pq_x9Lm4` and `Zt7w_bR3nV` now record as `[..]` instead of `[..]_[..]`, so a replay whose id
  has no `_` passes. A separator next to a part both runs agree on stays, so `id=[..]` keeps its
  `=`. Recordings made before keep their wildcards until the next `app record`
  (#680, #692 by @XiaoBinGan).
- `analyze` flags three more TS/JS risks: `Intl and locale-aware strings` (high: `Intl.*`,
  `localeCompare`, `toLocaleLowerCase`, `normalize("NFC")`), `Date parsing` (high: `Date.parse`,
  `new Date(string)`) and `URL and encoding APIs` (medium: `new URL`, `URLSearchParams`,
  `TextEncoder`/`TextDecoder`, `atob`/`btoa`), each with guidance on the traps a port must match
  (#678, #711 by @antont).
- On Windows, a debug build of the CLI (`cargo run`, and every test that runs the binary) no
  longer overflows the 1 MB main-thread stack before it parses its arguments: the CLI runs on a
  thread with an 8 MB stack on every OS. Release builds were not affected (#710 by @Layfate).
- `package` refuses `--bin` and `--original` on a napi, wasm or PyO3 crate, and `--fallback` on a
  wasm or PyO3 crate, naming the flag and what it is for, before anything is written (also with
  `--dry-run`). They used to be dropped silently (#701, #708 by @Papito084).
- `gate` starts every check's detail in the same column, padding names to the longest one, so
  `error cases` and `known diffs` line up with the rest (#700, #707 by @Papito084).
- `gate` says "the 1 function that fails in the original has a recorded failing case" instead of
  "each of the 1 functions that fail" in the `error cases` check (#698, #706 by @Papito084).
- `app drift` prints "1 case; build: ..." instead of "1 cases" for a one-case recording
  (#699, #705 by @Papito084).
- A napi export called without an optional structured argument, such as an array, now gets
  `None` on the JSON path that async calls always take, instead of failing with
  `invalid type: map, expected a sequence` before the port runs. A `serde_json::Value` parameter
  still receives the `undefined` marker (#681, reported by @antont).
- The MCP `app_replay` tool refuses a `build` or `case`, and `app_drift` a `build`, that is not a
  string with `` `case` must be a string, not 5 ``, instead of dropping it and replaying every case
  or the original. A missing or `null` value still means not given
  (#682, #691 by @susheelg1197).
- The MCP `package` tool refuses a `name`, `version`, `fallback` or `original` that is not a
  string with `` `version` must be a string, not 2 ``, instead of dropping it and packaging with the
  crate's own value. A missing or `null` value still means not given
  (#683, #690 by @susheelg1197).
- The MCP tools refuse an optional path (`out`, `inputs`, `module`, `resolve_from`, `ndk`,
  `jni_libs`) that is not a string with `` `out` must be a string, not 5 ``, instead of dropping
  it and using the default. A missing or `null` path still means not given
  (#686, #689 by @susheelg1197).
- Every MCP tool's required path (`root`, `repo` or `crate`) that is there but not a string is
  refused with `` `root` must be a string, not 5 `` instead of `` missing `root` ``. An absent or
  `null` path still says `` missing `root` `` (#685, #688 by @susheelg1197).
- The MCP `bridge` and `app_wire` tools refuse a `unit` that is not a string with
  `` `unit` must be a string, not 5 `` instead of `` missing `unit` ``, and `bridge` refuses a `name`
  that is not a string instead of writing the bridge under the default name. A missing or `null`
  `unit` still says `` missing `unit` ``. A wrong-type argument to any MCP tool now gets the
  `invalid_arguments` code, not retryable, instead of `failed` (#684, #687 by @susheelg1197).
- `docs/whole-apps.md` says six of the seven `app` commands are MCP tools and that `app shadow` is
  CLI only, instead of "All six are MCP tools too" (#672, #674 by @XiaoBinGan).
- The MCP `app_record` tool refuses a `ready`, `name`, `db_reset` or `db_dump` that is neither a
  string nor `null`, with `` `ready` must be a string, not 42 ``, instead of recording as if it had
  not been given. A `null` still means not given (#671, #673 by @XiaoBinGan).
- `app record --http --inputs` records every input, including one that repeats an earlier request.
  Cases run in name order against one server, so a read after a write is a case of its own; it
  used to be dropped as a duplicate, and the read the recording was made for was never recorded.
  Examples and generated traps are still deduplicated. `app replay`, and the replays `app wire`
  and `app lift` run, send HTTP cases in that same case-name order: sorting file names put
  `add-again` before `add`, so a server with state answered differently than when recorded
  (#677 by @antont).
- The generated `src/utf16.rs` has `slice_units`, JS `slice` on code units with a cut surrogate
  pair kept, and `bridge`'s string-offsets note says to return such a substring through
  `js::utf16_string(utf16::slice_units(..))`. `utf16::slice`, which the note pointed to before,
  turns a lone surrogate into U+FFFD, so a port that followed it failed `diff` on the cases the
  helpers exist for (`head("😀")` → `{"$utf16":[55357]}` in JS) (#676 by @antont).
- A napi port function returning `Option<T>` answers `None` as `null` when its TS export declares
  `T | null`, also through a type alias and `Promise<T | null>`. A call the binary wire refuses
  (such as `null` for an optional array), and every async call, used to get `undefined`, so
  `diff` failed with `expected {"ok":null}, got {"ok":{"$undefined":true}}`; the wire already
  answered `null`. A declared `undefined` still answers `undefined` (#675 by @antont).
- The table of saved settings in `docs/whole-apps.md` names the flag that clears `http`
  (`--no-http`), `http.port` (`--port 0`), `http.args` (`--no-args`) and `http.fresh` / `fresh`
  (`--no-fresh`) instead of saying they cannot be cleared
  (#665, #668 by @alvaroalonsoparrondo-a11y).
- The MCP `app_record` tool called with `http: false` and a `ready` request says
  `--no-http cannot be combined with --http, --port, --arg, --pace, --ready-timeout or --ready`;
  the message used to leave out `--ready`, the one setting the caller had passed
  (#666, #667 by @alvaroalonsoparrondo-a11y).
- `app record --ready` refuses a value that is not `/path` or `METHOD /path` with an uppercase method,
  such as `health`, `get /health` or `GET /health extra`, before the server starts, instead of
  failing after the whole `--ready-timeout`. The MCP `app_record` tool checks it the same way
  (#657, #659 by @susheelg1197).
- `app record --ready "GET /health"` waits, after the server's port opens, until that request
  answers 2xx, so a framework that opens its port before its routes work is not recorded answering
  503. The request is saved with the cases and used by `app replay`, `app shadow` and the front
  door's own start of the original, within `--ready-timeout`; the MCP `app_record` tool takes it
  as `ready` (#332, #656 by @XiaoBinGan).
- `gate`'s literals check no longer counts recorded error text. A port has to reproduce the
  original's error messages, and the runtime often writes them (`ZeroDivisionError: division by zero`),
  so they were wrongly flagged as copied answers and failed the demo gate on macOS.

## 5.0.1 (2026-11-26)

- rust-anything is now dual licensed under MIT OR Apache-2.0, the usual license for Rust projects. You
  can keep using it under MIT exactly as before; Apache-2.0 adds an explicit patent grant. `LICENSE` is
  now `LICENSE-MIT`, next to a new `LICENSE-APACHE`, and both ship in the crate, the npm package and
  the release archives.
- `map` counts units, not crates, in its first line: one unit bridged twice (napi and wasm, or a
  second `bridge --out`) reads `1 of 1 unit has a Rust port` instead of `2 of 2 units have`. The
  Ports table still lists every crate (#609 by @Aadithya-kl).
- `fuzz --time` given a negative or non-finite value says
  `` `--time` must be 0 or more seconds (0 runs one round), not -1 `` instead of
  `` `--time` takes seconds ``, and `--time -1` (with a space) gets the same message instead of clap's
  `unexpected argument '-1' found`. The MCP `fuzz` tool's `time` says the same (#612).
- `bridge --as-json Q` stops before writing anything when no export declares `Q` as a type
  parameter, with ``--as-json `Q` is not a type parameter of `src/math.ts` (type parameters: T)``
  (or "it has none"), instead of ignoring the name and leaving the generic export in TS, as
  `--keep` already does for unknown exports (#608).

## 5.0.0 (2026-11-25)

The old program is the judge, now with sharper eyes: every mismatch says where it starts and
which bug it belongs to, live traffic can be shadowed, drift is checked in CI, and `gate` catches
ports that copy the recording instead of computing it. Ideas from Anthropic's write-up on moving
the Messages API to Rust.

- `bridge` on a crate folder that already has a port names the next step: run
  `rust-anything refresh <crate>` to regenerate the bindings, pass `--out` or `--name` for a new
  crate, or remove the folder to start over (#568, #599 by @alvaroalonsoparrondo-a11y).
- `bridge --kind napi` (or `wasm`) on a unit that is not TS/JS stops before writing anything, with
  `--kind napi is for TS/JS units; mathy.py is Python and is bridged with PyO3`, instead of writing a
  napi crate for a Python unit that only fails at `diff`. The `--kind` help says it is for TS/JS units
  (#575, #598 by @alvaroalonsoparrondo-a11y).
- `--float-tolerance -0.5` (with a space) on `record`, `diff` and `app record` gets the range message
  instead of clap's `unexpected argument '-0' found`, and `diff` checks the tolerance before it
  reads or runs anything (#574, #597 by @alvaroalonsoparrondo-a11y).
- `diff` names where each mismatch first differs, as `at` in the JSON: the channel (`return`,
  `throw`, `return vs throw` or `call N` for a class scenario), the JSON path (`$.items[2].name`),
  the kind (`value`, `type`, `missing key`, `extra key`, `length`) and, for strings, the character
  with a little context around it. `app replay` does the same for each failing field: the line
  and column of text output, the JSON path in a JSON body, the status change (`200→500`).
- `diff`, `app replay` and `report` group the failing cases that differ the same way into
  distinct differences, each with a short stable id and a count, so 40 mismatches from one bug
  read as one line. `rust/app-cases/replay-report.md` gains a "Distinct differences" table before
  the failing cases, `report` a "Remaining differences" table (and `differences` in its JSON)
  while cases still differ, and `go` hands the agent each difference once with an example and
  where it starts, instead of the first five raw mismatches.
- `diff --redact` and `app replay --redact` (MCP `redact: true`) print mismatches as shapes only:
  where they differ, types, lengths and hashes, never arguments, outputs or request paths. The
  replay report is written redacted too, so it can be pasted into an issue or handed to another
  agent.
- `app shadow` checks a server build on live traffic: a proxy answers every request with the
  original, then sends it to the build and compares status, headers and body with the saved
  normalizers. The build's outbound HTTP and mail go to local sinks and its `--fresh` files to a
  copy of its own, so no side effect happens twice. Mismatches are logged to
  `rust/app-cases/shadow/mismatches.jsonl` as redacted shapes (where each field first differs,
  never values) and grouped into distinct differences at the end; `--keep` saves the requests in
  `app record --inputs` format. `--upstream` uses an original that is already running,
  `--requests` stops after a count. Servers only, no MCP tool.
- `app drift [REPO] [--build CMD]` (MCP `app_drift`) checks that the recording still describes
  the original and the port still matches: it replays the original against `rust/app-cases`
  (cases it no longer matches changed upstream), lists examples in its docs that no case records,
  and replays the build (`rust/app`, built with cargo, by default), marking each case "original
  changed, port still old" or "port regression". It writes `rust/app-cases/drift-report.md`,
  leaves `replay-report.md` as it was, and exits 1 on any change. `docs/whole-apps.md` has a
  GitHub Actions workflow that runs it on a schedule.
- `gate` has a cheat guard. `literals` fails when the port writes three or more recorded arguments
  or results (strings of 8 or more characters, numbers with 5 or more significant digits) that the
  original's source never spells out, or names `cases.jsonl`, `known-diffs.jsonl` or
  `recorded-from.json`; one or two are named and pass, and `// recorded-literal: <reason>` on a line
  lets a library constant stay. `tests` fails when the crate has fewer `#[test]`s or assertions, or
  more `#[ignore]`s, than at its last passing gate (kept in `.rust-anything/tests.json`). `record`
  now also saves the number and hash of the recorded lines in `recorded-from.json`, and `recording`
  fails when one of them was edited or removed; appended cases (`fuzz --keep`) still pass, and older
  recordings are not checked until recorded again. `go`'s TASK.md says so.
- `rules` keeps port rules in `rust/RULES.md`: `rust-anything rules <repo>` lists them and
  `--add "<rule>"` appends one (an exact duplicate is skipped; a crate folder stands for its
  repository). `go` puts them in TASK.md (and so in every retry prompt), both MCP prompts end with
  them, and the MCP server's instructions, the `rules` tool and the skill tell agents to read them
  before porting and to add each correction the user makes.

- Docs and tests: translated READMEs link the Types and packaging pages, CONTRIBUTING lists every
  test file, the FAQ states the PHP and .NET requirements, `build` and `refresh` help covers every
  language, and new unit tests cover `c_literal`, `local_module_version`, `shell_quote` and the
  base64 helpers (#564 by @Lewandox, #578 by @XiaoBinGan, #585 to #596 and #600 to #603 by
  @alvaroalonsoparrondo-a11y).

### Upgrading

- `rust/app-cases/replay-report.md` gains a "Distinct differences" section and an `at` line above
  each failing field, so a committed report differs once after upgrading: replay and commit it
  again.
- `gate` has new `literals` and `tests` checks and an extended `recording` check, which can fail
  ports that passed before. A constant the original really takes from a library can stay with
  `// recorded-literal: <reason>` on its line; to accept fewer tests on purpose, delete
  `<crate>/.rust-anything/tests.json` and run `gate` again to save a new baseline.

## 4.2.0 (2026-11-25)

- MCP boolean arguments (`force`, `dry_run`, `whole_app`, `ignore_key_order`, `pin_clock`, `partial`,
  `traps`, `http`, `library`, `check`, `skip_gate`) given a non-boolean such as `"false"` or `1` are
  refused with `` `<key>` must be a boolean ``, instead of silently falling back to the default
  (#563 by @Lewandox).
- `app record` and `app replay` without a fixed `--port` pick a port that is free on both `127.0.0.1`
  and `[::1]`, and try another one if something takes it before the server starts, instead of
  failing now and then with `port <N> is already in use` (#562 by @Lewandox).
- `bench --min-ms 0` and `report --min-ms 0` (and MCP `bench`/`report` with `min_ms: 0`) are refused
  with `` `--min-ms` must be at least 1 ``, like `fuzz --cases 0`, instead of timing a single pass
  (#561 by @ardazeybek).
- A failed MCP tool call whose program is missing (`` `cargo` not found on PATH ``) gets the
  `missing_tool` code with `rust-anything doctor` as the next step, instead of a generic `failed`
  (#557 by @Lewandox).
- `record --count 0` (and MCP `record` with `count: 0`) without `--inputs` or `--from-tests` stops
  with an error before touching `cases.jsonl` or the manifest, like `fuzz --cases 0`: it used to
  replace the recording with an empty file and exit 0. `--count 0 --inputs <file>` still records
  just the given cases (#539, #546 by @alvaroalonsoparrondo-a11y, with the early check and a test
  from #556 by @ardazeybek).
- `enable --partial` and `enable --only` (without `--force`) on a crate with no `cases.jsonl` say
  ``no cases.jsonl in <crate> (run `rust-anything record` first)`` like plain `enable`, instead of
  the bare `No such file or directory (os error 2)` (#540, #545 by @alvaroalonsoparrondo-a11y).
- MCP tools refuse a numeric argument of the wrong type, a negative one or one out of range
  (`count`, `rounds`, `cases`, `seed`, `min_ms`, `time`, `float_tolerance`, `port`, `pace`,
  `ready_timeout`) instead of silently using the default; a `port` above 65535 is an error rather
  than no port (#543 by @Lewandox).
- The "not found on PATH" check behind `build`, `gate` and the other outside-tool steps looks up a
  program the way it is spawned: on Windows only the name and `<name>.exe` count (a bare `.cmd`
  script no longer does), and on Unix a relative `PATH` entry is resolved from the command's
  working directory (#541 by @Lewandox).
- MCP `bridge` refuses a `kind` other than `"napi"` or `"wasm"`, `record`, `build` and `diff` refuse a
  non-string `profile`, and `fuzz` refuses a non-string `keep`, instead of silently falling back to
  the default (#530 by @Lewandox).
- `bridge --name` (and MCP `bridge`'s `name`) is refused before any folder is written when it is
  not a valid Cargo crate name: empty, padded, a path such as `../escape`, a leading digit, a Rust
  keyword or a reserved Windows file name. A unit whose file name starts with a digit or has no
  letters gets a valid default name (`unit-123-rs`, `unit-rs`) (#529 by @Lewandox).
- `build`, `gate` and the other steps that run an outside tool say `` `cargo` not found on PATH ``
  with `doctor`'s install command when the program is missing, instead of the raw `No such file or
  directory (os error 2)`; `gate` no longer tells you to fix a compile error when the build never
  started (#498, #525 by @Lewandox).
- `bridge --keep` refuses a name that is not an export or class of the unit, listing the ones it
  has, before writing anything: a typo used to bridge the export it meant to keep. A bare
  `--keep ' add '` is trimmed like `--keep "add: reason"` already was (#499, #522 by
  @alvaroalonsoparrondo-a11y).
- `--batch function:param` with an unknown function or parameter lists the ones `src/port.rs` has
  (`(functions: add)`, `(parameters: a, b)`), and an empty side (`add:` or `:a`) gets the same
  "write it as function:param" error as a spec without a colon (#504, #521 by
  @alvaroalonsoparrondo-a11y).
- `enable --only` (and `--partial`) with an unknown name lists the unit's exports after the unknown
  ones, so the right spelling is in the error (#500, #520 by @Papito084).
- `package --version` (and MCP `package`'s `version`) is refused before anything is written,
  `--dry-run` included, unless it is `MAJOR.MINOR.PATCH` with an optional `-prerelease`: an empty,
  padded or malformed version used to land in the package and fail only at publish time (#502, #519 by
  @alvaroalonsoparrondo-a11y).
- `gate`'s `size` check and `go`'s "no hot function to port" message say `1 line` and
  `1 hot function` for one, like `analyze`'s plan (#503, #516 by @Papito084).
- The `port_unit` and `port_whole_app` MCP prompts strip the skill's front matter when `SKILL.md`
  has CRLF line endings, as on a Windows checkout with `core.autocrlf=true` (#501, #515 by @Papito084).
- Generated Go, Java, Kotlin and Ruby bridges pass a zero-argument fallible port function straight
  to `run`/`guard` (`run(port::pong)`) instead of wrapping it in a closure, so freshly bridged code
  passes `gate`'s clippy `redundant_closure` check again (#310, #487 by @Lewandox).
- `app record --no-http` turns a saved server recording back into a CLI, `--no-args` and
  `--no-fresh` clear the saved `--arg` and `--fresh` lists (removing an unused snapshot), and
  `--port 0` drops a saved fixed port; MCP `app_record` takes `http: false`, `args: []`, `fresh: []`
  and `port: 0` the same way. A new run command re-infers an inferred app name, while one given
  with `--name` is kept (#330, #486 by @Lewandox).
- `app record --timeout 0` and `app replay --timeout 0` (and MCP `app_record` / `app_replay` with
  `timeout: 0`) are refused before anything runs, instead of killing every run at once; a saved
  `timeout_secs` of 0 is refused too, with a hint to pass `--timeout` (#497, #514 by @aloneangel45).
- `app record --ready-timeout SECS` (and MCP `app_record`'s `ready_timeout`) sets how long a server
  start waits for its port instead of a fixed 60 seconds. It is saved with the cases and used by
  replay and the front door; 0 restores the default (#332, #478 by @XiaoBinGan).
- `tests/skill.rs` fails when the skill names a command the CLI does not have, and `scripts/skill-eval/`
  compares skill versions on a fixed port task, scored by `gate` (turns and cost from the agent's JSON).
- `analyze` prefers a matching sibling source file over a directory index for extensionless
  TypeScript and JavaScript imports, as TypeScript's bundler resolution does
  (#476, #477 by @isnaenihidayat).
- Ruby ports turn `Err("Mod::Error: message")` into that namespaced exception, and Java, Ruby and
  PHP replays load the project's own classes, so a port that returns a project-defined exception
  matches the recorded `error_type`. The Ruby wrapper reopens each receiver with `class << Mod`,
  so it also loads over a class the original defined (#372, #457 by @SPD-RYUK).
- `app record` spreads CLI and HTTP traps round-robin across every base case, deduplicates traps
  before capping at 300, always keeps the HTTP unknown-route probe (`GET /rust-anything-missing`),
  and reports how many traps were dropped (`traps_dropped` in JSON and CLI output) (#335, #454 by @Lewandox).
- HTTP cases support binary request bodies via `body_base64` and `body_file` in `--inputs` and case
  `.toml` files, `curl --data-binary @file` reads raw bytes instead of UTF-8 text, and `curl -F` /
  `--form` builds deterministic `multipart/form-data` uploads (#336, #452 by @Lewandox).
- `app record --effect-response 'METHOD /path=STATUS [BODY|@FILE]'` (and MCP `app_record`'s
  `effect_response`) lets the HTTP effect sink answer matching requests (supporting `*` wildcards)
  with canned statuses and bodies, saved with the cases so replay sees identical replies
  (#334, #451 by @Lewandox).
- `bench`, `report` and the MCP `bench` tool leave functions whose recorded cases do not all match
  out of the overall speedup and the advice, and name them: a port that still panics was timed at
  the cost of catching the panic, which pulled the verdict down (#341, #470 by @AyhamJo7).
- `app record` keeps the saved effects a run does not name: `--effects` replaces the kinds,
  `--effect-env` adds or replaces one variable, and `--db-reset`/`--db-dump` replace only
  themselves, so re-recording with one flag no longer drops the HTTP sink and its variables
  (#331, #468 by @XiaoBinGan).
- Java bridges ordered sets (`SortedSet`, `NavigableSet`, `TreeSet`, `LinkedHashSet`) as arrays
  across the reflection converter and binary wire, and refuses unordered `Set` and `HashSet` with
  "an unordered collection" instead of treating them as project classes (#366, #449 by @Lewandox).
- `docs/types.md` lists, for each language, what each source type becomes in `port.rs` and the
  reason a type keeps a function in its language; a test checks every row against the code
  (#373, #456 by @puretechteam).
- Docs: the README unit-port caption and the FAQ runtime extras table include PowerShell (#505,
  #506), the translated READMEs got a `+ add yours` language link (#513), the PHP and C# unit
  bridges are documented in the Azerbaijani, Uzbek, Turkish, Ukrainian, Simplified Chinese and
  Korean READMEs (#479, #480, #481, #482, #483, #484), the `port.rs` comment is translated in the
  Vietnamese, Kazakh, Kyrgyz and Turkmen READMEs (#485), and the Turkish, Ukrainian, Azerbaijani,
  Vietnamese, Hindi and Arabic READMEs mention Gradle and Kotlin detection (#507, #508, #509,
  #510, #511, #512), by @Papito084; whole-app support for any language (#445), the
  `rust/app-cases/rust-anything.json` settings (#446) and the Japanese Gradle and Kotlin note
  (#444) by @Papito084; the Persian, Kazakh, Kyrgyz, Turkmen, Tatar, Uyghur and Uzbek READMEs
  mention Gradle and Kotlin detection (#547, #548, #549, #550, #551, #552, #553) and include
  PowerShell in the unit-port caption (#559, #560), by @alvaroalonsoparrondo-a11y; the Arabic,
  French, Spanish, German, Italian and Portuguese captions (#555 by @Lewandox) and the Turkish and
  Azerbaijani ones (#558 by @ardazeybek) do too; the Japanese README says PHP and C# units port
  now (#460 by @XiaoBinGan); the Simplified and Traditional Chinese READMEs mention Gradle and
  Kotlin detection (#471, #473 by @hkhkkh); a Thai README was added (#475 by @matavanary); and the
  plugin directory copy of `SKILL.md` is back in sync (#474 by @AyhamJo7).
- Tests: end-to-end Python and Node profile workloads (#447) and a report on a built crate (#459)
  by @Papito084; the busy-port and exiting-build messages (#458) and a check that a forked server
  is gone after record (#544) by @XiaoBinGan.

## 4.1.0 (2026-11-24)

- `enable --only <fn>` (repeatable) switches on just the named functions of a unit, and `--partial`
  switches on every function whose recorded cases all match, so a unit with many exports can go
  over one function at a time. Running it again widens or narrows the marked block in place (#351, #412 by @Lewandox).
- `report` takes `--resolve-from <dir>` like `bench` and `fuzz`, in the CLI and the MCP tool, for
  repositories whose `node_modules` lives elsewhere (#349, #402 by @XiaoBinGan).
- `analyze`'s Start here details name the sibling a unit imports in the same package, which is what
  puts it in a later wave (#348, #422 by @XiaoBinGan).
- A Kotlin unit that takes or returns `ByteArray`, `ShortArray` or `FloatArray` says that the Java
  wire cannot carry that array type yet, instead of calling it a class (#369, #386 by @XiaoBinGan).

- `bridge --batch` on a C unit, and `refresh --batch` on a C, PHP, C# or PowerShell crate, fail with
  "`--batch` is not available for <language> units yet" instead of saving a batch that generates
  nothing (#434, #440 by @SPD-RYUK).
- Java and Kotlin ports turn `Err("Type: message")` into the same exception on the JSON, typed, wire
  and batch paths, and look in `java.time` and `java.util.concurrent` too, so a typed call no
  longer throws `RuntimeException` for `DateTimeException` (#359, #438 by @SPD-RYUK).
- `report` says "Nx slower" for a port slower than the original in the card's `aria-label` and in
  REPORT.md's speedup rows, instead of "0.0x faster" and `0.0x` (#433, #437 by @draarivpatel-ui).
- `analyze` and `map` count lines of code the way `gate`'s size check does, leaving out comment
  lines, so licence headers and JSDoc no longer move a file's Lines, Start here or priority (#342, #435 by @AyhamJo7).
- `profile` and `go` on a Node that refuses `--cpu-prof` in `NODE_OPTIONS` (22.14) pass it to a `node`
  command as arguments, and stop with a clear message for wrappers such as `npx` (#346, #429 by @AyhamJo7).
- `profile` leaves a script's top level (Python's `<module>`, Node's line-1 module function) out of
  "What to port" and no longer advises porting it with a function it calls in a loop (#344, #428 by @AyhamJo7).
- Java and Kotlin parameters accept covariant wildcards and projections (`List<? extends CharSequence>`,
  `List<out CharSequence>`, `Map<String, ? extends Integer>`, `Array<out String>`) through both the
  reflection converter and the binary wire; return positions, `? super T` and `in T` stay refused (#367, #439 by @Lewandox).
- `app record --effects smtp` records each email as its decoded `From`, `To`, `Cc` and `Subject`
  and its MIME parts (attachments as name, type, size and SHA-256), so the same mail sent with
  different MIME framing matches. `--effects smtp-raw` keeps the raw DATA, and cases recorded
  before this keep replaying against the raw form until recorded again (#333, #426 by @Lewandox).
- Go ports map matching error messages back to the package's exported sentinel errors, so callers using `errors.Is` keep working (#365, #413 by @Lewandox).
- `record --ignore-key-order` saves the choice with the crate, so `gate`, `enable`, `report` and
  `go` compare objects regardless of key order too; `--no-ignore-key-order` undoes it (#352, #409 by @XiaoBinGan).
- `map` gives each unit without a port `analyze`'s status and lists the blocked and kept ones
  with the reason, so they no longer read as work left to port (#350, #421 by @XiaoBinGan).
- On Windows, `profile` and `go` find the project's own functions in a Node workload: Node's
  `file:///C:/...` frame URLs kept the slash before the drive letter, so no frame matched the
  project root and `go` stopped with `no hot function to port` (#407, #408 by @Momentum96).
- `app record` and `app replay` fail with `port <port> is already in use` before spawning a server
  on an occupied port and check whether the child exited before treating the port as ready (#326, #395 by @brixtonpham).
- `Err("Outer+Inner: message")` in a C# port throws the nested exception the recording names,
  like `Acme.Parser+ParseException`, instead of falling back to `InvalidOperationException` (#363, #423 by @draarivpatel-ui).
- Recording again removes the cases the last recording wrote and this one did not (an input that
  was removed or renamed, or a case that timed out), for CLIs and in `http/`, so replay no longer
  compares against them. Cases added by hand are kept and named in the output, as are the files
  a kept case reads through `fs.base`; switching between a CLI and `--http` removes nothing (#329, #420 by @puretechteam).
- `app record --http` and `app replay` stop the whole server when it was started through a wrapper
  (`npm start`, `sh -c`, a `.bat` file): every process it started, waiting until all are gone.
  Ctrl-C during `app record --http` still stops the server too.
  The real server used to keep running, so the next run met it on the port and a replay could
  pass against it (#325, #419 by @puretechteam).
- `record`, `fuzz`, `diff` and `bench` pass `--experimental-strip-types` to Node 22.6 to 22.17 (and
  23.0 to 23.5), which only strip TypeScript types with that flag, so TS units no longer fail with
  `ERR_UNKNOWN_FILE_EXTENSION` on Node versions `doctor` accepts (#340, #418 by @puretechteam).
- `app record --http` and `app replay` find a server that listens only on `::1` (a `localhost`
  bind can get IPv6 only) and send its requests there, instead of waiting 60 seconds and failing
  with `did not listen within 60s` (#327, #417 by @puretechteam).
- `app record --db-reset` and `--db-dump` run in the folder a CLI case runs in, as they already did
  for servers, so a database the app keeps next to it is the one reset and dumped, and the
  repository's own copy is left alone. Scripts they name by a relative path with a script
  extension (`reset.py`, `"my scripts/seed.sh"`, `< schema.sql`) still come from the repository;
  output redirects (`> dump.sql`, also unspaced) stay in the case's folder, and here-document
  bodies and `#` comments are left as written. Quotes, line continuations and `( )` blocks are
  read as sh and cmd read them (#324, #416 by @puretechteam).
- `bridge` types Go `uint64` and `uint` as `u64` and `uintptr` as `usize` in `port.rs`, alone or as
  slice items. They were `i64`, so a result above `i64::MAX` could not go back to Go and failed
  `diff` (#357, #414 by @AyhamJo7).
- Docs: the FAQ says how each language reads an error type (#387), the MCP `analyze` and `fuzz`
  descriptions name every language (#392), the risks table lists only what `analyze` detects
  (#424) and how readiness, priority and Start here are scored (#436), by @XiaoBinGan; the
  `shadow --out` help gives each language's default (#401) and the Spanish, Italian and German
  tables gained Kotlin and PowerShell (#396, #399, #400), by @hkhkkh; `enable` help and docs name
  PowerShell (#441) and the Russian and Korean READMEs explain Gradle and Kotlin detection (#442,
  #443), by @Papito084; the German, French and Portuguese READMEs got the same (#410 by @lyn1dev,
  #415 by @sniwas, #411 by @P-r-e-m-i-u-m); CONTRIBUTING lists the missing-folder tests (#391 by
  @NYeswanth08); and the site moved to rust-anything.com with every translated languages table
  brought up to date.

## 4.0.0 (2026-11-24)

- A documentation site at [rust-anything.com](https://rust-anything.com): the README, the guides
  and the real-world ports in 18 languages, with search, falling back to English for pages not
  translated yet. The README links it, and the project has a Discord.
- Python `bytes`, `bytearray` and `memoryview` hints become `Vec<u8>` in the port, and the PyO3
  bridge takes any of the three and returns `bytes` without going through JSON (#370, #381 by
  @Lewandox).
- An integer default on a Python `float` parameter (`def f(x: float = 1)`) types as `f64` instead
  of failing the bridge (#380 by @d0ukan).
- `doctor` checks `adb`, which Android recording needs (#308 by @XiaoBinGan).
- `analyze` refuses a folder that does not exist (#294 by @AyhamJo7), and `analyze --lang` for a
  language the folder does not hold is an error that names the language it found and why any
  files were left out, instead of an empty plan with exit 0 (#345, #385 by @puretechteam).
- `analyze` lists distinct lines as a risk's samples, so several hits on one line no longer hide
  the next location (#347, #382 by @puretechteam).
- `map` says when no unit was found (#296 by @AyhamJo7), refuses a file instead of printing an
  empty map (#307 by @XiaoBinGan), and refuses `--write` with `--json` instead of printing JSON
  and writing nothing (#343, #383 by @puretechteam).
- `diff`, `disable`, `report` and `shadow` on a missing folder say to run `bridge` first (#306 by
  @P-r-e-m-i-u-m, #302 by @Kiddo4993).
- `app record` joins example commands wrapped with a trailing `\`, so a curl example split over
  lines becomes its request (#328, #384 by @puretechteam).
- Parsers: C integer specifiers in any order (#374), Java wildcards named as wildcards and still
  refused (#375), Java C-style array parameters keep their dimensions (#378), phpdoc types that
  contain a space (#376), and YARD `@param` tags in either order (#377), all by @XiaoBinGan.
- HTTP recording treats only two hex digits after `%` as an escape (#298 by @XiaoBinGan).
- `bench` for Go says how much slower instead of "1.0x slower" (#299 by @XiaoBinGan), the card
  never prints a non-zero fallback rate as 0.0% or a partial one as 100.0% (#304 by
  @puretechteam), and `report` leaves out the Risk guide when no unit has a risk (#320 by
  @puretechteam).
- The workflow skill's description names Kotlin, PHP, C# and PowerShell, so agents also pick it
  for those ports (#280, #313 by @athrvd77).
- Docs: an Indonesian README (#295 by @agusmakmun); the PHP and C# unit bridge in Traditional
  Chinese (#316 by @XiaoBinGan), Portuguese (#318 by @XiaoBinGan) and Vietnamese (#379 by @tlq5l);
  the Russian README brought up to date (#319 by @onlykushalll); the PHP wrapper and the `.Rust`
  namespace explained (#317), PowerShell named in the descriptions (#312) and the plugin directory
  listing updated (#315), by @XiaoBinGan; the runtime versions `doctor` checks and `app replay`'s
  default timeout documented (#321, #322 by @puretechteam).

## 3.4.0 (2026-11-23)

- PowerShell units (#209). `analyze` reads `.psm1` modules, `.ps1` scripts and `.psd1` manifests,
  follows `Import-Module`, `using module` and dot-sourced files, and adds notes on implicit output
  and array unrolling, case-insensitive comparison and implicit conversions. `bridge` takes the
  functions a module exports (`Export-ModuleMember`, wildcards included) with `[string]`, integer,
  `[double]`, `[bool]`/`[switch]`, array, `[hashtable]` and untyped parameters, and writes
  `ps/<crate>.ps1`: each function with its original `param()` block, calling the Rust library
  through P/Invoke methods that Reflection.Emit declares at load time, so only `pwsh` 7.4+ is
  needed on Windows, macOS and Linux. Filters, `begin`/`process` blocks, pipeline input,
  `[scriptblock]`, `[ref]`, `$PSCmdlet` and other .NET types stay in PowerShell with the reason.
  `record`, `diff`, `bench` and `gate` run a generated `pwsh` driver; a result is what
  `$r = f ...` holds, and `Write-Host`, warnings and non-terminating errors are recorded as
  `output`. `enable` dot-sources the wrapper after the module's last function, so its own callers
  get Rust too; `disable` restores the file byte for byte. `doctor` checks `pwsh`, and `profile`
  says to time one `pwsh` start first. Each call costs about 0.1 ms of PowerShell-side encoding on
  an Apple M4, more with arrays and hashtables: a port pays off only for functions that do more
  work than that, and never for the start-up of a shell per command.
- Generated JSON dispatchers read `$num` markers (NaN, the infinities, -0) inside an optional list
  of floats (`Option<Vec<f64>>`), which failed with "invalid type: map, expected f64".
- `doctor` checks that PHP has the `FFI` extension loaded (`php -m`), not only PHP 8.0+
  (#240). When `FFI` is missing, the PHP line says so with how to enable or install it, and stays
  optional so the report does not fail.
- `profile` says when the time goes to child processes, not code (#208). Time under Node's
  `execSync`, `execFileSync` and `spawnSync`, and under Python's `subprocess` calls and
  `os.system`, is a new "Child processes" section and a `child_processes` field in `--json`
  (command, file, line, function, ms, share; left out when empty), with the advice to keep one
  long-lived process open, batch the commands, or call the API directly; `child_pct` gives the
  total. A hot function that waits gets its own `child_pct`, and from half its time that advice
  instead of "port it"; `go` passes over such a function, says why when `--function` names it,
  and stops when waiting is half the run or more (`--function` still names a function that does
  not itself mostly wait). A function's `child_pct` counts only waits known to happen under it:
  a helper shared by several callers, or a recursive call, does not pass its waits to every
  caller. `child_pct` for the run also counts waits that no user function owns (a library's
  timer callback); Python waits count only in the interpreter's own `subprocess` and `os`
  modules, not a dependency's file of the same name; `go` looks past the 25 rows the report
  shows; an executable named on its own (`execFileSync("/opt/My Tools/worker")`, an argv list)
  is shown whole.
- `profile` and `go` judge each hot exported function with the rules `bridge` applies. A hot
  function `bridge` keeps in the source language (one taking a callback, typed in TypeScript or
  in a sibling `.d.ts`; a Python generator) was advised as "port it", and `go` picked it and
  stopped with "every export ... must stay in TypeScript/JavaScript" (or Python). `profile` now
  says why `bridge` keeps it (a new `js_only` field in `--json`), and `go` passes over it to the
  next hot function that can cross, or says none can. `go --function f` names `bridge`'s reason for
  such a function and, when two hot functions share the name, takes the one that can cross; the
  functions `go` leaves in the original keep `bridge`'s own reason in `rust-anything.json`
  instead of "not hot in the profiled run"; `profile` reads and judges each hot file once.
- Python ports can raise the original's own exception classes: `Err("sqlparse.exceptions.SQLParseError: msg")`
  imports `sqlparse.exceptions` and raises `SQLParseError("msg")`, and any built-in exception goes by
  its name (`Err("FileNotFoundError: msg")`), not only the twelve the bindings matched before. A
  name that is no `Exception` subclass still gives `ValueError` with the whole message. `diff`
  compares exception classes, so a library with its own error class could not match before;
  `diff` now keeps the original's packages importable (put first on `sys.path` as `record` does,
  not imported), as they are in the app.
- `bridge` no longer refuses a Java, Kotlin or C# file because of an overloaded method. Every
  overload of a name is kept in the original (the bindings name each export by its name
  alone), but the duplicate-name check counted each kept overload, so the unit failed with "`pad`
  is defined twice ... rename one of them" while `analyze` listed it as ready. An overload set
  kept in the original is now one name in one file, also when `--keep` names it; other kept
  functions that share a name still fail the check. C# blocked only the later overloads and
  offered the first one for porting; it now keeps them all, as Java and Kotlin do, so `analyze`
  stops counting it as bridgeable, and it tells overloads apart by class: `Json.Parse` and
  `Xml.Parse` in one file are no longer called overloads (`bridge` still refuses them as one name
  defined twice). The same overloaded name in two files of one unit is still refused.
- Fixes from new contributors, each with a test:
  - `/*` inside a `//` comment no longer hides the Gradle dependencies after it (#222 by @XiaoBinGan).
  - The second `<script>` block of a `.vue`, `.svelte` or `.astro` file keeps its line numbers (#259 by @XiaoBinGan).
  - A huge `RUST_ANYTHING_COMMAND_TIMEOUT`, `RUST_ANYTHING_CASE_TIMEOUT` or `fuzz --time` is rejected or ignored
    instead of panicking (#254, #278 by @XiaoBinGan).
  - The coverage line never rounds 999 of 1000 lines up to 100% (#283 by @XiaoBinGan).
  - The card and `clean` print values just under a unit boundary in the next unit, such as `1.0 ms`, not
    `1000.0 µs` (#221, #253 by @XiaoBinGan).
  - Plans and `record` say "1 line", "1 unit", "1 case" and "1 export" (#282 by @XiaoBinGan, #284 by @tridev-tej).
  - A wasm port switches on through its wrapper (#205 by @XiaoBinGan), and a Python float subclass is recorded as its
    float value (#203 by @fullfud).
- Docs: every CLI flag and MCP parameter is described, and `--help` no longer calls the tool TypeScript-only
  (#228, #236, #250, #258 by @XiaoBinGan); the FAQ names the two timeout variables (#230 by @Kiranpjk); CONTRIBUTING
  lists every test file and what CI installs (#232 by @XiaoBinGan, #244 by @MikeFTV); the Spanish, Italian, French
  and German READMEs describe the PHP and C# unit bridges (#234, #235, #251, #252, #276, #277 by @XiaoBinGan).
- Thanks to @OnurrUgur (#206, #211, #212, #255), @fullfud (#203, #207), @XiaoBinGan, @tridev-tej, @ege-arhan (#224),
  @Kiranpjk and @MikeFTV: 32 contributions from 7 people since 3.3.0.

## 3.3.0 (2026-11-22)

- `go` keeps the file's other classes in the original as well as its other functions. It kept
  only functions, so porting `get_close_matches` from `difflib` also bridged `HtmlDiff`, and
  `diff` and `gate` could not pass on stubs the task said to leave alone.
- Generated Python bindings allow `clippy::too_many_arguments`: a function or method with many
  parameters made `gate` fail on rust-anything's own `lib.rs`.
- `go` takes the repository root as its first argument, as `profile` does
  (`rust-anything go . -- node bench.mjs`); `--root` still works.
- `docs/STATUS.md`'s table of which command covers which language (XiaoBinGan, #201) has rows
  for C++, Swift, Dart, PowerShell, CUDA and Assembly (whole apps only, no adapter yet), from
  his audit in #200.
- PHP units cross into Rust: `bridge` no longer refuses them. A PHP unit's top-level functions
  (namespaced or global) with scalar, string, list and assoc-array parameters get a JSON
  dispatcher crate and `php/<crate>.php`, which redeclares each function in its namespace with the
  original parameters and calls the Rust library through `ext-ffi`. `record` runs the original
  under a generated `driver.php` (after `vendor/autoload.php` when present; omitted middle
  arguments are passed by name so their defaults apply), `build` puts the library beside the
  wrapper, and `diff` and `bench` call it through the wrapper. Results are compared as PHP values
  (int and float, key order, `[]` and `{}` as PHP sees them), exceptions by class, printed output
  too. Static methods, classes, object-typed parameters or returns, references, variadics,
  callables and generators stay in PHP with the reason; `shadow` and `--batch` refuse PHP units.
  A bare `array` parameter or return now gives way to its phpdoc type (`list<float>` is
  `Vec<f64>`), and an undocumented one is a `serde_json::Value`, since it may be a list or a map.
- Real-code run: PEAR `XML_Util` 1.4.5 behind a facade of seven top-level functions, PHP 8.5.12:
  1,838/1,838 recorded cases, gate passes; through the JSON bridge every function is slower than
  PHP (0.09x geometric mean on a busy machine), as `bench` reports
  (`demo/real-world/php-xml-util-rs`, `docs/real-world-languages.md`).
- C# units can be bridged. `bridge` takes the public static methods of a unit's public classes
  and generates a JSON dispatcher over the C ABI (`Kind::Ffi`) plus `cs/<Lib>.cs`: the same
  classes and signatures (defaults kept) under `<namespace>.Rust`, each calling Rust through
  P/Invoke. Arguments may be scalars, strings, arrays, lists, string-keyed dictionaries, nullable
  values, and records or plain classes from the project (public auto-properties, no constructor
  arguments), which cross as JSON objects. Generic, `ref`/`out`/`in`, `params`, overloaded and
  async methods, and `decimal`, `char`, `ulong`, `object`, delegates, tuples and spans stay in
  C# with the reason. `record`, `diff` and `bench` build a console driver with `dotnet build`
  from the owning project's sources and package references, call each method by reflection, and
  compare results as typed JSON, exceptions by type and `Console.Out` output. An input that does
  not convert to a parameter type (a `long` for an `int`) is not recorded. `build` copies the
  library into `cs/`. `shadow` and `--batch` refuse C# units with the reason. Verified on
  Google.Protobuf 3.29.0's `CodedOutputStream.ComputeSize.cs`: 16 methods, 2,941/2,941 with two
  lone-surrogate strings listed in `known-diffs.jsonl` (a Rust `String` cannot hold them), gate
  passes. Calls cost 0.56-3.3 µs, so these 29-87 ns functions are 0.03x as fast through Rust.

## 3.2.0 (2026-11-22)

- Python `record --from-tests` keeps the calls that pass arguments by keyword where Python does
  not bind them by position: keyword-only parameters (`def f(x, *, flag)`) and an argument named
  after a skipped default (`f(x, fill='*')`). They were dropped from `test-inputs.jsonl`; they
  are now written with a `kwargs` object, which the driver already replays.
- Python `diff` and `bench` pass keyword arguments to the Rust bindings under the bindings'
  snake_case names (`scaleBy=10` as `scale_by=10`); a keyword-only parameter named in camelCase
  failed every case with `TypeError: unexpected keyword argument`. A Python mismatch shows its
  keyword arguments after the positional ones, as Ruby's does, so a `known-diffs.jsonl` entry
  for `f(x)` no longer explains `f(x, flag=True)`.
- JS/TS `record --from-tests` encodes the captured arguments with the codec `record` and `diff`
  use, instead of a copy of it that had fallen behind. A `Buffer` was captured as a `Uint8Array`
  (so `b.toString('hex')` recorded `"104,105"`), a `Float64Array` or other typed array as an
  object with numeric keys, and an object shaped like a marker (`{ $date: 'x' }`) was replayed as
  a `Date`. A call is now left out, as one that passes a function or a cycle already was, when an
  argument is a RegExp, an Error, a `URL`, an `ArrayBuffer`, a boxed primitive or an object with
  `toJSON`: those were captured as `{}`, their own fields or their `toJSON` result, which replay
  passed as a different value. A class instance is still captured as its own fields.
- `analyze` judges each export with the same functions `bridge` uses. It had its own copy of
  the rules and put units in `start_here` that `bridge` then refused ("every export ... must
  stay in TypeScript/JavaScript"). The copy missed three of them: a JS unit's sibling `.d.ts` (a
  callback declared there), a class method or constructor that takes a function, and a type
  alias such as `type Pat = RegExp`. Such a unit is now `caution` and left out. Both commands
  share `bridge::ts_functions` and `bridge::class_reason`, so in Python, Ruby, Java, Kotlin and
  Go `analyze` now also checks class methods.

## 3.1.0 (2026-11-22)

- `go` runs `bench` after gate and switches a unit on only when it is not slower as its callers
  call it; otherwise it stops at `slower` with the measured factor and where the speed is (the
  batch class to call from the loop, or the caller to port with it). A wasm port called once per
  item from a JS loop was 7.6x slower than the JIT, and `go` had switched it on. `go` also runs
  `refresh` before building a crate with batches, so a `<Name>Index` the agent writes is used.
  Both found by running `go --agent claude` on a TS fixture (`tests/fixtures/go-search-ts`).
- wasm builds write `pkg/package.json` with `"type": "commonjs"`: inside a `"type": "module"`
  project Node loaded wasm-bindgen's CommonJS output as an ES module and `diff` failed.
- `profile` batches over the innermost loop: with nested loops it named the outer loop's
  variable (`--batch score:query`) instead of the one that runs over the items.
- README: "Only the slow part" in all 20 translations. Website: "Only the slow part" and
  "Switch on and off" pages (English).

- `go`: the hot-path loop in one command. `rust-anything go --agent claude -- python3 app.py`
  profiles the workload, picks the outermost exported function of a portable unit that takes
  arguments (bridged with `--batch` where `profile` advises it; `--function` picks another),
  bridges it alone into `rust/<function>-rs`, records generated cases plus the calls the workload
  makes, writes `TASK.md`, and hands the port to the agent (`claude`, `codex`, or any command
  reading the prompt on stdin, with up to `--rounds` turns that get the failing cases back). It
  then builds, diffs and gates, and switches the unit on with `enable` only when every case
  matches and gate passes. Without `--agent` it stops at the task; run it again after porting and
  it keeps every finished step. Also an MCP tool.
- `enable` and `disable`: switch a TS/JS or Python unit on in place, for every caller, with one
  marked block in its original files that rebinds the ported functions to the Rust module (no
  import changes, no absolute paths); `disable` removes exactly those lines. The unit's
  `RUST_ANYTHING_UNIT_<NAME>=original` or `RUST_ANYTHING_MODE=old` keeps the original for a run,
  and `record`, `diff`, `bench` and `fuzz` set it for the original's side. `enable` refuses
  while recorded cases differ (`--force`). Every switch is listed in one place,
  [Switch a port on and off](docs/how-it-works.md#switch-a-port-on-and-off). MCP tools too.
- `report`: `report/REPORT.md` and `report/card.svg` for a pull request or README, in the v3
  card's light style: the unit, recorded cases matched (`diff`), speedup with the call included
  (`bench`), fallback rate, on or off, and the machine with its core count. MCP tool too.
- One build cache: ports and bridges build into one cargo target folder (`CARGO_TARGET_DIR`, else
  `rust-anything/target` in the user's cache folder), so dependencies compile once. A package
  name belongs to one crate folder there, so a second crate with the same name builds in its own
  `target/` and cannot replace the first one's library; libraries are copied to each crate's
  `target/release`. `clean` removes the folder (`--dry-run`, `--force` for a `CARGO_TARGET_DIR`).
  Front doors keep their own `target/`.
- `profile` reports `portable`, `params` and the `batch` spec per hot function, and its workload's
  stdout goes to stderr, so the MCP `profile` tool no longer writes the workload's output into
  the protocol stream.

- Python: `__all__` was ignored when an earlier top-level statement held no assignment (a
  docstring or an import, as in most modules), so `analyze`, `bridge`, library ports and
  `profile` took every name without a leading underscore: names left out of `__all__` were
  bridged and underscore names listed in it were dropped. `__all__` is now read after such
  statements, with or without a type, follows `__all__ += [...]` and a later `__all__ = [...]`,
  and reads prefixed strings (`r'name'`). An `__all__` built or changed any other way
  (`+ other.__all__`, `sorted(...)`, `__all__.extend(...)`, inside an `if` or `try` block) falls
  back to the names without a leading underscore rather than a partial list. Functions and
  classes a module's `__all__` leaves out are no longer bridged.
- When `build` of a Ruby bridge fails with rb-sys's `Stable API is needed but could not find a
  candidate` and the Ruby rb-sys used (`RUBY`, else `ruby`) is older than `doctor`'s 3.0, the
  error now ends with that Ruby's version and `doctor`'s install command (macOS's system Ruby is
  2.6), instead of only rb-sys's hint to enable `stable-api-compiled-fallback`.

## 3.0.0 (2026-11-21)

- `package` makes a whole Node CLI installable: pointed at a front door (`rust/app`), it lays out
  a main npm package whose command runs the front door binary from `<name>-<platform>` (macOS
  arm64 and x64, Linux x64 and arm64 with glibc, Windows x64), with the original as a dependency
  pinned to the exact recorded version for every run the port hands back. A Node launcher runs
  the binary (or the original where no binary fits, or with `RUST_ANYTHING_MODE=old`); outside
  Windows a postinstall script puts the binary in its place, so a covered run starts no Node.
  `--bin` names the commands, `--original <package>@<version>` the original when it is not in
  the repository's `node_modules`. A generated workflow builds and packs every platform.
- Front doors find the original next to themselves when installed: inside `node_modules`, the
  generated `original.rs` resolves `node_modules/...` from `RUST_ANYTHING_ORIGINAL_BASE` (set by
  the launcher) or the nearest folder above the binary, before the repository it was built in.
  The Prettier and ESLint front doors carry the change.
- The ESLint port on npm as `eslint-rs` (ESLint 10.15.0; macOS and Linux binaries, Windows runs
  ESLint itself), the Prettier port packaged the same way as `prettier-rs` but not published, and
  `.github/workflows/ports-npm.yml`, which builds both front doors on every platform, packs them
  with `package`, and checks the Linux install against the originals.
  Installed from their tarballs on macOS arm64: `prettier-rs` replays 17,224/17,224 recorded runs
  at 7.9% fallback, through the binary and through the launcher; `eslint-rs` replays
  10,492/10,585, the 93 others printing ESLint's own install path, as ESLint 10.15.0 run from the
  same install does (`docs/packaging.md`). The Prettier front door's `Cargo.lock` now lists
  `ignore`, which `--locked` builds needed.

- Prettier demo: about 75 differences from Prettier 3.6.2 outside the recorded corpus, found by
  differential fuzzing, mutation testing and a Doc-level comparison, are fixed or handed to
  Prettier. 355 new cases and the checks are in `demo/real-world/prettier-rs/equiv`. The
  recorded corpus now runs 15,861 of 17,224 cases fully in Rust (7.9% fallback, was 15,837 and
  8.1%).
- `package` (CLI and MCP tool) lays out a gated port as packages end users install, and writes
  files only: PyO3 ports become a maturin project (an abi3 wheel importable under the original's
  module name), napi ports the napi-rs per-platform npm layout (a main package with
  `optionalDependencies` on `<name>-<platform>`, a loader that falls back to the original JS where
  no binary fits), wasm ports one npm package. PyO3 and napi ports also get a GitHub Actions
  workflow building macOS (arm64, x86_64), Linux (x86_64, aarch64) and Windows. Installed from
  their packages, `demo/fuzzy-napi` and `demo/fuzzy-rs` pass 2000/2000 recorded cases and
  `demo/fixtures/python-textkit-rs` 1144/1144 (`docs/packaging.md`).
- `fuzz`: a live differential test. It generates fresh inputs from a new seed (the record
  generator, skipping inputs already recorded), runs them through the original and the built
  port, and reports the mismatches the recorded cases missed. `--cases N` per function, `--seed S`
  (random by default and printed, so a run replays), `--time SECS` for rounds until the budget is
  spent, and `--keep` (`--keep=all`) appends the mismatching (all) fresh cases to `cases.jsonl` so
  the next `diff` and `gate` check them. Works wherever `record` and `diff` do on this machine:
  TS/JS, Python, C, Go, Ruby, Java and Kotlin. Also an MCP tool.
- `record --pin-clock` (TS/JS with the napi bridge, Python): units that read the clock or random
  numbers can be recorded. While pinned (`pinned.json`, until `--no-pin-clock`), record, diff,
  fuzz and gate run with `RUST_ANYTHING_NOW_MS`, `RUST_ANYTHING_SEED` and `TZ=UTC`; `Date.now()`,
  `new Date()`, `Date()` and `Math.random` (`time.time`, `datetime.now`, `date.today` and the
  `random` module) return that instant and a mulberry32 sequence restarted at every case. The port
  reads the same values from `src/port/pinned.rs` (`now_ms()`, `random()`, `getrandbits()`,
  `randbelow()`), written by the first `--pin-clock`. Recordings without the flag are unchanged.
- `analyze`: reads of the clock and random numbers (`Date.now()`, `new Date()`, `Math.random`,
  `performance.now()`; `time.time`, `datetime.now`, `random.*`, `uuid4`) are a new low-severity
  risk, "Clock and random numbers", whose advice points to `record --pin-clock`.
- C bridge: structs of scalar fields cross by value, as parameters and returns (ffmpeg's
  `AVRational av_mul_q(AVRational b, AVRational c)`). The struct is read from the project's
  headers (`typedef struct ... Name;` or `struct tag`), is a `#[repr(C)]` struct at the crate root
  (`crate::AVRational`), and is an object of its fields in cases and recorded results. Structs
  with a pointer, array, bit-field or nested struct field stay in C.
- C bridge: a `char **` / `const char **` out-parameter that the function points into a string
  argument (`av_strstart(str, pfx, &ptr)`, `strtol`'s `endptr`) is recorded as the offset into
  the first `char` string or buffer parameter, and is `Option<&mut Option<usize>>` in the port.
  In cases, give its starting offset, `true` for a `NULL` start, or `null` for a `NULL` pointer.
  A `**` the function advances as its own string (`av_get_token(const char **buf, ...)`) still
  stays in C, as does anything returning memory it allocates.
- `record` (C): generated cases give struct parameters objects of their fields (zeros, ones,
  minus ones, each field's minimum and maximum, each field zero with the others one, and small
  random values) instead of values that were recorded as `TypeError`s, and `char **`
  parameters `true`, `null` or offset 0.
- `diff` (C): a case where the original returned or wrote a pointer outside its arguments (read
  past a string, say) is undefined behavior like a crash, and any answer of the port passes it.
- Case study: the ffmpeg ports add `av_mul_q`, `av_div_q`, `av_add_q`, `av_sub_q`, `av_d2q`,
  `av_nearer_q`, `av_q2intfloat`, `av_rescale_q`, `av_rescale_q_rnd`, `av_strstart` and
  `av_stristart`; 226,447/226,447 recorded calls, gate passes on all 10 crates.
- `diff` (C): a case the original recorded as an error (an argument ctypes could not convert)
  passes when the port gives the same error, instead of always failing.
- Contributors: `mise.toml` pins the toolchains CI uses, and `hk.pkl` runs the secrets check,
  `cargo fmt` and `cargo clippy` before each commit (suggested by u/touristtam on Reddit).
- `setup --no-backup`: changes settings files in place without saving a
  `<file>.rust-anything.bak` copy first.
- `record`/`diff` (TS/JS): plain data shaped like a marker (`{"$date": ...}`, Mongo extended JSON)
  is kept as data instead of being read as a Date, undefined or bigint, and a marker key next to
  other keys is ordinary data. Write such an object in `--inputs` as `{"$object": {...}}` (#121).
- `record`/`diff` (TS/JS): typed arrays other than `Uint8Array` (`Float64Array`, `Int32Array`,
  `Uint8ClampedArray`, ...) are recorded with their kind, so a port returning the wrong one
  fails (#122).
- `app replay`: lines compared in any order are paired by value within `--float-tolerance`, not
  by sorted position (#123, #125 by @sameermankotia).
- `analyze` (C): a unit depends on what its own header includes, so `a.c` that reaches `b()`
  through `a.h` waits for `b.c` (#126).
- README: right under the tagline, it now says the tool doesn't translate code (your agent or you
  write the Rust) and what PHP and C# support today. From a new-user QA review of the README by
  [Mashhood Ali](https://github.com/MASHHOOD2019), an independent AI evaluation and QA specialist
  who tests products and docs the way a new user would ([LinkedIn](https://www.linkedin.com/in/mashhood-ali-514862296)) (#118).
- `app record --fresh` works for CLIs: every case starts from a snapshot of the given files or
  folders (kept in `app-cases/files/`; a path ending in `/` gives its contents, so a whole project
  can be the working directory), written files are what differs from it, and files a case
  removes are recorded in `<case>.removed` and compared on replay. An `--inputs` case can also
  `"remove"` files of the snapshot before it runs. `--fresh` alone no longer implies `--http`.
- Front doors built by rust-anything find the original when `rust/` is a symlink to a folder
  outside the repository (`RUST_ANYTHING_REPO` is set at build time).
- Case study: ESLint 10.15.0 behind a Rust front door, 90 rules (`eslint:recommended` and 26
  more), 10,585/10,585 recorded CLI runs at 3.6% fallback, gate passes on `eslint-stdin-rs`
  (`demo/real-world/eslint-rs`, `docs/real-world.md#eslint`).
- `bridge` and `analyze` read CommonJS exports: `module.exports = { a, b: c }`,
  `module.exports.a = a` and `exports.a = a`, for functions declared in the file. ESLint (and
  most Node code before ESM) exports this way, so its units showed no functions to bridge.
- The `timestamps` normalizer also matches JavaScript's `Date#toString()`
  (`Sat Nov 21 2026 01:30:12 GMT+0300 (...)`). ESLint's HTML report stamps its time that way:
  the two recording runs differed only in the seconds, so only the seconds became a wildcard and a
  replay ten minutes later failed.
- Case study: ffmpeg's libavutil (n7.1), 10 units, 135,773/135,773 recorded calls, gate passes
  on each (`demo/real-world/ffmpeg-rs`, `docs/real-world.md#ffmpeg`).
- C bridge: pointer parameters cross it with the original signatures. A `const` byte pointer
  followed by an integer is a buffer of that length (`&[u8]` in the port); a byte pointer the
  function writes, paired with the next integer or one named like a size, is `&mut [u8]` and its
  contents after the call are recorded; a mutable pointer to one scalar is `Option<&mut T>` and
  its value after the call is recorded; a returned pointer into a parameter is the offset into
  it (`Option<usize>`). In cases, buffers are hex text (or a size, for that many zero bytes) and
  length parameters are left out. Typedefs of scalars and enums resolve through the project's
  headers. Found porting ffmpeg's libavutil, whose APIs all take pointers.
- C bridge: a port can take a `const char *` as `Option<crate::CChars<'_>>` instead of
  `Option<&[u8]>`: the bytes are read as the port goes, without the `strlen` the slice needs
  first. Change the parameter's type in `port.rs` and run `refresh`; `lib.rs` defines `CChars`
  (an iterator over the bytes, with `peek` and `to_bytes`) only when a port uses it. On short
  words the two `strlen` calls cost more than ffmpeg's whole `av_strcasecmp`; with `CChars` the
  port went from 0.23x to 0.68x of C's speed, `av_stristr` from 0.44x to 1.4x.
- `record` and `bench` for C: a folder holding a header named like a C standard one (ffmpeg's
  `libavutil/time.h`) is passed with `-iquote`, so `<time.h>` stays the system's; flags in
  `CFLAGS` are added (`-DHAVE_AV_CONFIG_H`, `-I<tree>`).
- `build`, `diff`, `bench`, `shadow`, `gate`, `refresh` and `record` take the crate path as
  given and make it absolute: `rust-anything build .` on a C crate failed, since the compiler and
  cargo run in a scratch folder.
- `analyze` for C: files under a CPU-architecture folder (`x86/`, `aarch64/`, `arm/`, ...) stay
  in C; `goto` no longer marks a unit blocked as dynamic code; `#include "libavutil/x.h"`
  resolves when the folder analyzed is `libavutil` itself, so its `tests/*.c` count as tests; a
  unit with no bridgeable export gets a note about the C bridge instead of the JS one.
- `gate` for C: the size check counts the project functions the ported functions call, since the
  port does their work too.
- C `char` parameters are `c_char` in generated code (signed or unsigned as the platform's C
  `char`), not `c_schar`.
- `doctor` on Windows: a missing `cargo` gets the C++ Build Tools, then
  `winget install Rustlang.Rustup`, instead of the `sh.rustup.rs` script, which needs a Unix shell.
  The Build Tools come first because winget runs `rustup-init -y`, which skips them with a warning.
  The Build Tools fix also adds the C++ workload (and the ARM64 tools on Windows on ARM): without
  `--add`, winget installs them with no compiler, linker or Windows SDK. Since winget adds nothing
  to Build Tools already installed, the fix also says how to add the workload by hand. A fix of
  several commands prints one per line.
- `analyze --whole-app` lists a library once when the code imports it under one name and the
  manifest declares it under another (`yaml` and `PyYAML`, `androidx.room` and
  `androidx.room:room-runtime`, `active_support` and `activesupport`): names the crate table
  resolves to the same row are one dependency. The JSON keeps `name` (the manifest's spelling
  when declared) and adds `names`, every spelling; `declared` is true if any of them is, and
  `used_by` counts each unit once. The Markdown plan shows the other spellings next to the name.
  Names the table does not know stay separate.
- `analyze --whole-app` reads declared dependencies from each language plugin's own manifest
  parser instead of a second copy, so `analyze` and `--whole-app` agree, and every manifest in
  the workspace counts, not only the root's (a dependency on another package of the workspace is
  not an external one). The plugin parsers gained what only the copy had: Gradle and Maven
  comments are skipped, a Gradle declaration on the same line as `dependencies {` is read,
  requirements files follow `-r` includes and skip VCS, URL and path lines, Poetry dependencies
  count, a gem inside a `group` block is a development dependency, and `add_dependency` is read
  on any receiver. TS/JS counts `peerDependencies` and `optionalDependencies` as declared too.

## 2.1.0 (2026-11-20)

- `app record --float-file PATH=f32le|f64le`: a written file of raw floats is compared value by
  value within `--float-tolerance` instead of byte for byte, and a mismatch reports the count,
  the first index and the largest differences (#114).
- Kotlin/JVM: detection, `analyze`, bridge, record and diff for Gradle modules of top-level
  functions and `@JvmStatic` methods, through the Java bridge (#117).
- Android: `build --profile android` builds the JNI library for the NDK ABIs into `jniLibs/`,
  and `record`/`diff --profile android` run the unit under ART on a connected device; `doctor`
  checks the NDK and Rust targets (#117).
- Java: every overload of an exported name is blocked, since the reflection driver keys methods
  by name; the generated Java runtime uses plain reflection, which ART also runs (#117).
- `record`, `diff` and `bench`: the original and the port run in a temporary working folder,
  removed afterwards, so a function that writes to a path it is given no longer leaves files
  named after the generated inputs in the caller's folder (#115).
- `analyze`: Python functions with numpy array parameters (`np.ndarray`, `NDArray`) carry a new
  risk, since `record` cannot generate arrays for them yet: record the app instead (#113).
- README: the 2.0 headline says how many Prettier runs are fully in Rust (15,837 of 17,224)
  rather than only that none mismatch (#118).
- `lift`: the lifted app or library gets `rust-version` and a release profile with `lto = true`
  and `codegen-units = 1`, unless it sets them already.
- `bridge`: generated crates set `rust-version` and warn on `unused_lifetimes` and
  `trivial_numeric_casts` (so `gate` fails on them).
- `gate`: two checks that never fail it: `lockfile` warns when `Cargo.lock` is missing or out of
  date, and `advisories` runs `cargo deny check advisories` or `cargo audit` when one is
  installed.
- Skill: typed errors inside `port.rs`, Rust's `is_alphanumeric` against the original's letter
  rule, `#[expect(lint, reason = "...")]` over `#[allow]`, and `cargo test` with no tests proving
  nothing.
- `bridge --allocator rusty_alloc`: an opt-in pure-Rust global allocator for batch crates (wasm
  too), kept by `refresh` and taken by the MCP `bridge` tool. mimalloc stays the default (#112).

## 2.0.2 (2026-11-19)

- The npm package carries `mcpName`, and `server.json` describes the MCP server, so it can be
  listed in the official MCP Registry as `io.github.erkamyaman/rust-anything`.

## 2.0.1 (2026-11-19)

- `record`: string traps now include characters that languages classify differently (`²`, `Ⅰ`,
  `½`, `①`, vowel signs such as `ि`, Arabic-Indic digits and letters beyond U+FFFF), so a port
  that tests letters and digits with Rust's `char::is_alphanumeric` instead of the original's
  rule fails `diff`.
- Java demo: `slugify` in `demo/fixtures/java-text-rs` now matches Java's
  `Character.isLetterOrDigit` per UTF-16 unit, and the Java benchmark claims (3x batch) are
  withdrawn: the batch ran on 4 cores against a serial loop. Reported in #110.
- `analyze`: Python packages in a workspace match their dependencies by the normalized name, so
  a package named `core_lib` is found when another one depends on `core_lib>=1` and the port
  order puts it first.
- `setup`: a symlinked settings file (a dotfiles repo) is written at its target instead of being
  replaced by a plain file, and an existing file keeps its permissions (a `600` file stays `600`).
- `setup`: a Codex entry written as an inline table (`rust-anything = { ..., env = { ... } }`)
  keeps its other keys on reinstall, as a `[mcp_servers.rust-anything]` table already did.

## 2.0.0 (2026-11-19)

Highlights:

- **Prettier, ported to Rust.** Prettier 3.6.2 behind a Rust front door: 15,837 of 17,224
  recorded runs format fully in Rust across JavaScript, TypeScript, JSX, JSON, GraphQL, YAML,
  Markdown, CSS, SCSS and Less, and all 17,224 match Prettier byte for byte. 19x faster on JS/TS
  from the command line (Apple M4). See `docs/real-world.md#prettier`.
- **json-server, ported to a pure Rust binary**: 514/514 HTTP cases, 3.5 MB of memory at idle
  instead of 74 MB.
- **Side effects are compared**: outgoing HTTP calls, email and the database's state, per case.
- **`map`**: the whole port at a glance, and every piece whose original changed upstream.
- **Plain JavaScript gets types** from JSDoc and `.d.ts`; **OpenCode** joins the agents `setup`
  registers.

Changed: `diff` exits non-zero when the recording is stale (it printed a warning and exited 0).

- `app record --effects http,smtp`, `--effect-env VAR=KIND`, `--db-reset` and `--db-dump`: record
  what leaves the process per case (outgoing HTTP calls, email, the database's state) and compare
  it on replay, for CLIs and servers. A rewrite that sends an email twice or writes different rows
  now fails the case even when its output matches.
- `map` (CLI and MCP tool): every unit of the original with the crate that ports it, functions
  in Rust and kept in the original, cases and last `diff`, the whole app's glue, units not ported
  yet, and every port whose original changed since it was recorded. `--write` saves
  `rust/PORT_MAP.md`; `--check` exits 1 while a recording is stale.
- Docs: what a real port takes, keeping the original's shape, keeping up with the original, and
  checking a rewrite in any language with `app record` and `app replay --build`.
- `bench` and `record` pass `--experimental-transform-types` only when Node accepts it: Node 26
  removed the option and refused to start (#99).
- `profile` keeps a Python workload script given as a relative path (`python3 bench.py`); its
  functions were dropped and the table came out empty (#101).
- `diff` exits non-zero when the recording is stale, as `gate` already refuses it, and the
  warning says "Stale recording" (#102).
- `analyze` says what a wasm-bindgen bridge needs installed, and that `--kind napi` runs with
  what Node already has (#102).
- `bridge` reads types for plain JavaScript: from a sibling `.d.ts`/`.d.mts`/`.d.cts`, then
  JSDoc `@param {T} name` (with `[name]`, defaults and `opts.field` shapes) and `@returns {T}`.
  Before, only parameters with a literal default were typed and the rest became `unknown`, so
  `record` generated mostly wrong-typed inputs (#100).
- TypeScript analysis and recording resolve path aliases against the inherited `baseUrl`,
  including when the two options come from separate tsconfig files.
- `setup` also registers OpenCode: the MCP server in `~/.config/opencode/opencode.json`
  (`mcp.servers`, the whole argv in `command`) and the workflow skill in
  `~/.config/opencode/skills/`, and `doctor` reads that entry back.
- A port of [Prettier](https://github.com/prettier/prettier) 3.6.2 behind a Rust front door in
  `demo/real-world/prettier-rs` and `docs/real-world.md`: 17,224/17,224 recorded CLI cases match,
  15,837 of them (91.9%) formatted fully in Rust and byte-identical to Prettier, the rest handed to
  Prettier per file (8.1% fallback). JavaScript, TypeScript (849 of 929 cases), JSX, JSON in
  all four flavours (995 of the 1,019 JSON cases Prettier formats), GraphQL (all 787 cases
  Prettier formats, with graphql-js's parser ported) YAML (3,309 of 3,310, with yaml's CST
  parser and yaml-unist-parser ported), Markdown (6,305 of 6,350, with remark-parse ported and
  code blocks formatted by the other ports) and CSS, SCSS and Less (all 2,004 cases Prettier
  formats, with postcss, postcss-scss, postcss-less and the value, selector and media query
  parsers ported) run in Rust, as do all 59 real repository
  files. Ported: the doc printer (`gate` 2,848/2,848), the printer on oxc's ESTree with
  comments, classes, decorators, template literals, ternaries and `prettier-ignore` (`gate`
  15,753/15,753 on `format()`), and the CLI's config resolution (`.prettierrc` JSON,
  `package.json`, overrides, `.editorconfig`, ignore files). On an M4 against Node 24: 19x faster
  per file from the command line, 7x on the median JavaScript/TypeScript file in process (18x on
  the median GraphQL file, 13x on the median YAML file, 25x on the median Markdown file, 9.5x on
  the median CSS/SCSS/Less file); per file from the command line 8.5x for style files and 15.6x
  for Markdown. No file was slower in a first run; one per language was in a second run (a one-off
  stall, 135 µs against 8.5 µs in the first).
- Glue can decline one case after reading stdin: `original::run_with_stdin(args, input)` hands the
  case and its bytes to the original; `lift` refuses while any glue still calls `original::`.
- `bridge` reads `export { a, b as c }` lists.
- Fix: with `CARGO_TARGET_DIR` set, every front door built into the same directory and replays
  ran another app's binary; the front door builds into its own `rust/app/target`.
- Fix: case and input lines nested deeper than 128 levels failed to parse; the harness and
  generated `from_json` read them without a depth limit.
- Fix: `--inputs` case names were cut to 24 characters and collided; given names keep 96.

- A port of [json-server](https://github.com/typicode/json-server) to a pure Rust binary in
  `demo/real-world/json-server-rs` and `docs/real-world.md`: 514/514 recorded HTTP cases match
  with no fallback, and six units pass `gate`. Against Node 24 on an M4: first response in 8 to
  10 ms instead of about 80, 3.5 MB of memory idle instead of 74, 2 to 4x the requests per second
  on one connection and 5 to 13x on 50; 1.1 to 1.2x on writes and on the whole list of a 10,000-post
  database.
- `app record --http --fresh <path>` copies a file or folder the server writes (its database)
  into a fresh working directory for every start, from a snapshot kept with the cases;
  `--arg <arg>` passes the original's arguments to every build too; `--pace <ms>` waits after each
  response for work a server does after answering. json-server needed all three: it rewrites its
  database on every write, and its file watcher can undo a write that follows quickly.
- `bridge --as-json T` bridges a generic the app only fills with data (`paginate<T>(items: T[])`)
  as JSON values instead of keeping it in TS.
- Fix: the HTTP front door's routes with parameters answered axum's own 400 when an escape was not
  UTF-8 (`/posts/%E0`); parameters are now read from the URI and the unit gets the request.
- Fix: `app lift` left out the crates the wired units depend on by path (shared code, other
  units), so the staged build failed; they are staged, stripped of their bindings and synced back
  with the units.
- Fix: a lifted unit that keeps generated modules (`js.rs`) got a `lib.rs` that failed
  `cargo fmt --check`: modules are now listed in the order rustfmt keeps.

- A port of [Fuse.js](https://github.com/krisk/Fuse) string-list search in
  `demo/real-world/fuse-rs` and `docs/real-world.md`: 3,414/3,414 recorded cases match, results
  byte-identical to Fuse, 32x faster on 50,000 strings (5.3x on one core).
- `gate`'s size check counts the files a TS unit imports from the repository, not only the unit.
- Batch crates with string items but other results no longer get an unused helper (clippy failed).
- TypeScript analysis and recording keep inherited path aliases when a later tsconfig base
  omits `compilerOptions.paths`.

- Faster single calls on napi: every sync function crosses the binary wire with no napi
  arguments (each argument cost about 55 ns), behind one JS wrapper instead of two;
  `serde_json::Value` parameters cross as tagged values of any shape, and errors come back in the
  wire buffer. vercel/ms per call: 0.39x to 0.56x of JS over its recorded cases, and `parse` on a
  normal string at JS speed (79 ns).

- `record --from-tests -- <test command>` runs the original's own test suite and records every call
  it makes to the unit's functions: TS/JS with any runner (Jest, Vitest, `node --test`), Python
  with pytest or unittest. On vercel/ms it captured 340 calls from 167 Jest tests, more than
  twice the 149 inputs copied by hand.
- `record` saves a fingerprint of the original unit (`recorded-from.json`); `diff` warns and `gate`
  fails when the original changed since.
- `known-diffs.jsonl` lists cases where the port is meant to differ, each with the result it gives
  and a reason. `diff` and `gate` count them as passing only with exactly that result, and `gate`
  fails on an entry that no longer differs.
- `app replay`'s report no longer carries temporary paths, so CI can commit it and fail on
  `git diff --exit-code`.
- A port of [vercel/ms](https://github.com/vercel/ms), end to end, in `demo/real-world/vercel-ms-rs`
  and `docs/real-world.md`: 975/975 recorded cases match; slower per call, 19x faster as a batch
  for `parse` (3.6x with the handover), 1.3x for `format(x, { long: true })`.
- Fix: a napi batch method with a structured parameter did not compile; crates with batch classes
  now enable napi's `serde-json` feature.
- napi batches pass strings joined: items cross as one string plus UTF-16 end offsets and string
  results come back the same way, about 2x faster each way. The JS API is unchanged.
- napi batches of numbers take their items as one `Float64Array`: `new Batch(numbers)` costs
  0.1 ms instead of 3.3 ms for 100,000 values.
- The generated `js.rs` gains `number_to_string` (JS `String(n)`, digits from serde_json's fast
  printer) and `math_round` (JS `Math.round`), tested against Node.
- `analyze` flags `Math.round`, `toFixed`, and `/i` regexes without `u` where Rust's case folding
  matches more (`ſ`, the Kelvin sign).

## 1.5.1 (2026-11-18)

- Fix ([#80](https://github.com/erkamyaman/rust-anything/issues/80)): `record` could stall for good
  when a generated input made the original run away (a trap like `2147483647` in `[w] * n`
  allocates gigabytes, a loop that never ends). Every recorder (TS/JS, Python, C, Go, Ruby, Java)
  now runs under a watchdog: a case that runs past the limit (10 seconds, or
  `RUST_ANYTHING_CASE_TIMEOUT`) stops the original, the cases recorded so far are kept, and that
  input is left out and listed. Drivers now write each case as it finishes instead of at the end.

## 1.5.0 (2026-11-18)

Twenty-two contributions from the community, Windows in CI, and a README in 16 languages.

- From @rudycelekli, 20 fixes, each with a test that fails without it:
  - TypeScript: keyword-shaped export names survive `refresh` (#57); objects with more than 255
    properties fall back to dynamic wire keys (#58); recording works from a folder with `#` in its
    path (#59); large strings with lone surrogates replay without overflowing Node's argument
    limit (#60); exact `tsconfig` path aliases beat wildcards (#61); `.mjs`/`.cjs` specifiers
    find the authored `.mts`/`.cts` (#62); `bench` awaits async exports (#63).
  - C, Go, C#, PHP, Java: a C function or parameter named `c_str` no longer breaks the bridge
    (#64); private nested Go struct fields stay out of the wire schema (#65); `go.mod` directives
    with tabs and comments are read (#66); a root C# test project's sources count as tests (#67);
    grouped PHP `use` imports keep their namespace (#68); Go packages that use cgo can be recorded
    (#69); a comment in a Java package declaration no longer hides the class (#70).
  - Whole-app and HTTP: HTTP cases sharing a name are all kept (#71); curl `--data @file` bodies
    match curl (#72); an invalid `Content-Length` is an error, not a body read until close (#73);
    Node profiles work from a folder with spaces (#74) and decode `file://` URLs (#75); re-recording
    keeps a configured port (#76).
- From @atontini: native Windows CI for the bridge tests, with Windows fixes for absolute paths in
  run commands, Python's UTF-8 output and line endings (#78). On top of it: the Go driver finds the
  Rust DLL on Windows, and the Ruby test skips there, since RubyInstaller's MinGW Ruby cannot load
  an MSVC extension.
- From @lyufeng000: README translations in Chinese (simplified and traditional), German, French,
  Japanese, Korean and Russian (#36). Turkish, Azerbaijani, Uzbek, Kazakh, Kyrgyz, Turkmen, Tatar,
  Uyghur and Ukrainian followed, every translation is marked community-maintained, and each README
  links all 16 languages.
- CI runs on Node 24. `profile` on Node needs it, since Node 22 refuses `--cpu-prof` in
  `NODE_OPTIONS`.
- A social preview image (`docs/assets/social-preview.png`) without a version or star count, so
  link cards never go stale.

## 1.4.0 (2026-11-18)

Fixes from the first round of outside issues and pull requests, and the project's community files.

- Fix ([#31](https://github.com/erkamyaman/rust-anything/issues/31)): `bridge` takes a single source
  file inside a unit (`rust-anything bridge . pkg/core.py`) and ports just that file, so a helper
  script next to the real module can stay in the source language.
- Fix ([#32](https://github.com/erkamyaman/rust-anything/issues/32)): a name defined twice in one
  unit (a wrapper in `__init__.py` and the function it wraps in a submodule, say) made one
  definition silently replace the other. `bridge` now refuses with both locations and how to
  bridge the module you want on its own.
- Fix ([#33](https://github.com/erkamyaman/rust-anything/issues/33)): Python command-line entry
  points are kept in Python instead of offered as ports: functions the
  `if __name__ == "__main__":` block (or a package's `__main__.py`) calls, and functions returning
  an `argparse.ArgumentParser`. `app front-door` takes the command line over in a whole-app port.
- Fix ([#34](https://github.com/erkamyaman/rust-anything/issues/34)): a Python parameter typed with a
  dataclass now gets real instances in recorded cases, built from the dataclass's fields (defaulted
  fields sometimes left out), instead of only `None` and shapes that can never fit.
- From @elfenomenokim: optional dataclass parameters bind as `Option` so the generated crate
  compiles (#28), and Python's whole standard library resolves as stdlib, with unknown dependencies
  named by the language's registry instead of "npm" (#29).
- Community files: CONTRIBUTING, SECURITY (with private vulnerability reporting), CODE_OF_CONDUCT
  and ACCESSIBILITY, issue and pull request templates. The README names its author up top, opens
  with the speed demo, and lists its sponsors.

## 1.3.0 (2026-11-18)

Two new languages in preview (PHP and C#), a stricter `diff`, a sturdier napi bridge, and 18
fixes from the first outside contributor.

- 18 pull requests from @rudycelekli, merged with their history: HTTP responses that end early
  or carry informational headers (#7, #8), empty recordings no longer pass (#9), float tolerance
  keeps key order (#10), own `__proto__` keys survive (#11), curl examples with a query right
  after the host (#12), TS projects without a manifest and `.mts`/`.cts` (#13), `setup` keeps
  custom MCP server options (#14), replay deadlines cover child processes (#15), colliding case
  names (#16), binary CLI cases keep both streams exactly (#17), the Rust 1.97 minimum is declared
  and checked (#18), a missing port source fails `gate` (#19), Ruby recordings are published
  only when complete (#20), JNI names with `$` and non-ASCII (#21), Go internal packages and
  versioned modules (#22), generated Go names that clash with parameters (#23), and invalid Java
  replay arguments (#24).
- PHP, in preview: detected from `composer.json` or `.php` files. `analyze` reads Composer
  packages and PSR-4 roots, resolves `require` and `use`, finds functions and public static
  methods with native and phpdoc types, flags PHP's risks (loose `==`, `/` returning floats, byte
  strings against `mb_*`, PCRE features the `regex` crate lacks, `eval`, magic methods) and maps
  common Composer packages to crates. `analyze --whole-app` finds CLIs (`$argv`, composer `bin`)
  and servers (front controllers, `$_SERVER`). `app record`, `app replay` and `app front-door`
  work on PHP CLIs and on servers run with `php -S`. Bridging PHP units into Rust (PHP FFI over a
  C ABI) comes next; `bridge` says so for now.
- Fix ([#3](https://github.com/erkamyaman/rust-anything/issues/3)): `.gitignore` files in folders
  above the repository (a `*` in a home-folder `.gitignore`, for one) no longer hide every source
  file. Ignore files are read the way git reads them: from the repository root down, and not from
  parent folders at all outside a repository. When ignore rules still exclude every source file,
  `analyze` says so instead of returning an empty plan.
- PHP's built-in server echoes a `Host` header with the port it ran on; HTTP recordings now skip
  it, like `Date`.
- Fix ([#4](https://github.com/erkamyaman/rust-anything/issues/4)): Python units bridged on macOS
  failed to link (`Undefined symbols ... _Py_IsInitialized`). The PyO3 crate now gets the same
  `build.rs` the Ruby bridge has, which lets the interpreter provide Python's symbols at load time.
  `rust-anything refresh <crate>` adds it to crates bridged before this fix.
- Fix: a Rust panic no longer passes `diff` where the original also threw. An unfinished port
  (`unimplemented!`) used to "match" every recorded error case in TS, Ruby (a panic arrives as
  `RuntimeError`), Java (`RuntimeException`) and Go (a panic where the original panicked). Python's
  driver crashed on the first `PanicException` instead of reporting it, and `shadow` let it reach
  the app. Each language now has a test that an unported crate passes no case.
- The napi bridge, after a study of napi-rs:
  - errors keep their JS class: a port's `Err("TypeError: ...")` throws a real `TypeError`, a
    wrong argument type is a `TypeError`, and a panic carries `code: 'RUST_PANIC'`;
  - async exports reject their promise instead of throwing at the call;
  - `Uint8Array`, `Buffer`, `Map`, `Set`, `Date` and `bigint` cross the bridge with the markers
    recordings use, instead of turning into plain objects or throwing;
  - a string with a lone surrogate is refused with a `TypeError` instead of silently becoming
    U+FFFD in a Rust `String`;
  - `build` replaces the `.node` (and the PyO3 module) by renaming, and `diff` refuses to run when
    the built module is older than the crate's sources.
- From Reddit feedback:
  - `analyze` refuses a compiled app (APK, JAR, .exe and the like) with an explanation instead of
    an empty plan, and warns when a folder holds only compiled files.
  - Platform-dependent C (inline assembly, `volatile`, bitfields, packed layouts, compiler
    extensions, integers cast to addresses) is a new `platform-specific` risk, and its unit is
    kept in C: a recording on this machine says nothing about the target.
  - `analyze --whole-app` recognizes Electron apps and plans the main process, keeping the web UI.
  - `gate` has a `size` check: the port fails above 3x the original's lines (plus 50).
  - README FAQ: APKs, battle-tested C, Electron, C#, and code growth.
- C#, in preview: detected from a `.csproj` or `.sln`. `analyze` reads projects and their NuGet
  packages (mapped to crates, test projects left out), resolves `using` directives to folders,
  finds `public static` methods with their types, and flags C#'s risks (`decimal`, `dynamic`,
  LINQ queries, `Task`, `lock`, `unsafe`, reflection, regex features the `regex` crate lacks).
  `analyze --whole-app` finds CLIs (`<OutputType>Exe`) and ASP.NET servers (the web SDK,
  `WebApplication.CreateBuilder`). `app record`, `app replay` and `app front-door` run built C#
  apps (`dotnet out/app.dll`). Bridging C# units (P/Invoke over a C ABI) comes next; `bridge` says
  so for now.
- Fix ([#25](https://github.com/erkamyaman/rust-anything/issues/25)): a bridge crate in its default
  place inside the project (`rust/<crate>`) was read as source, so Java `record` compiled its
  generated wrappers next to the originals ("duplicate class"). Every source walk now skips
  folders holding a `rust-anything.json`.
- Fix ([#26](https://github.com/erkamyaman/rust-anything/issues/26)): on Windows, paths handed to
  `java -cp`, `-D` library paths and the other runtimes carried the `\\?\` verbatim prefix, so
  Java `record`, `diff` and `bench` could not find their classes. Paths are now made absolute
  without it everywhere.
- Fixes found by gating every demo port ([#27](https://github.com/erkamyaman/rust-anything/issues/27)
  reported the first): the generated binary wire tripped the new
  `chunks_exact_to_as_chunks` lint (every Go and Java port failed `gate`), and on macOS the
  Python and Ruby extension's test binary failed to link; the generated `build.rs` now covers it,
  and `refresh` adds it to Ruby crates too. CI now gates all demo ports on macOS.
- CI also runs every test on macOS, and a new `tests/native.rs` builds a napi addon and a PyO3
  extension for real, loads them from Node and Python and replays the recording, so link problems
  like #4 fail CI instead of reaching users.
- A `.github/FUNDING.yml` turns on the repository's Sponsor button.

## 1.2.0 (2026-11-17)

Numeric code, already compiled code, and the edges of what a recording proves, after the first
round of feedback. rust-anything is now on crates.io: `cargo install rust-anything`.

- Float tolerance for numeric code: `record --float-tolerance 1e-9` (and `app record`) lets floats
  match when they differ by at most that share of the larger value (absolute below 1); `diff`,
  `gate`, `app replay` and the lift use it. Integers, text, structure and formatting (`3` against
  `3.0`) still match exactly. `diff --float-tolerance` tries one for a single run.
- Noise is sorted by kind: when the original's two runs disagree, `app record` reports `values`
  (now `[..]`), `order` (the same lines in another order: replay now compares them in any order
  instead of wildcarding them, so a port must still produce exactly those lines) and `lines` (a
  `...` that matches anything), by case and field. The lift's trycmd test skips `order` cases and
  says to run `app replay` in CI for them.
- Regex traps: functions in files that use regexes get extra string inputs where engines
  disagree (Turkish i, Kelvin sign, long s, joiners, non-ASCII digits and spaces, line separators,
  final sigma), in all six languages. Every string trap now also includes a dotless `ı`, a
  zero-width-joiner emoji and a non-breaking space.
- C: recording runs the library in the "C" locale, as a C program has unless it calls
  `setlocale`. It ran in the user's locale (Python sets it at startup), so `isspace` and friends
  could record a different answer per machine; the new non-breaking-space trap caught it.
- The pitch says what passing means: the Rust matches the original's recorded behavior. The
  README has a "What it can't catch" section.
- Already compiled code: `profile` and `analyze` flag Python functions compiled by numba
  (`@njit`, `@jit`, `@vectorize`, ...), Cython or JAX, and the hot functions that call them, since
  a Rust port gains little speed there.
- Go: crates no longer build a static library. Go 1.26.11 and 1.27.4 refuse to link archives
  that carry LLVM bitcode, which every Rust static library does; cgo links the shared library
  anyway. Run `rust-anything refresh <crate>` on a Go crate bridged by an earlier version.

## 1.1.0 (2026-11-16)

Whole-app ports: rust-anything now takes a TypeScript/JavaScript, Python, C, Go, Ruby or Java app
all the way to a pure Rust app, proving at every step that it still behaves the same. CLIs and HTTP
servers in all six languages, and libraries (proven end to end in TypeScript, Python and C so
far); every `app` command is also an MCP tool (`app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`).

- Plan: `analyze --whole-app` gives the app's shape (CLI, server or library) and entry points,
  every external dependency (imported, or declared in package.json, pyproject or
  requirements.txt, go.mod, Gemfile or gemspec, pom.xml or Gradle) with its crate and fit, the port
  order as pieces (units that import each other move together), and numbered decisions with a
  recommended answer for the user. POSIX headers and `com.sun.*` now map to their Rust
  counterparts.
- Record: `app record` runs the original on its own documented examples, an `--inputs` file and
  semantic traps (non-ASCII and emoji, negative division and modulo, overflow, float extremes,
  map order, missing files), twice; what differs between the two runs becomes a wildcard, and
  named normalizers (`timestamps`, `uuids`, `key-order`, `name=regex`) are saved with the cases.
  CLI cases are trycmd files in `rust/app-cases/` (argv, stdin, env and files in; stdout, stderr,
  exit code and files out). With `--http` (or `--port`) the command starts a server, and requests
  from the `curl` lines in its docs, `--inputs` and traps are stored in `rust/app-cases/http/`.
- Replay: `app replay` runs any build (the original, the front door, the final binary) against
  the cases, prints `passed/total` and the fallback rate, writes a report with a diff per failing
  case, and exits non-zero on any mismatch, so CI can run it. Any build can report its fallbacks
  through the file named by `RUST_ANYTHING_FALLBACK_LOG`. `--case <name>` reruns one
  failing case, the fast loop for fixing a mismatch.
- Front door: `app front-door` scaffolds `rust/app`, which owns the entry point from day one and
  hands what is not ported yet to the original, counting every fallback: for a CLI a clap binary
  that runs the original with the same argv, stdin and environment; for a server an axum app that
  starts the original on its own port and proxies to it; for a library (no `main`, no server, or
  `--library`) a Rust crate with the public API, while the original package keeps its callers.
  Replay against a fresh front door passes 100% at 100% fallback.
- Wire: after a unit passes `gate`, `app wire` adds its crate to `rust/app` and routes commands
  (`--command`), routes (`--route 'GET /users/{id}'`, `*`) or, for a library, the unit's exports
  to it, behind a kill switch, `RUST_ANYTHING_UNIT_<NAME>=original`; it writes a glue file to fill
  in once, then builds and replays. For a library it writes the unit's shadow drop-in for the
  original package to import, and shadows in every language but C (switched at build time) honor
  the kill switch.
- Lift: once the fallback rate is 0 and every unit passes `gate`, `app lift` makes it pure Rust.
  It refuses until then and lists every blocker; it works on a copy, removes the fallback path and
  the kill switches, turns each unit crate into a plain library (no `cdylib`, bindings or
  generated wrappers), runs the lift checklist (fmt, clippy, tests, every target in
  `rust/DECISIONS.md`, assets, replay in CI) and replays the pure binary, and writes the result
  back only when all of it passes. CLIs get a trycmd test that runs the cases on `cargo test`.
  The original is never touched.
- `gate` checks error paths: every place the original fails on purpose (`throw`, `raise`, an
  `error` result) needs a recorded failing case, and `port.rs` may not `unwrap`, `expect` or
  `panic!` in a function whose original returns an error, unless marked `// INFALLIBLE: <why>`.
- `record` adds the semantic traps to every function: each trap value of a parameter's type, one
  parameter at a time, next to the generated cases.
- Proved end to end on a small CLI and a small HTTP server written in each of the six languages,
  replayed with the original moved away, and on libraries (a C library lifted to one pure Rust
  crate; TypeScript and Python libraries moved unit by unit).
- The skill gains a whole-app guide (`whole-app.md`, installed by `setup`): it grills the user in
  rounds with a recommended answer for each question, researches dependencies against primary
  sources, and keeps the port going across sessions with `rust/MAP.md` (claim one unit at a
  time), ADRs in `rust/DECISIONS.md`, `rust/GLOSSARY.md` and a `rust/HANDOFF.md` at each
  checkpoint. It fixes red replays one case at a time, can port a wave in parallel with
  subagents, and ends with `rust/cutover.sh` for the steps only the user can do. Grilling,
  research and these practices are adapted from Matt Pocock's agent skills.
- The pitch is now rewriting a whole app in Rust; making the slow part fast stays as a smaller use
  of the same tool.

## 1.0.1 (2026-11-16)

- npm: one `rust-anything` package that carries the binaries for macOS, Linux and Windows.

## 1.0.0 (2026-11-16)

First version: the full loop for TypeScript/JavaScript, Python, C, Go, Ruby and Java.

- Python: PyO3 crates, typed exceptions, literal defaults and keyword-only parameters kept.
- C: drop-in `extern "C"` replacements and a header; crash-safe recording through ctypes; shadow
  builds with `-D` renames.
- Go, Ruby, Java: one JSON dispatcher and generated wrappers (cgo package, Fiddle file, JNI
  classes) with the original API; generated drivers record and replay in the original runtime;
  shadow drop-ins per language.
- Direct bridges: PyO3 conversion without JSON and `pynum::Num`, a magnus extension for Ruby,
  typed JNI and cgo functions, and a generated binary wire for lists, maps, structs and several
  results in Go and Java, and for TS through napi-rs and wasm-bindgen; `bench` times calls in process and ends with a verdict.
- `profile`: hot functions of a real workload (node `--cpu-prof`, cProfile, `go test` pprof, JFR,
  a Ruby tracer) with callers, call counts and advice. Batch classes (`--batch fn:param`) for TS,
  Python, Ruby, Go and Java keep items in Rust and run on all cores, with mimalloc; a
  `<Function>Index` in `port.rs` prepares the items once. x18n's search: 20x faster than TS (11.5x in wasm), with an index that ranks by itself; Go
  and Java batches 4x and 3x faster than the originals.
- Run end to end on Go's `path`, Ruby's `shellwords`, Python's `colorsys`, and JDK and glibc
  behavior; fixes from those runs: exact float parsing, crash isolation, `module_function :name`,
  numeric inference for untyped parameters, ints in Python inputs.

- `analyze`: units, port order, risks with `file:line`, bridge per unit, npm-to-crate fit,
  readiness and priority; counts bridgeable exports; follows tsconfig paths and workspace packages.
- `bridge` / `refresh`: crates with napi-rs or wasm-bindgen bindings generated from `port.rs`;
  functions, async functions and classes; JS-only exports stay in TS with a reason.
- `record` / `diff` / `bench`: recorded TS behavior from shaped, edge-case inputs and class
  scenarios, replayed against Rust; timing with the bridge included.
- `shadow`: drop-in modules running TS and Rust side by side, `RUST_ANYTHING_MODE=old|shadow|new`.
- `gate`: fmt, clippy, tests, `unsafe` audit, nothing unported, build, API, behavior.
- `setup`: registers the MCP server and the workflow skill with Claude Code, Codex, Cursor and
  Gemini CLI, keeping other settings and backing up each file it changes; `--remove` and
  `--dry-run`.
- `mcp`: every step as an MCP tool; agent skill in `skills/rust-anything`.
- Install with `npm install -g rust-anything` (prebuilt binaries for macOS, Linux and Windows) or
  from the binaries attached to the GitHub release.
