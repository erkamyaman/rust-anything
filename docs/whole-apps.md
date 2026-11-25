# Port a whole app

Port a whole CLI, server or library, unit by unit, until it is pure Rust. Back to the [README](../README.md).

## Port a whole app

The goal is a pure Rust app that behaves like the original, with the original kept as reference.
Your agent writes the Rust; rust-anything orders the work, scaffolds it, and proves every step.
The [whole-app skill](../skills/rust-anything/whole-app.md) walks the agent through it, starting
with the decisions only you can make:

- **Grilling**: the agent asks in rounds, each question with its recommended answer, until every
  decision the port depends on is settled, and writes each one as an ADR in `rust/DECISIONS.md`,
  with `rust/GLOSSARY.md` mapping the original's terms to Rust names.
- **Research**: background agents read each dependency's primary sources and write
  `rust/research/<topic>.md`, with every semantic trap they found.
- **Across sessions**: `rust/MAP.md` lists every unit with its state; a session claims one unit
  before it starts, and each checkpoint leaves a `rust/HANDOFF.md` for the next session. Units in
  one wave can be ported in parallel by subagents.
- **Cutover**: the agent does what it can and writes `rust/cutover.sh`, an interactive script for
  the steps only you can do (deploys, DNS, secrets), with a way back at each step.

