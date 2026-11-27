# Evals

How much the rust-anything skill helps an agent move code to Rust, compared with the same agent
without it. Back to the [README](../README.md).

**No results are published yet.** Numbers go here only after a full run (every case, both arms,
at least 3 runs each) on stated hardware, with the run count, the model, the Claude Code version
and the rust-anything version next to them. Nothing on this page is an estimate.

## What runs

The suite lives in [`evals/`](../evals) and runs with `claude plugin eval`. Each case is run twice
per repetition: once with the rust-anything plugin loaded (the with arm) and once without it (the
baseline arm). Same prompt, same tools, same starting folder.

| Case | Module | Caller (must not change) | Functions | Held-out calls |
| --- | --- | --- | --- | --- |
| `ts-textkit` | `textkit.ts` | `cli.ts`, run with `node cli.ts` | `slugify`, `truncate`, `formatBytes`, `parseDuration` | 40 |
| `py-textkit` | `textkit.py` | `cli.py`, run with `python3 cli.py` | `title_case`, `format_number`, `parse_kv`, `wrap` | 45 |
| `go-textkit` | package `textkit` | `cmd/textkit/main.go`, run with `go run ./cmd/textkit` | `Humanize`, `Abbrev`, `ParseRange`, `Percent` | 36 |

Each module is a few functions with the edge cases real ports get wrong: UTF-16 against UTF-8
lengths and surrogate pairs, Unicode case and whitespace rules, `toFixed` and `format` rounding,
big integers, `-0`, error types and messages. Its caller is a driver that reads one JSON call per
line (`{"fn": ..., "args": [...]}`) and prints `{"ok": ...}` or `{"error": ...}`.

A scaffold script copies the module, the driver and `examples.jsonl` (sample calls) into an
empty working folder. The task is rust-anything's normal job: make the module's functions run in
Rust while every caller keeps working, unchanged, with the same behavior. The prompt says the
driver must not change, the work must happen in Rust when the driver calls the functions, the
module's own files may change, everything must be left built, and the agent should not stop for
questions.

## How it is scored

Two layers. The first comes from `claude plugin eval` itself; the second is the one that says
whether the job was done.

**In the harness** (`evals/<case>/graders/`), all deterministic:

| Grader | Arm | Passes when |
| --- | --- | --- |
| `rust-crate` | both | the run created a `Cargo.toml` somewhere in the working folder |
| `skill-loaded` | with only | the agent loaded a skill (an indicator, not part of the score) |
| `gate-run` | with only | the agent ran `rust-anything gate` (an indicator, not part of the score) |

**After the run** ([`evals/score.py`](../evals/score.py)), on the kept working folders, without
rebuilding anything the agent built:

1. **Callers unchanged**: the driver file is byte for byte the original's.
2. **Held out**: the workspace's own driver and a fresh copy of the original run every call in
   `evals/heldout/<case>.jsonl`, one process per call, and the results are compared as parsed
   JSON: values, key order and types (an int and a float of the same value differ for the Python
   case, where they are different types to a caller). These calls are not in `examples.jsonl`,
   are never copied into the working folder, and the prompt does not mention them.
3. **Runs in Rust**: a probe ([`evals/probes/`](../evals/probes)) loads the module the way the
   driver does and calls each function with every held-out call, counting the calls that entered
   a Rust library. A run passes only if every call of every function did, and the library it
   entered carries Rust's own symbols or strings (`rust_begin_unwind`, `__rust_alloc`, `/rustc/`,
   `rust_panic`, or wasm-bindgen's `__wbindgen`). Without this check, a run that changed nothing
   would pass the held-out comparison.
4. **Gate**: `rust-anything gate --json` on every crate with a `rust-anything.json` (made by
   `rust-anything bridge`). A run that never used the tool has no gate row.

How each probe sees a call enter Rust:

| Case | Probe | What counts as entering Rust |
| --- | --- | --- |
| `ts-textkit` | `probes/ts.mjs`, run with Node | a call into a function exported by a native addon loaded through `process.dlopen` (napi-rs) or by a WebAssembly instance the workspace created, when that addon or wasm module holds Rust markers |
| `py-textkit` | `probes/py.py`, run with Python | a call into a function of an extension module (PyO3 and the like) or of a library opened with `ctypes`, when that file holds Rust markers |
| `go-textkit` | `probes/go/main.go`, built inside the workspace | `runtime.NumCgoCall()` goes up during the call, and the built binary or a library it links holds Rust markers |

The probes check that each call crosses into Rust, not how much of the work happens there: a
port that calls a trivial Rust function and does the rest in the original language would pass.
A port that rejects some arguments in the original language before calling Rust fails the check
for that function. `--out` writes each probe's per-function counts and the libraries it saw.

`score.py` reports, per case and arm: runs, runs with the callers unchanged, runs in Rust,
**exact** (the result or the error message is identical) and **outcome** (exact, or both sides
returned an error with a different message), each as matched calls over all held-out calls, and
gate passes over bridged crates. `--out` also writes every mismatch with its call, the expected
line and the workspace's line.

## Run it

You need, on `PATH`: Node 22.18 or later (type stripping), Python 3, Go, a Rust toolchain with
clippy and rustfmt, and a `rust-anything` built from this checkout (the skill describes the
commands of the same version, so an older install gives the with arm a CLI that does not match).

The eval runs the agent with a sandbox home and denies reads under your home folder, except for
folders on `PATH`. A rustup toolchain lives under `~/.rustup`, so put its `bin` and `lib` folders
on `PATH` directly; the rustup proxies in `~/.cargo/bin` cannot find a toolchain from the sandbox.
The sandbox also blocks the network unless you grant domains, and cargo needs crates.io for the
crates both arms depend on (napi, PyO3, serde_json).

From the repository root:

```sh
cargo build --release
TC="$(rustc --print sysroot)"
export PATH="$TC/bin:$TC/lib:$PWD/target/release:$PATH"

claude plugin eval . --scaffold --keep-temp --no-publish --trust-plugin \
  --allow-tools Bash Write Edit "WebFetch(domain:index.crates.io)" "WebFetch(domain:static.crates.io)" \
  --json evals/results/run.json

python3 evals/score.py --results evals/results/run.json \
  --rust-anything target/release/rust-anything --out evals/results/heldout.json
```

- Runs per case: 3 by default (`runs:` in each `case.yaml`), times 2 arms: 18 agent runs for the
  suite. Each run may take up to 150 turns and 45 minutes. They are billed to your account.
- One case: add `--case ts-textkit`. Fewer runs: `--runs 1` (a smoke test, not a result).
- Check that every case loads and the plugin resolves, without starting an agent:
  `claude plugin eval . --scaffold --trust-plugin --max-cost-usd 0 --no-publish`.
- `--trust-plugin` answers the first-run trust prompt; leave it out to be asked in a terminal.
- The plugin's MCP server is not started in an eval (it has no mock), so the with arm uses the
  `rust-anything` CLI, as the skill says to when the MCP tools are missing.
- `--keep-temp` keeps each run's sandbox; `score.py` finds the working folder from each run's
  `tracePath`. The harness seals what the agent wrote (`<kept>/sealed`, mode 000); `score.py`
  opens the kept folder and `sealed/` with `chmod 700`, as the harness notice says, and copies
  the working folder back to the path it had during the run, so libraries linked by absolute
  path still load; it removes that copy when done. It runs the agent's code (build scripts,
  native libraries) as you, so score only runs you started.
- To score one folder by hand: `python3 evals/score.py --case go-textkit --workspace <folder>`.
- `evals/results/` is git-ignored.

## Publishing results

A result row names the date, the machine (CPU, cores, memory, OS), the model, `claude --version`,
`rust-anything --version`, the run count per arm, and both layers above for both arms, including
the cases where the skill did not help. Keep the `aggregate-result.json` and the `--out` file of
the run it came from.
