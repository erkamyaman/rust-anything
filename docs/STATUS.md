# Status

What works, and how each part was last checked. Updated 2026-11-18.

**Verified**: run end to end (bridge, record, port, diff, gate) on real code someone else wrote,
named below. **Tested**: covered by this repository's tests on fixtures written for them, not yet
run on a real project. **Preview**: works for part of the flow, as described.

## Porting units

| Language | Status | Last real run | Result |
| --- | --- | --- | --- |
| TypeScript/JavaScript | Verified | [json-server](real-world.md#json-server) (2026-11-18), [Fuse.js](real-world.md#fusejs) (2026-11-18), [vercel/ms](real-world.md#vercelms) (2026-11-18), x18n's `packages/core` | json-server: six units, 3,399 cases, gate passes on each; Fuse.js, vercel/ms, x18n: 3,414/3,414, 975/975 and 7,925 cases match |
| Python | Verified | `colorsys.py` from CPython 3.13 (2026-11-16) | 2,400/2,400; gate passes |
| Go | Verified | `path` from Go 1.24 (2026-11-16) | 2,642/2,642; gate passes |
| Ruby | Verified | `shellwords.rb` from Ruby 3.3 (2026-11-16) | 1,617/1,617; gate passes |
| Java | Verified | JDK 21 behind a facade (2026-11-16) | 4,492/4,492; gate passes |
| C | Verified | glibc behind a facade (2026-11-16) | 1,859/1,859 after dropping one undefined-behavior case; gate passes |
| PHP | Preview | none | `analyze` and whole apps only; units cannot be bridged yet |
| C# | Preview | none | `analyze` and whole apps only; units cannot be bridged yet |

Details of every run: [real-world.md](real-world.md) (TS/JS) and
[real-world-languages.md](real-world-languages.md) (the others).

## Commands

| Command | Status | Notes |
| --- | --- | --- |
| `analyze` | Verified | every language above, x18n's 142 units |
| `bridge`, `record`, `diff`, `gate`, `build` | Verified | the runs above; `bridge --as-json` on json-server's `paginate` |
| Types for untyped JS in `bridge` (sibling `.d.ts`/`.d.mts`/`.d.cts`, then JSDoc) | Tested | `tests/untyped_js_types.rs` |
| `bench` | Verified | the runs above; numbers in [real-world.md](real-world.md) |
| `record --from-tests` | Verified (TS/JS), Tested (Python) | vercel/ms's Jest suite |
| Coverage of the original in `record` and `gate` | Verified (TS/JS) | Fuse.js: cases run 81% of the lines it imports; other languages do not measure it yet |
| `--batch` | Verified | Fuse.js, vercel/ms, x18n search |
| wasm bridge (`--kind wasm`) | Verified | x18n search |
| `shadow` | Verified | x18n's `packages/core` |
| `profile` | Tested | |
| `app record`, `app replay` | Verified (servers), Tested (CLIs) | json-server: 514 HTTP cases with `--fresh`, `--arg`, `--pace`; CLIs in `tests/app.rs` |
| Front door (`front-door`), `wire` | Verified (TS server) | json-server: reads wired first (22.6% fallback), then every route (0%); servers in eight languages in `tests/http.rs` |
| `lift` | Verified (TS server), Tested (CLIs) | json-server: pure Rust binary, 514/514; CLIs in all eight languages in `tests/app.rs` |
| MCP server (`mcp`) | Tested | protocol test in `src/mcp.rs` |
| `setup` | Tested | registers the MCP server (and the skill, where the agent reads skills) with Claude Code, Codex, Cursor, Gemini CLI and OpenCode; per-agent tests in `src/setup.rs`, read back in `src/doctor.rs` and `tests/doctor.rs`. The OpenCode entry was also confirmed by OpenCode 2.3.2 itself (`mcp.servers`, `command` as an array) |

## Where the tests run

CI installs Node, Python, Go, Ruby and Java on macOS and Windows. It does not install PHP or
.NET, so the PHP and C# tests skip there and run only on a machine that has them.