Grilling, research and these practices are adapted from
[Matt Pocock's skills](https://github.com/mattpocock/skills).

**Checking a rewrite in any language.** `app record` and `app replay` only run programs: the
recording is what the original did from the outside (arguments, stdin, files, HTTP), and
`app replay --build "<command>"` runs any build against it. A rewrite in Go, or by hand, is
checked the same way as the Rust front door:

```sh
rust-anything app record . -- java -jar app.jar     # what the original does
rust-anything app replay . --build "./app-go"       # does the rewrite do the same?
```

The unit tools (`bridge`, `record`, `diff`, `gate`) and `lift` are Rust-specific.

### Side effects: email, webhooks and databases

A rewrite can answer with the same JSON and still send an email twice or write different rows.
`app record` can record what leaves the process too, per case, and `app replay` compares it:

```sh
rust-anything app record . --effects http,smtp --effect-env WEBHOOK_URL=http-url \
  --db-reset "sh scripts/reset-db.sh" --db-dump "sqlite3 app.db .dump" -- python3 app.py
```

- **`--effects http`** runs a local HTTP sink that answers every request with `200 {}`, set as
  `HTTP_PROXY`, `http_proxy` and `RUST_ANYTHING_HTTP_SINK`. Each request is recorded as method,
  URL, content type and body. HTTPS through the proxy is recorded as `CONNECT host:443` and refused,
  since the sink cannot read it: point HTTPS settings at the sink with `--effect-env` instead.
- **`--effects smtp`** runs a local mail server (`SMTP_HOST`, `SMTP_PORT`, `RUST_ANYTHING_SMTP_SINK`)
  that accepts everything, and records sender, recipients and the message.
- **`--effect-env VAR=KIND`** points one of the app's own settings at a sink: `http-url`,
  `smtp-host`, `smtp-port` or `smtp-url`. Most apps read their webhook URL or mail host from a
  setting like this.
- **`--effect-response 'METHOD /path=STATUS [BODY|@FILE]'`** (repeatable) makes the HTTP sink
  answer matching requests with that status and body instead of `200 {}`: the first rule that
  matches wins, `*` is a wildcard in the method and path, and `@FILE` is read once and saved with
  the cases, so replay gets the same replies. Giving the flag again replaces the saved rules.
- **`--db-reset` and `--db-dump`** are shell commands: reset puts the database in a known state
  before each case (each start, for servers), and dump prints its state after each case (each
  request, for servers). Cases then run one at a time. Both run in the folder the case runs in
  (the server's, with `--fresh`), so `app.db` is the one the app opened; a script they name by a
  relative path (`scripts/reset-db.sh`, `reset.py`, `< schema.sql`) is still the repository's. That
  holds for files with a script extension (`.py`, `.sh`, `.bash`, `.js`, `.mjs`, `.cjs`, `.ts`,
  `.rb`, `.php`, `.pl`, `.ps1`, `.bat`, `.cmd`, `.sql`); any other relative path, an
  extensionless script or the target of `>` included, is the case folder's.

What reached the sinks is stored as `<case>.effects` next to the case and compared like stdout:
the count and order of effects matter, values that differed between the original's two runs
(dates, ids) become wildcards, and effects the original sends in varying order are compared in any
order. The settings are saved with the cases, so replay starts the same sinks for every build.

Plan first: `rust-anything analyze . --whole-app` adds the app's shape (CLI, server or library)
and entry points, every external dependency (imported or declared) with its Rust crate and fit,
the port order as pieces (units that import each other move together), and numbered decisions
with a recommended answer: keep units, blocked units, dependencies with no or a partial crate.

Then prove the whole program from the outside. `record` and `diff` prove one unit; `app record`
and `app replay` prove the app, for any program, in any language (the app is a
black box):

```sh
rust-anything app record . -- python3 cli.py          # the command that runs the original
rust-anything app replay .                            # the original must pass 100%
rust-anything app replay . --build "target/release/mytool"
```

- `app record` runs the original on the app's own documented examples (`$ mytool ...` lines in
  its Markdown and shell files, with the files they name), an `--inputs` JSONL file
  (`{"name", "args", "stdin", "env", "files"}`), and semantic traps derived from those cases
  (non-ASCII and emoji strings, negative division and modulo, overflow, large and tiny floats,
  map order, missing files). Each case runs twice; what differs between the two runs becomes a
  `[..]` wildcard. `--normalize timestamps|uuids|key-order` or `--normalize name=regex` adds
  normalizers, saved with the cases.
- Cases are [trycmd](https://docs.rs/trycmd) files in `rust/app-cases/`: `<case>.toml` (argv, env,
  exit code), `.stdin`, `.stdout`, `.stderr`, `.in/` (files in) and `.out/` (files written). Read
  and edit them by hand; a Rust test can run them with `trycmd` once the port has a binary.
- Recording again replaces what the last recording wrote: a case it no longer writes (its input
  was removed or renamed, or the original timed out on it) is deleted, in `http/` too. The names
  are kept in `rust-anything.json` as `recorded`, so a case you added by hand is left in place,
  and `app record` names it, even when an input of its name timed out. A removed case's `.in/`
  folder stays while the `fs.base` of a case that is kept is in it or goes through it.
  The names count only in the folder they were recorded in: after switching between a CLI and
  `--http`, nothing is removed.
- `app replay` runs any build against the cases, prints `passed/total`, writes
  `rust/app-cases/replay-report.md` with a diff per failing case, and exits non-zero on any
  mismatch. A build that appends to the file named by `RUST_ANYTHING_FALLBACK_LOG` (one line per
  fallback to the original), as the front door below does, also gets its fallback rate reported.
- `app replay --case <name>` reruns one case by the name the report gives it: the fast,
  repeatable loop for fixing a mismatch before running everything again.
- Each failing field names where it first differs: the line and column of the text (where its
  literal part stops matching), the JSON path inside a JSON body, or the status change
  (`200→500`). The report starts with a "Distinct differences" table: failing cases that differ
  the same way (same field, path and kind) are one row with a short stable id and a count.
- `app replay --redact` writes the report and prints the failures as shapes only: where and how
  they differ, lengths and hashes, with no outputs, arguments or request paths, for a report you
  can share.
- The report has no timings or temporary paths: two replays of the same build write the same
  file. Commit it and run `app replay` then `git diff --exit-code rust/app-cases/replay-report.md`
  in CI, and a half-ported app fails CI on any change in what passes, better or worse, until the
  report is updated with it.

Then move the entry point to Rust and hand it commands one unit at a time:

```sh
rust-anything app front-door .                            # rust/app: every command runs the original
rust-anything app wire . textkit --command upper          # after textkit-rs passes gate
# write the glue in rust/app/src/units/textkit.rs, then:
rust-anything app wire . textkit                          # 100% again, lower fallback rate
```

- `app front-door` scaffolds `rust/app`, a Rust binary named after the app: a clap parser for the
  commands and flags in the recorded cases, where every command runs the original with the same
  argv, stdin and environment and passes its output and exit code through. Each fallback is
  counted, so replaying the fresh front door gives 100% passed at 100% fallback (it builds and
  replays to show it). It never overwrites `rust/app`: the code is yours from then on.
- `app wire <repo> <unit> --command <name>` runs `gate` on the unit crate (`--skip-gate` if it just
  passed), adds it to `rust/app` as a library, writes `src/units/<unit>.rs` once for you or your
  agent to fill in (parse the arguments, call `<crate>::port`, print what the original prints),
  routes the commands to it, then builds and replays. `RUST_ANYTHING_UNIT_<NAME>=original` sends a
  wired unit back to the original until the lift.

Wire the app's own name as a command (`--command mytool`) for argv that names no command: no
arguments, unknown commands, usage errors.

When replay passes with fallback rate 0 and every unit passes gate, make it a pure Rust app:

```sh
rust-anything app lift .
```

- `app lift` refuses until both hold, and says what blocks it: the cases still falling back,
  glue still a stub, units failing gate.
- On a copy first, it removes the fallback path and the kill switches, turns each unit crate into
  a plain library of `rust/app` (no `cdylib`, bindings or generated wrappers), and adds
  `rust/app/tests/app_cases.rs`, which runs `rust/app-cases` with trycmd on `cargo test`, so CI
  needs nothing but cargo.
- Then it runs the lift checklist: fmt, clippy `-D warnings` and tests on every unit and on
  `rust/app`, every target named in `rust/DECISIONS.md` built or checked where it is installed,
  files read at run time and asset folders not embedded, and replay on the pure binary. Only when
  all of it passes are the changes written back. The original is never touched.

### Servers

The same commands prove HTTP servers. `--http` says the command starts a server; it takes its
port from `PORT` or `{port}` in the command (`--port` for one that always listens on the same
port):

```sh
rust-anything app record . --http -- node server.js
rust-anything app replay .
rust-anything app front-door .                                    # axum: every route proxied
rust-anything app wire . api --route 'GET /users/{id}' --route '*'
rust-anything app lift .
```

- Requests come from `curl` lines in the app's docs (`curl -X POST -d ... http://localhost:8080/x`,
  `--data-binary @file` sent as raw bytes, `-F`/`--form` as a `multipart/form-data` upload),
  `--inputs` (`{"name", "method", "path", "headers", "body" | "body_base64" | "body_file"}`, a
  `body_file` relative to the inputs file or the repository) and traps: path segments and query
  values swapped for the same traps as CLI arguments, bodies for text and JSON traps, and an
  unknown route. Each run starts the server fresh and sends the requests in name order, so state
  between requests is kept.
