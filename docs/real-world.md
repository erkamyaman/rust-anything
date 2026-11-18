# Real-world runs

What happened when rust-anything met real code, and what changed in the tool because of it.

## x18n (the case-study repository)

| Target | What ran | Result |
| --- | --- | --- |
| `packages/core`, 9 units, 29 functions | analyze, bridge, record, diff, bench, shadow, gate | every gate passes; 7,925 recorded cases match; slower than TS through a bridge ([report](../demo/x18n-core/README.md)) |
| `packages/builtins/src/formats/shared/`, 7 functions | same | 3,672/3,672 cases match, 960 of them built from 40 real files of the repository (YAML locales, JSON, Markdown, TS), in LF and CRLF |
| whole repository | analyze | 142 units; `start_here` now lists only units with bridgeable exports |

Found in the tool, and fixed:

- `analyze` recommended the format parsers first, but their only exports are plugin factories
  (`properties(): Plugin`). `analyze` now counts bridgeable exports with the rules `bridge`
  applies, and leaves units with none out of `start_here`.
- Types imported from workspace packages (`@x18n/core`) were not followed, so `Plugin` looked
  like data. Type lookup now follows workspace packages to their source.
- `record` could not run a module importing `@x18n/core` at runtime: the package has no build
  output. The resolve hook now maps workspace packages to their source.
- Shaped inputs found a real bug in the `text.ts` port (Infinity in a range), then an order
  dependency on V8's sort with NaN comparators; `js::sort` now reproduces V8.

## vercel/ms

