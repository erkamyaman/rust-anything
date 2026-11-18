# How it works

How rust-anything checks a port, what its commands decide, and where its limits are. Back to the [README](../README.md).

## How it works

```
profile ──> analyze ──> bridge ──> record ──> port.rs ──> diff ──> bench ──> shadow ──> gate
 where the    what to     the Rust   what the    you or      every     faster?    both side   done means
 time goes    port first  crate and  original    your agent  recorded  the        by side in  this passes
                          wrapper    does        write it    case      verdict    production
```

The original stays the source of truth: `record` runs it on edge cases (and your own inputs) and
saves what it returns or throws; `diff` replays every case against the Rust; nothing ships until
they all match and `gate` passes. Callers keep the original API throughout.

A whole app goes through the same loop unit by unit, inside a frame that proves the whole
program ([Port a whole app](whole-apps.md#port-a-whole-app)):

```
analyze --whole-app ──> app record ──> app front-door ──> per unit: the loop above, then app wire ──> app lift
 shape, every           what the app    Rust owns the       gate passes, so the unit answers in         pure Rust,
 dependency, the        does from the   entry point; the     Rust: replay 100%, fallback rate down      proved by
 decisions to make      outside         original answers                                                replay
```

## The loop

| Step | Command | What it does |
| --- | --- | --- |
| 0. Profile | `rust-anything profile <repo> -- node bench.mjs` | Runs a real workload under the language's profiler (node, cProfile, `go test` pprof, JFR, a Ruby tracer): hot functions by inclusive time, their callers, and which loops to batch; flags code that is already compiled (numba, Cython, JAX), where Rust gains little |
| 1. Plan | `rust-anything analyze <repo>` | Units, port order, risks with `file:line`, bridge per unit, dependency-to-crate fit, best first ports; `--whole-app` adds entry points and shape, every dependency (imported or declared) with its crate, pieces (units that import each other move together) and numbered decisions with a recommended answer |
| 2. Bridge | `rust-anything bridge <repo> <unit>` | Crate with the bindings (`lib.rs`, plus the wrapper the source language loads) and stubs to port (`port.rs`); `--batch fn:param` adds a class that keeps many `param` values in Rust; `--as-json T` bridges a type parameter the app only fills with data as JSON values |
| 3. Record | `rust-anything record <crate>` | Runs the original on generated edge cases (non-ASCII, emoji, `-0`, `NaN`, ints where floats are expected, ...) and on every semantic trap in every parameter (string length and case, signed division, overflow edges, float printing, map and sort order), and saves `cases.jsonl`; `--from-tests -- npx jest` (or `-- pytest`) also records every call the original's own test suite makes; `--float-tolerance 1e-9` lets floats match within that share for numeric code |
| 3. Build | `rust-anything build <crate>` | Builds the Rust side: `.node`, wasm package, Python extension or shared library |
| 3. Diff | `rust-anything diff <crate>` | Replays the recorded cases against Rust, called the way the original's callers call it; lists mismatches |
| 3. Bench | `rust-anything bench <crate>` | Times the recorded cases on the original and on Rust, bridge included |
| 4. Shadow | `rust-anything shadow <crate>` | Drop-in that runs both, returns the original's result, reports mismatches; `RUST_ANYTHING_MODE=old\|shadow\|new` |
| 5. Gate | `rust-anything gate <crate>` | fmt, clippy `-D warnings`, tests, `unsafe` audit, nothing unported, a recorded failing case for every error path of the original, no `unwrap`/`expect`/`panic!` where the original returns an error (unless marked `// INFALLIBLE: <why>`), build, API, behavior |
| Agents | `rust-anything setup` | Registers the MCP server (`rust-anything mcp`, every step as a tool) and the workflow skill ([`skills/rust-anything/SKILL.md`](../skills/rust-anything/SKILL.md)) with your coding agents |

## Hot paths: keep the data in Rust

A port called once per item pays the bridge once per item, and loses to a JIT. What wins is
moving the loop: `profile` finds it, and `--batch` generates a class that takes the items once
and runs the function over all of them per call, on all cores, with typed-array results.

```sh
rust-anything profile ../x18n -- node --experimental-strip-types $PWD/demo/x18n-search/search.ts .
# -> fuzzyScore: a quarter of the run, called per key by `matched`: batch it
rust-anything bridge ../x18n packages/core/src/fuzzy.ts --batch fuzzyScore:text
```

```js
const search = new FuzzyScoreBatch(keys)          // once
const order = search.rank(query, 0)               // per keystroke: indexes, best first
```

A `FuzzyScoreIndex` struct in `port.rs` replaces the default loop with one that prepares the
items once, and can rank by itself (`rank`): it keeps the keys lowercased with a character mask
each, a table of each key's first position of every common character (a one-letter query is one
column), and the last query's matches (typing `se` after `s` rescores only what `s` matched). On
x18n's sidebar search over 50,000 keys, typed letter by letter, with identical results:

| | Per query | vs TS |
| --- | ---: | ---: |
| TS: `map(fuzzyScore)`, filter, sort | 10 ms | 1x |
| TS with the same narrowing as the index | 4.1-5.4 ms | 2x |
| The same port, called per key | - | 0.2-0.4x |
| `FuzzyScoreBatch`, default index (4 cores) | 5.1 ms | 1.7x |
| `FuzzyScoreBatch` with `FuzzyScoreIndex` (napi, 4 cores) | 0.45-0.56 ms | **18-22x** |
| The same with wasm (one thread) | 0.95 ms | **11.5x** |

Against TS doing the same narrowing, Rust is still 7-12x faster (wasm 5x). Turn the returned
indexes into keys with a plain loop: `Array.from(order, i => keys[i])` cost 20x more, 0.4 ms.

Every language gets batches, and they beat the original even where per-call ports lose. The
same work on 4 cores, identical results and errors (benchmarks in each demo folder):

| Language | Workload | Original | Per-call port vs original | Batch vs original |
| --- | --- | ---: | ---: | ---: |
| TS | x18n search, 50,000 keys | 10 ms | 0.2-0.4x | **20x** |
| Python | `slugify`, 20,000 strings | 98 ms | - | **17x** |
| Ruby | `slugify`, 20,000 strings | 110 ms | - | **12.7x** |
| Ruby | `shellescape` (stdlib), 50,000 mixed values | 88 ms | - | **11.8x** |
| Go | `Slugify`, 50,000 strings | 49 ms | 0.93x | **4x** |
| Java | `slugify`, 50,000 strings | 43 ms | 0.68x | **3x** |

Compiled originals gain less: the per-item work is already fast, so the batch wins by the number of
cores. Batch crates allocate with mimalloc: with the system allocator the Go batch was slower than
one thread. Demo: [`demo/x18n-search`](../demo/x18n-search/README.md).

## What `bridge` decides

- Exports that cannot cross a bridge stay in the source language, with the reason, and the shadow
  module re-exports the original. For TS:
  functions that take or return functions, generators and iterators, RegExp, `instanceof` checks,
  generics (`--as-json T` bridges one the app only fills with data), and values whose types (followed through imports, re-exports and `extends`) hold
  functions. `--keep "name: reason"` keeps more, e.g. code that depends on `Intl` data.
- TS types map to Rust types, with type aliases resolved through imports and parameter types
  inferred from default values; structured values cross as a generated binary wire (JSON only
  as a fallback for shapes the wire does not model).
- JSON cannot express `undefined`, NaN, Infinity, -0 or lone surrogates: structured values carry
  markers (`{"$undefined": true}`, `{"$num": "NaN"}`, `{"$utf16": [...]}`) that the generated
  `index.cjs` translates, with helpers in the crate's `js.rs`.

`record` runs TS sources as they are: a Node resolve hook follows bundler-style imports
(`./errors` to `./errors.ts`), `--resolve-from` supplies npm packages when the repo has none
installed, and TS that needs transforming (enums, parameter properties) is retried with
`--experimental-transform-types`.

## What `analyze` looks at

- Packages (`package.json`) and source files; skips `node_modules`, build output, fixtures,
  playgrounds, tool configs. Vue/Svelte/Astro `<script>` blocks are parsed for imports.
- **Units**: a folder of source files, or a file at a package's source root.
- **Waves**: a unit's imports come in earlier waves; import cycles are ported together.
- **Bridge**: napi-rs when the unit or anything it imports needs Node; wasm-bindgen for portable
  code; keep in JS for UI and editor code.
- **Readiness** (how safe) and **priority** (safe and worth it: loops and string work, tests, size).

| Risk | Severity | Example |
| --- | --- | --- |
| String offsets | high | `slice`, `indexOf`, `matchAll`, the `d` flag: UTF-16 vs UTF-8 |
| Regex syntax | high | lookaround, backreferences, `y`: not in the `regex` crate |
| Dynamic regex | high | `new RegExp(userPattern)`: JS syntax at runtime |
| Dynamic code | high | `eval`, `new Function`, computed `import()` |
| Dynamic typing | medium | `any`, `'key' in value` |
| Ordered collections | medium | `Map`, `Set`, `Object.keys`: insertion order |
| Async, generators | medium | promise and generator boundaries |
| Class inheritance, reflection | medium | `extends`, `Proxy`, `Reflect`, `Symbol` |
| Callback-shaped APIs | medium | function-typed params and methods: chatty bridges |
| Exceptions, numbers, JSON | low | `throw`, bitwise ops, `JSON.stringify` order |

## What it can't catch

Passing means the Rust matches every case that was recorded, nothing more. What bites rewrites
usually lives outside the recording, so know where the edges are:

- **Inputs nobody recorded.** The generated cases, the traps, the original's test suite
  (`record --from-tests`) and your own `--inputs` are the ceiling. Add real inputs (production
  samples, bug reports); the user input that breaks prod is rarely one a generator thinks of.
- **An original that moved on.** `record` saves a fingerprint of the unit's files
  (`recorded-from.json`); when the original changes later, `diff` warns and `gate` fails until you
  record again, instead of passing against what the original used to do.
- **Regexes.** The `regex` crate has no lookaround or backreferences, and even a pattern that
  compiles can treat `\w`, `\b`, `\d` and case-insensitive matching on non-ASCII text differently
  from JS, Python or Java. Swapping in `fancy-regex` or rewriting the pattern can pass every
  recorded case and still differ on a Turkish dotless `ı` or a zero-width joiner. `record` adds
  regex traps (those, the Kelvin sign, long s, Arabic-Indic digits, line separators, final sigma)
  to every function in a file that uses regexes, but fuzz your real patterns against real text too.
- **What changes between runs.** `record` runs the original twice and sorts out what differed:
  changed values (times, ids) become wildcards; the same lines in another order (hash maps,
  concurrency) are compared in any order, so a port still has to produce exactly those lines; a
  different number of lines becomes `...`, which matches anything. `record` lists each kind:
  look at every one, since a hash-order or concurrency difference is exactly what can change for
  real in a port.
- **Floats.** Number-to-text formatting differs between languages (`1e21`, `-0`, digit counts).
  The traps include those values, and they must match exactly; `--float-tolerance` forgives
  rounding only, never formatting.
- **Anything not in the output.** Timing, memory, concurrency under load, and side effects other
  than stdout, stderr, exit codes, written files and HTTP responses (network calls, databases,
  signals) are not compared.
- **Other platforms.** A recording proves behavior on the machine that ran it. C whose results
  depend on type sizes, endianness or the compiler can behave differently on the target; `analyze`
  keeps hardware-facing C in C for that reason.