- A server that keeps state in files gets a fresh copy for every start: `--fresh db.json` (a file
  or folder, repeatable) is snapshotted into `http/files/` and copied into a new working
  directory each time the original, the front door or a build starts; `--arg db.json` is an
  argument every start gets after its command, so the Rust binary is started as the original is.
  `--pace 50` waits 50 ms after each response, for work a server does after answering (a file
  watcher reloading what it just wrote). A server gets 60 seconds to accept connections;
  `--ready-timeout 180` gives a slow start (a JVM, .NET) longer, and `--ready-timeout 5` makes a
  build that never listens fail fast.
- Cases are `rust/app-cases/http/<case>.toml` (the request, the status and the headers that matter)
  and `<case>.body` (the response body, with `[..]` where two runs differed). Date, server and
  other per-response headers are left out; delete any header line you do not care about.
- The front door is an axum server that starts the original on a port of its own and proxies
  every route no unit answers. `wire --route` takes axum routes: `METHOD /path/{param}`, `/path`
  for any method, `*` for everything else; the glue gets the route, its parameters and the
  request. `lift` removes the proxy; HTTP cases then run in CI with `app replay`.

#### Shadow live traffic

Recorded cases cover what the docs, `--inputs` and traps thought of. `app shadow` checks a build
against real requests too: a proxy sends each request to the original, relays its answer to the
client byte for byte, then sends the same request to the build and compares the two answers
(status, headers other than the per-response ones a recording leaves out, body) with the saved
normalizers:

```sh
rust-anything app shadow . --build rust/app/target/release/app --listen 8080
rust-anything app shadow . --build ./server-rs --upstream 127.0.0.1:3000 --keep
```

- Clients only ever get the original's answer, and the build is asked after the client has it,
  so a slow or broken build changes nothing for them. Requests are handled one at a time.
- The original runs for real: the saved command starts it (with its saved `--arg`s), or
  `--upstream HOST:PORT` points at one already running. Each side effect happens once, there.
- The build runs on a port of its own with the HTTP and mail sinks of `--effects` always on
  (`HTTP_PROXY`, `SMTP_HOST`, `SMTP_PORT` and any saved `--effect-env`), and the `--fresh` paths
  copied from the repository into a working directory of its own when it starts. Its effects are
  dropped, not compared. `--db-reset` and `--db-dump` are not run, and a database the build
  reaches over the network is not isolated: point the build at a copy.
- A header whose value differed between the original's own runs when it was recorded is checked
  for presence only.
- Each mismatch is a line in `rust/app-cases/shadow/mismatches.jsonl` with the method, a hash of
  the path and, for each field that differed (`body`, `status`, `header content-type`), its
  shape as `app replay --redact` writes it: where it first differs, kind, line and column,
  lengths, never the values. The terminal shows the same, and at the end the distinct
  differences with how many requests each covers. `--keep` appends the request itself to
  `rust/app-cases/shadow/inputs.jsonl`, which `app record --inputs` turns into a case, so a
  mismatch seen live is replayed with the full diff from then on.
- It runs until Ctrl-C; `--requests 100` stops after 100 requests and exits 1 if any mismatched.
  CLIs are not covered yet, and there is no MCP tool, since it runs until stopped.

### Libraries

A library has no process to replay: its public API is the front door, and each unit's recorded
cases are the proof.

```sh
rust-anything app front-door .        # rust/app: a Rust crate with the public API
# each unit through the loop: bridge, record, port.rs, build, diff, gate; then:
rust-anything app wire . textkit-rs   # re-exported from rust/app; the original imports its shadow
rust-anything app replay .            # every wired unit's cases; fallback = exports still in the original
rust-anything app lift .
```

