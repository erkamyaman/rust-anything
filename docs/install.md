# Install, update and develop

Every way to install rust-anything, keep it up to date, remove it, and build it from source. Back to the [README](../README.md).

## Install

```sh
cargo install --path .
```

`record`, `diff` and `bench` run the original in its own runtime: Node 22.6+ for TS (it runs `.ts`
sources directly, adding `--experimental-strip-types` on 22.6 to 22.17), Python 3.9+ for Python
and C (ctypes), Go 1.22+, Ruby 3+, JDK 17+ (`javac` and `java`), `kotlinc` for Kotlin, PHP 8+
with the FFI extension, the .NET SDK (`dotnet`) for C#, and PowerShell 7.4+ (`pwsh`). `profile` passes `--cpu-prof` to a `node` command as arguments when that Node refuses it in `NODE_OPTIONS` (22.14 does); through a wrapper such as `npx` or `npm run` it needs a Node that allows it there (24 does). C also needs a C compiler. wasm bridges also need `rustup target add wasm32-unknown-unknown` and
`cargo install wasm-bindgen-cli --version <the crate's wasm-bindgen version> --locked`; `build` names
the exact version.

### Windows

Native Windows CI uses the `x86_64-pc-windows-msvc` Rust target. Install the stable MSVC
toolchain and Visual Studio Build Tools with the **Desktop development with C++** workload; the
C compiler and linker are required for C bridges and native Rust extensions. Install the runtime
for the source language you are migrating. Python may be available as either `python3` or
`python`; rust-anything detects both. The GNU Windows target is not exercised by CI.

Run the native bridge integration suite from a Developer PowerShell prompt:

```powershell
rustup default stable-x86_64-pc-windows-msvc
cargo test --locked --test native -- --nocapture
```

The suite builds and loads real DLL-backed Node and Python extensions and the supported native
bridges for each runtime present on `PATH`. Missing optional runtimes are reported as skipped;
Cargo, the active Rust standard library, and the MSVC linker are required. If Cargo reports that
it cannot find `std`, reinstall the active target with
`rustup target remove x86_64-pc-windows-msvc` followed by
`rustup target add x86_64-pc-windows-msvc`. If linking fails, run from a Visual Studio Developer
PowerShell and verify that `where.exe link` finds the MSVC linker. Tests use temporary folders,
including a path with spaces and Unicode characters, and remove generated artifacts after a
successful run.

## Agents

An agent needs two things: the MCP server (`rust-anything mcp`, every step as a tool) and the
skill in [`skills/rust-anything/`](../skills/rust-anything) (the workflow, with `whole-app.md` and
one file per language in `languages/`). Every route below still needs the `rust-anything` CLI
installed.

### `rust-anything setup`

`rust-anything setup` registers the MCP server with Claude Code, Codex, Cursor, Gemini CLI and
OpenCode, and copies the skill to `~/.claude/skills/`, `~/.codex/skills/` and
`~/.config/opencode/skills/`. To try it without installing, `npx rust-anything@latest setup` sets
each agent to start `npx -y rust-anything@<version> mcp`, because npm may clear the copy npx ran.

### Plugins

The repository is a Claude Code plugin marketplace (`.claude-plugin/marketplace.json`); the plugin
carries the skill and the MCP server:

```sh
claude plugin marketplace add erkamyaman/rust-anything
claude plugin install rust-anything@rust-anything
```

The same MCP server and skill ship as a Codex plugin from `.agents/plugins/marketplace.json`
(`codex plugin marketplace add erkamyaman/rust-anything`, then
`codex plugin add rust-anything@rust-anything`).

### The skill alone

The [skills CLI](https://github.com/vercel-labs/skills) finds the skill in this repository's
`skills/` folder and copies the whole folder, `languages/` included:

```sh
npx skills add erkamyaman/rust-anything                    # into this project, for the agents it detects
npx skills add erkamyaman/rust-anything -g -a claude-code   # into ~/.claude/skills/, for one agent
```

Or copy [`skills/rust-anything/`](../skills/rust-anything) yourself, as a whole folder named
`rust-anything`, into a folder the agent reads. One copy in `~/.agents/skills/` reaches Codex,
Cursor, GitHub Copilot, Gemini CLI and OpenCode.

| Agent | Your home folder | A project |
| --- | --- | --- |
| Claude Code | `~/.claude/skills/` | `.claude/skills/` |
| Codex | `~/.agents/skills/` | `.agents/skills/` |
| Cursor | `~/.cursor/skills/`, `~/.agents/skills/` (also reads `~/.claude/skills/`, `~/.codex/skills/`) | `.cursor/skills/`, `.agents/skills/` |
| GitHub Copilot | `~/.copilot/skills/`, `~/.agents/skills/` | `.github/skills/`, `.claude/skills/`, `.agents/skills/` |
| Gemini CLI | `~/.gemini/skills/`, `~/.agents/skills/` | `.gemini/skills/`, `.agents/skills/` |
| OpenCode | `~/.config/opencode/skills/`, `~/.claude/skills/`, `~/.agents/skills/` | `.opencode/skills/`, `.claude/skills/`, `.agents/skills/` |

The folders come from each agent's skills documentation (checked 2026-11-27); the skills CLI
route from its documentation and source (version 1.3.7), not from a run.

### The MCP server by hand

To add the MCP server to one project (`.mcp.json`):

```json
{ "mcpServers": { "rust-anything": { "command": "rust-anything", "args": ["mcp"] } } }
```

OpenCode takes a local server under `mcp.servers` and the whole argv in one `command` array, in
`~/.config/opencode/opencode.json` (an older file that lists servers directly under `mcp` keeps
that shape):

```json
{
  "mcp": {
    "servers": {
      "rust-anything": { "type": "local", "command": ["rust-anything", "mcp"] }
    }
  }
}
```

Every tool, its parameters and what it may touch: [docs/tools.md](tools.md). The server also
offers two prompts, `port_unit` and `port_whole_app`, built from the skill.

The server also serves the skill itself through MCP's skills extension
(`io.modelcontextprotocol/skills`): `skills/list` and `skills/get` describe it at
`skill://rust-anything/SKILL.md`, and `resources/read` returns `SKILL.md`, `whole-app.md` and
each `languages/` file. A client that supports the extension (VS Code shows server skills as
slash commands) gets the skill from the MCP server alone, with no plugin and no copied files.

## In CI

The repository is also a GitHub Action that runs `rust-anything gate` on each crate and fails the
job when any of them fails, so a port cannot drift from the original unnoticed. The runner needs
Rust, plus the runtime of the original (Node is already on GitHub's images):

```yaml
- uses: actions/checkout@v4
- uses: erkamyaman/rust-anything@main
  with:
    crates: |
      rust/parse-rs
      rust/format-rs
```

`version` picks the npm version to install (default `latest`).

## Development

```sh
cargo test                        # unit tests, fixture repo, record/diff/shadow loop (needs node), app record/replay per language
cargo clippy --all-targets -- -D warnings
cargo fmt
```

Design choices and their reasons: [DECISIONS.md](../DECISIONS.md).

## Update or remove

```sh
cargo install rust-anything                                    # update (Cargo)
npm install -g rust-anything@latest                            # update (npm)
rust-anything setup                                            # refresh agent settings
rust-anything setup --remove && cargo uninstall rust-anything  # remove (Cargo)
rust-anything setup --remove && npm uninstall -g rust-anything # remove (npm)
```

`setup --remove` takes out only rust-anything's entries; the copies it saved
(`<file>.rust-anything.bak`) stay until you delete them.
