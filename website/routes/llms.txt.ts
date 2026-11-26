import { defineHandler } from 'void'

const BODY = `# Rust Anything

> Rewrite an app in Rust one piece at a time, checked against the original at every step. Rust Anything records what the original code does (returns, errors, side effects, HTTP responses), then replays every recorded case against the Rust port. A port is switched on only when every case matches. Free and open source (MIT OR Apache-2.0). Works with Node.js/TypeScript, Python, C, Go, Ruby and Java; PHP, C#, Kotlin and PowerShell in preview.

## Install

- npm: \`npm install -g rust-anything\`
- cargo: \`cargo install rust-anything\`
- Then: \`rust-anything setup\` registers the MCP server and the workflow skill with Claude Code, Codex, Cursor and Gemini CLI.

## Prompt for a coding agent

Install Rust Anything (npm i -g rust-anything, or cargo install rust-anything) and run rust-anything setup. Then run rust-anything go -- <the slow command of this app>, and port the function it picks by following TASK.md. Done when every recorded case matches and rust-anything gate passes. Work on a copy with a test database, since recording runs the real code.

## Main commands

- \`rust-anything go\`: profile a real run and port the slow part end to end
- \`rust-anything record <crate>\`: record what the original returns, throws and does
- \`rust-anything diff <crate>\`: replay every recorded case against the port
- \`rust-anything gate <crate>\`: fail ports with copied answers, unsafe code or missing tests
- \`rust-anything app lift\`: turn a whole server into one Rust binary

## Docs

- [Quick start](https://rustanything.ai/docs/introduction/quick-start)
- [Install](https://rustanything.ai/docs/introduction/install)
- [Set up your agent](https://rustanything.ai/docs/introduction/agents)
- [The loop](https://rustanything.ai/docs/concepts/the-loop)
- [Languages](https://rustanything.ai/docs/languages/overview)
- [CLI reference](https://rustanything.ai/docs/reference/cli)
- [Port a whole app](https://rustanything.ai/docs/whole-apps/overview)

## Measured results

- Prettier: 128.6 s to 6.9 s on 2,360 files, byte-identical output
- json-server lifted to one Rust binary: 74 MB to 3.5 MB idle memory, 514/514 HTTP cases matching

## Links

- Source: https://github.com/erkamyaman/rust-anything
- npm: https://www.npmjs.com/package/rust-anything
- crates.io: https://crates.io/crates/rust-anything
- Contact: hello@rustanything.ai
`

export const GET = defineHandler((c) => c.text(BODY))