[vercel/ms](https://github.com/vercel/ms) (5.5k stars, commit `4ff48ce`) turns `"2 days"` into
milliseconds and back; most Node projects pull it in through some dependency. Ported end to end
with rust-anything 1.5.1; the crate is in
[`demo/real-world/vercel-ms-rs`](../demo/real-world/vercel-ms-rs).

| Step | Result |
| --- | --- |
| `analyze` | 1 unit (`src/index.ts`), ready, readiness 96 |
| `bridge --kind napi` | `ms`, `parse`, `parseStrict` and `format` bridged, none kept in TS |
| `record --from-tests -- jest` | 975 cases: generated ones, traps, and 340 calls captured from vercel/ms's 167 Jest tests (the first port used 149 inputs copied from them by hand) |
| port | 200 lines of Rust for 184 of TS, including a `FormatIndex` that reads `options` once per batch |
| `diff` | 784/784 matched on the first build; 975/975 with the captured calls |
| `gate` | passes (fmt, clippy, tests, error paths, size, behavior) |
| `bench`, per call | 0.56x over the recorded cases (0.39x at first): `parse` on a normal string takes 79 ns, level with JS, but edge cases and thrown errors still cost more through the bridge |
| batch, `parse` x100,000 | JS 11.8 ms, Rust 0.65 ms: **18x**; 3.4x counting the one-time handover of the strings (2.1x before strings crossed joined) |
| batch, `format` x100,000 | JS 2.1 ms, Rust 2.5 ms with the handover: **0.8x** (0.2x at first); `format(x, { long: true })` 1.2x |

What the port had to get right, all caught by recorded cases or traps:

- `/.../i` without the `u` flag folds only ASCII letters, while Rust's `regex` folds Unicode, so
  `"1 ſ"` (long s) and the Kelvin sign would have parsed as seconds. The port matches units by
  hand, ASCII only.
- `Math.round` rounds halves up (`-2.5` gives `-2`); Rust's `round` goes away from zero.
- `${n}` prints `1e+21`, `5e-324` and `0` for `-0`; the port reproduces JS number-to-string.
  Its first version also had a tie bug (`25.728225708007812` printed as `...813`) that ms's
  integer outputs never reach.
- `str.length` counts UTF-16 units, so the 100-character limit counts an emoji as two.

Found in the tool, and fixed: a napi batch method with a structured parameter (`format`'s
`options`) did not compile, since the generated binding took a `serde_json::Value` that napi
cannot convert without its `serde-json` feature. Crates with batch classes now enable it.

Taken back into the tool from this port:

- The generated `js.rs` has `js::number_to_string` and `js::math_round`, checked against Node on
  20,000 random doubles; the port uses them instead of its own copies. `number_to_string` takes
  its digits from serde_json's printer: 5x faster than `format!` for integers, 3x for fractions.
- napi batches send string items as one joined string with UTF-16 end offsets, number items as
  one `Float64Array`, and return string results joined (`index.cjs` splits them with `slice`):
  2x faster for strings and 20x for numbers than one JS value per item. `new Batch(items)` and
  `run()` keep their shape.
- `analyze` flags `Math.round`, `toFixed` and `/i` regexes without `u` that mention `s` or `k`,
  with guidance on each.
- Calls cost less: on napi every sync function goes over the binary wire with no napi arguments
  (an argument costs about 55 ns, the whole call 22 ns), one JS wrapper per function instead of
  two, `serde_json::Value` parameters cross as tagged values of any shape, and errors come back
  in the wire buffer. `parse` per call went from 121 to 79 ns.
- `record --from-tests` replaces copying test inputs by hand: it records each call the suite makes
  while the tests run, with the capture code kept out of coverage (ms enforces 100%).

The verdict: `ms` is a good port and a bad speed target. Correct on every recorded case, faster
only when a program parses many durations in one go.

## Fuse.js

[Fuse.js](https://github.com/krisk/Fuse) (20k stars, about 20 million npm downloads a week,
commit `edf2fb6`) is the fuzzy-search library behind countless search boxes. Ported with
rust-anything 1.6 (unreleased); the crate is in [`demo/real-world/fuse-rs`](../demo/real-world/fuse-rs).

`analyze` put every folder of `src/` in one import cycle, so Fuse cannot be ported file by file.
The port takes the path a search spends its time on instead: a 54-line adapter
(`adapter/relevance.ts`) exports Fuse's own score for one string of a string list, built from
Fuse's `BitapSearch`, field norm and `computeScoreSingle`. That function is what a search over
50,000 strings runs 50,000 times. `bridge --batch relevance:text` keeps the strings in Rust and
runs a search over all of them, on all cores.

| Step | Result |
| --- | --- |
| adapter | the adapter's ranking equals `new Fuse(list).search(q)` on 81 queries and option sets before any Rust |
| `record --inputs` | 3,414 cases: titles from Fuse's own fixtures, non-ASCII, emoji, patterns longer than 32 units, every supported option, and wrong-typed options (refused, 149 cases) |
| port | 673 lines of Rust for the 839 lines of Fuse it stands in for |
| `diff` | 3,414/3,414 match on the first build |
| `gate` | passes |
| drop-in | `withRust(Fuse)` answers string-list searches in Rust and passes everything else to Fuse; it returns byte-identical results to Fuse on 360 searches (12 option sets, 15 queries, with and without `limit`) |

Search time per query, Apple M4 (10 cores), Node 24, results checked identical before timing:

| List | Fuse.js | Rust | Speedup |
| ---: | ---: | ---: | ---: |
| 1,000 strings | 1.9 ms | 0.1 ms | 18x |
| 10,000 strings | 19.1 ms | 0.7 ms | 27 to 30x |
| 50,000 strings | 91.2 ms | 2.8 ms | **32x** |
| 50,000 strings, one core | 92.0 ms | 17.5 ms | 5.3x |

What the port had to get right:

- Bitap runs on 32-bit JS integers: `1 << 31` is negative and `(1 << 31) - 1` is a double that
  becomes `2147483647` at the next shift. The port keeps those values as JS has them.
- Text is indexed in UTF-16 code units, so an emoji is two positions, as in Fuse.
- `ignoreDiacritics` is NFD plus a list of combining-mark ranges and a few letters spelled out
  (`ł`, `ß`, `ı`); the port uses the same list.
- `String.prototype.trim` decides which strings Fuse skips: JS whitespace, not Rust's
  `char::is_whitespace` (which includes U+0085 and leaves out U+FEFF).

Found in the tool, and fixed: batch crates whose items were strings but whose results were not
got an unused helper and failed `clippy -D warnings`; and `gate`'s size check counted only the
adapter (54 lines, so 12.5x "too big") instead of the Fuse files it imports.

What it does not cover: object lists with `keys`, extended and token search, logical queries,
`includeMatches`, a custom `sortFn` or `getFn`. The drop-in hands those to Fuse unchanged. The
speedup needs the list to stay in Rust between searches; building it costs about as much as
Fuse's own index (3.4 ms against 2.8 ms for 50,000 strings).

## json-server

[json-server](https://github.com/typicode/json-server) (75k stars, MIT, commit `89a34a4`,
1.0.0-beta.15) serves a REST API from a JSON file: Node, tinyhttp and lowdb. It is the first whole
app ported end to end with the whole-app flow (`app record`, `front-door`, `wire`, `lift`):
the result is one Rust binary that answers what the Node server answers. Everything is in
[`demo/real-world/json-server-rs`](../demo/real-world/json-server-rs).

| Step | Result |
| --- | --- |
| `analyze --whole-app` | a server, 9 units (7 ready, 2 caution), 13 dependencies to replace (tinyhttp, lowdb, sirv, eta, milliparsec, dot-prop, sort-on, inflection...) |
| `app record --http` | 514 cases: 268 requests given with `--inputs` (every route and method, the where operators, sort, pagination, embeds, CORS preflight, static files and ranges, 29 malformed JSON bodies, `/__proto__` and other prototype names, bad escapes) and 245 traps; each recorded twice, random ids left as `[..]` |
| `front-door` | 513/514 at 99.8% fallback; the miss is a raw `ü` in the URL, which Node answers with 400 and the proxy re-encodes |
| units | `where-operators` 207, `parse-where` 289, `matches-where` 1,346, `paginate` 853 cases (calls from json-server's own tests included); `service` 461 and `app` 243 through two small adapters that run Service calls and real HTTP requests; `gate` passes on all six |
| `wire` | reads first: 514/514 at 22.6% fallback, Node answering the writes and Rust reloading the file Node wrote; then every route: 514/514 at 0% |
| `lift` | a pure Rust binary (5.3 MB), 514/514; `bin.ts`'s command line ported by hand (help, version, the file checks, the startup banner) |

After the lift the units have no bindings, so each crate's `tests/recorded.rs` replays its
recorded cases on the port: all 3,399 run with `cargo test`.

What the port had to get right, all caught by recorded cases:

- tinyhttp's router does not dispatch once: after every `next()` it matches all middleware and
  routes again, on the URL the last middleware rewrote, and runs every route for `HEAD`. So
  `HEAD /posts` runs the GET handler, then POST's, and answers POST's 400; `//posts` skips the CORS
  headers and the body parser. The port runs that loop as it is.
- milliparsec parses every POST, PUT, PATCH and DELETE body as JSON, whatever its type, and a bad
  one is a 500 with V8's message: `Expected double-quoted property name in JSON at position 7
  (line 1 column 8)`, `Unexpected token 'x', ..."bbbbbbb": x}" is not valid JSON`. The port
  reproduces V8's parser errors, positions in UTF-16 units included; checked against Node on
  3,806 malformed inputs.
- Sorting uses `localeCompare` (ICU), done with ICU4X's collator and checked on 3,000 pairs;
  `Number()`, `JSON.stringify` (`1e+21`, integer keys first), Node's two query-string parsers
  (which decode `é%E9` differently from WHATWG), dot-prop with its error messages, and inflection
  for embeds (`person` and `people`) are each checked against Node in `json-server-js`.
- `db.data[name]` reads `Object.prototype`: `/__proto__` answers `{}`, `/constructor` an empty
  200, and `PATCH /__proto__` replaces the prototype of the database, which later lookups see.
- Node's HTTP parser answers 400 and closes on raw non-ASCII targets and unknown methods.

Found in json-server: after each of its own writes, its file watcher reloads the file. When the
next request writes before that reload ends, the reload can wipe that write out: the first
recording, sent as fast as the server answered, had a POST answered 201 that later reads did not
show. `app record --pace 50` waits between requests, and the cases are recorded that way. The port
reloads only when someone else changed the file, so it does not lose that write: a deliberate
difference.

Benchmark: Apple M4 (10 cores, 16 GB), macOS 27.3, Node 24.15.0 running `lib/bin.js` as
`tsc` builds it for npm, the Rust binary built with `--release`. Both serve a copy of the same
database on loopback; `ab -k` sends the same requests, 20,000 per row (40 for the slow embed on
the big database) after a warm-up. Two runs, each started with the 1-minute load average under 3;
cells give run 1 / run 2 in requests per second. [`bench/bench.py`](../demo/real-world/json-server-rs/bench/bench.py)
is the script.

The fixture database (6 posts, 3 comments):

| Request | Node, 1 connection | Rust, 1 connection | Node, 50 connections | Rust, 50 connections |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts` | 11.1k / 10.8k | 27.3k / 23.9k | 19.7k / 20.4k | 115.0k / 110.7k |
| `GET /posts/1` | 11.0k / 11.3k | 27.7k / 25.4k | 21.9k / 22.6k | 119.9k / 99.0k |
| filter, sort, page | 10.5k / 10.0k | 25.2k / 21.5k | 18.3k / 16.4k | 107.5k / 96.3k |
| `?_embed=comments` | 8.7k / 6.6k | 22.5k / 21.8k | 13.8k / 11.0k | 98.3k / 100.6k |
| `GET /` (index page) | 7.3k / 6.3k | 27.6k / 21.7k | 10.2k / 9.5k | 135.1k / 119.3k |
| `GET /test.html` (static) | 12.0k / 11.0k | 25.8k / 25.9k | 30.3k / 26.1k | 103.9k / 104.4k |
| `POST /comments`, 10 connections | | | 2,973 / 2,467 | 5,821 / 5,976 |

A generated database (10,000 posts, 20,000 comments, 3.6 MB):

| Request | Node, 1 connection | Rust, 1 connection | Node, 50 (10) connections | Rust, 50 (10) connections |
| --- | ---: | ---: | ---: | ---: |
| `GET /posts/5000` | 8.4k / 9.2k | 14.6k / 14.8k | 15.5k / 16.1k | 69.5k / 67.4k |
| `?_page=3&_per_page=10` | 4.6k / 4.2k | 8.8k / 8.4k | 6.5k / 6.8k | 47.4k / 46.2k |
| `title:contains`, sorted | 431 / 396 | 600 / 560 | 465 / 453 | 2,516 / 2,329 |
| `?_embed=comments`, a page of 5 (10 connections) | 2.4 / 2.4 | 81 / 103 | 2.4 / 2.2 | 253 / 216 |
| `GET /posts`, the whole list (10 connections) | 328 / 319 | 399 / 407 | 381 / 387 | 1,514 / 1,657 |
| `POST /comments` (10 connections) | | | 236 / 241 | 292 / 264 |

| | Node | Rust |
| --- | ---: | ---: |
| Start to first response, fixture database | 85 / 79 ms | 9.9 / 7.9 ms |
| Start to first response, 10,000 posts | 95 / 91 ms | 48 / 47 ms |
| Memory (RSS) idle, fixture database | 74 MB | 3.5 MB |
| Memory (RSS) idle, 10,000 posts | 94 MB | 41 MB |
| Memory (RSS) peak during the runs, fixture database | 424 / 390 MB | 18 MB |
| Memory (RSS) peak during the runs, 10,000 posts | 462 / 485 MB | 289 / 291 MB |

Where the gap is small: one connection reading the whole 10,000-post list (1.2x: both spend the
time printing 1.5 MB of JSON), writes on the big database (1.1 to 1.2x: both rewrite the whole
3.6 MB file on every write, Rust coalescing queued writes as lowdb does), and the sorted title
search on one connection (1.4x). The large gains come from concurrency, which a Node process does
not have, and from the embed, where json-server filters every comment for every post (the port
indexes the comments once per request, same result).

The first lifted binary was slower than Node on the big database (33 against 298 requests per
second for the whole list, 39 against 217 for writes). Profiling found, in order: no
`TCP_NODELAY` (each large response waited about 40 ms for a delayed ACK), the whole database
cloned to add `$schema` before every write, every listed item cloned before printing, one lock
for all requests, and `item.id` hashed with SipHash for each of 10,000 items. All fixed; the 3,399
unit cases and 514 HTTP cases pass after each change.

Found in the tool, and fixed:

- A server that writes its state could not be recorded twice from the same start:
  `--fresh <path>` now copies the database (and `public/`) into a fresh working directory for every
  start, from a snapshot kept with the cases; `--arg` gives every build the original's arguments;
  `--pace` is above.
- `bridge` kept `paginate<T>(items: T[])` in TS (generic over anything, functions included);
  `--as-json T` bridges it as JSON values.
- The front door's routes with parameters answered axum's own 400 for `/posts/%E0`, before the
  unit saw the request; parameters are now read from the URI.
- `lift` staged only the wired unit, not the crates it depends on by path, and wrote a `lib.rs`
  that `cargo fmt --check` rejected; both fixed.
- The servers it generates did not set `TCP_NODELAY`: a response written in two parts waited
  for the client's delayed ACK, about 40 ms. They set it now.

What it does not cover: JSON5 databases (the binary says so and exits), the 100 KiB body limit
(Node's depends on how the body arrives in 64 KiB reads; the port assumes full reads), strings
with lone surrogates (they become U+FFFD), and a database value of `1e400` (Node keeps `Infinity`
in memory until the next reload; the port stores `null`, which is what the file gets).

## Fixtures written to stress the tool

| Fixture | Exercises | Result |
| --- | --- | --- |
| `tests/fixtures/async` | async functions on both bridges | 900/900, both gates pass |
| `tests/fixtures/classes` | classes: state, constructor errors, `Map` order, `split('')` on emoji | 600/600 scenarios, both gates pass |
| `tests/fixtures/aliases` | tsconfig `paths` with `extends`, comments, trailing commas | analyze and record follow them |
| `tests/fixtures/monorepo` | a package importing a sibling with no build output | record resolves it to source |

Found in the tool, and fixed: napi turned `None` into `null` (JS gives `undefined`); scenarios
too short and too random to catch a stateful bug (now up to 12 calls, reusing earlier strings);
generated `let out = ()` and immediately-called closures that clippy rejects.

## External libraries (static only)

`unjs/scule`, `unjs/ufo` and `unjs/pathe` were cloned and analyzed and bridged. Running their
code (`record` executes the original) was refused by the session's safety policy for code
downloaded from the internet, so these runs stop before `record`.

| Library | analyze | bridge |
| --- | --- | --- |
| scule (case conversion) | 2 units, ready | all 11 functions bridgeable after the fix below |
| ufo (URL utilities) | 7 units, ready; `punycode.ts` ranked first | `toASCII` bridged (untyped JS: crosses as JSON) |
| pathe (path utilities) | 5 units; `_glob.ts` caution (dynamic regex) | 13 functions bridged; `typeof path.join`-typed exports cross as JSON |

Found in the tool, and fixed: constrained generics (`camelCase<T extends string | readonly
string[]>`) were treated as "any value, functions included", which kept 10 of scule's 11
functions in TS. Type parameters with data constraints are now substituted by the constraint.

A port of scule was written (`demo` not included: it was never recorded against the original).