- `app front-door` finds the public API (every unit's exports) when the repository has no `main`
  and no server (`--library` otherwise), and scaffolds `rust/app` as a library crate. Each wired
  unit becomes a module of it: `pub use textkit_rs::port as textkit;`.
- `app wire` takes a crate made by `bridge`, runs `gate`, and writes the unit's shadow drop-in:
  the original package imports it in place of the unit, so its callers keep the original API and
  reach the Rust. `RUST_ANYTHING_UNIT_<NAME>=original` sends the unit back (C: build without the
  shadow's flags).
- `app replay` runs every wired unit's recorded cases through its bindings. The fallback rate is
  the share of public exports Rust does not answer yet: unwired units, and exports `bridge` keeps
  in the original language.
- `app lift` needs fallback 0 and every unit through `gate`; it strips the bindings, wrappers and
  shadows from the units and leaves `rust/app` as one pure Rust library. It does not touch any
  `port.rs`, so the cases that matched through the bindings still describe it; the checklist
  warns about files of the original that still import the shadows.

### Saved settings (`rust/app-cases/rust-anything.json`)

`app record` saves its configuration in `rust/app-cases/rust-anything.json`, and `app replay` and
`app lift` read it on every run. Editing this file by hand is supported (for example to remove a
sticky setting or clear `unordered`). `app front-door` reads it once, when it scaffolds `rust/app`,
and copies `run` into `rust/app/src/original.rs`: after changing `run`, edit that file too and
rebuild, since `app front-door` never overwrites an existing `rust/app`.

| Field | Flag | On re-record |
| --- | --- | --- |
| `name` | `--name` | Replaces if passed; keeps saved otherwise |
| `run` | `-- <cmd>` | Replaces if passed; keeps saved otherwise |
| `timeout_secs` | `--timeout` | Replaces if passed; keeps saved (default `30`) otherwise |
| `normalize` | `--normalize NAME` | Merges (appends new built-in normalizer names) |
| `normalizers` | `--normalize NAME=REGEX` | Merges (inserts or updates by `NAME`) |
| `http` | `--http`, `--port`, `--arg`, `--pace`, `--ready-timeout` | Sticky once set; cannot be cleared via CLI flags |
| `http.port` | `--port` | Replaces if passed; keeps saved otherwise |
| `http.args` | `--arg` | Replaces if passed; keeps saved (cannot be cleared via CLI) |
| `http.pace_ms` | `--pace` | Replaces if passed; `--pace 0` clears it |
| `http.ready_secs` | `--ready-timeout` | Replaces if passed; `--ready-timeout 0` restores the default `60` |
| `http.fresh` / `fresh` | `--fresh` | Replaces if passed; keeps saved (cannot be cleared via CLI) |
| `effects` | `--effects`, `--effect-env`, `--effect-response`, `--db-reset`, `--db-dump` | Merges with saved sink and database settings; `--effect-response` replaces the saved rules |
| `float_tolerance` | `--float-tolerance` | Replaces if passed; `--float-tolerance 0` clears it |
| `float_files` | `--float-file` | Replaces if passed; keeps saved otherwise |
| `unordered` | *(written by `app record`)* | Recomputed from `order` noise; tells `replay` to compare those lines in any order |
| `recorded` | *(written by `app record`)* | Tracks generated case names so stale cases are cleaned up on the next `app record` |

All six are MCP tools too: `app_record`, `app_replay`, `app_front_door`, `app_wire`,
`app_lift` and `app_drift`.

### Keep up with the original

The original keeps changing after the port starts, and after the lift. `app drift` checks both
sides at once:

```sh
rust-anything app drift .                                # the build is rust/app, built with cargo
rust-anything app drift . --build ./target/release/tcli  # or any build
```

- It replays the original against `rust/app-cases`: a case the original no longer matches
  changed upstream.
- It lists the examples in the original's docs (shell lines for a CLI, curl lines for a server)
  that no case records yet.
- It replays the build: a case it no longer matches while the original still does is a port
  regression; one the original changed while the build still passes is marked "original changed,
  port still old".

It writes `rust/app-cases/drift-report.md` (with a diff per case, and the same stable paths as
`replay-report.md`, so CI can commit it), leaves `replay-report.md` as it was, and exits 1 on any
change. After a change upstream: `app record` again (it adds the new examples), then port what
changed until `app replay` passes. Libraries are not covered: for units, `map --check` exits 1
while any recording is stale (see [How it works](how-it-works.md#keep-up-with-the-original)).

To check every week and on demand, in GitHub Actions:

```yaml
name: drift
on:
  schedule:
    - cron: "0 6 * * 1"
  workflow_dispatch:
  push:
    branches: [main]
jobs:
  drift:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: 24
      - uses: dtolnay/rust-toolchain@stable
      - run: cargo install rust-anything --locked
      - run: rust-anything app drift .
      - if: failure()
        uses: actions/upload-artifact@v4
        with:
          name: drift-report
          path: rust/app-cases/drift-report.md
```

Set up the original's own runtime in place of `setup-node` (`actions/setup-python`,
`actions/setup-go` and so on).
