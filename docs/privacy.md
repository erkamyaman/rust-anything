# Privacy

rust-anything runs on your machine. The CLI, the MCP server and the skill send none of your code,
recordings or results anywhere, collect no telemetry, and need no account.

What does reach the network is what you or your agent run: Cargo downloads crates when a port is
built, `npm`, `pip` and the other package managers install what the original needs, and `gate`
runs `cargo deny` or `cargo audit` when one is installed, which fetch the RustSec advisory
database. Your agent's model provider sees what your agent sends it, under its own policy.

To share a failing diff or replay without the data in it, use `diff --redact` or
`app replay --redact` (`redact: true` over MCP): mismatches come out as shapes only. Where the
outputs first differ, types, lengths and line and column are kept, function and case names and
object keys too; arguments, outputs and request paths are replaced by FNV-64 hashes (strings,
arrays and objects) or by their type alone (numbers, booleans, null), so a value can be compared
across runs without being shown.

`app shadow` sends live requests to the original and to your build, both on your machine, and
writes only shapes of what differed (method, a hash of the path, where and how each field
differs, as `--redact` shows it) to
`rust/app-cases/shadow/mismatches.jsonl`. Request values are written only with `--keep`, to
`rust/app-cases/shadow/inputs.jsonl`; that file can hold what your users sent (headers, cookies,
bodies), so keep it out of version control unless you have checked it.

Questions: [open an issue](https://github.com/erkamyaman/rust-anything/issues).
