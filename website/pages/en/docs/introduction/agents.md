---
title: Set up your agent
description: rust-anything setup registers the MCP server and the workflow skill with Claude Code, Codex, Cursor and Gemini CLI.
---

# Set up your agent

```sh
rust-anything setup
```

`setup` adds the MCP server to each agent it finds and installs the workflow skill where the agent reads skills. It keeps your other settings, is safe to run again, and `--remove` undoes it.

## Works with

| Agent | `rust-anything setup` adds |
| --- | --- |
| Claude Code | MCP server in `~/.claude.json`, skill in `~/.claude/skills/` |
| Codex | MCP server in `~/.codex/config.toml`, skill in `~/.codex/skills/` |
| Cursor | MCP server in `~/.cursor/mcp.json` |
| Gemini CLI | MCP server in `~/.gemini/settings.json` |
| Any other MCP client | add `rust-anything mcp` as a stdio server by hand (below) |

**Community integrations** (not maintained by Rust Anything): [dsh-rust-anything](https://github.com/lonyness/dsh-rust-anything) by [@lonyness](https://github.com/lonyness) runs Rust Anything inside DeepSeek Harness. It ships its own pinned copy of the engine (currently 3.3.0).

Before changing a file that exists, `setup` saves a copy beside it (`<file>.rust-anything.bak`). `setup --remove` takes out only rust-anything's entries; the copies it saved stay until you delete them.

## The MCP server

`rust-anything mcp` serves every step as an MCP tool over stdio: `profile`, `analyze`, `bridge`, `refresh`, `record`, `build`, `diff`, `bench`, `shadow`, `gate`, and for a whole app `app_record`, `app_replay`, `app_front_door`, `app_wire`, `app_lift`. Every tool, its parameters and what it may touch: [MCP tools](/docs/reference/mcp-tools). The server also offers two prompts, `port_unit` and `port_whole_app`, built from the skill.

To add the MCP server to one project by hand instead:

**.mcp.json**
```json
{ "mcpServers": { "rust-anything": { "command": "rust-anything", "args": ["mcp"] } } }
```

## The skill

The workflow skill ([`skills/rust-anything/SKILL.md`](https://github.com/erkamyaman/rust-anything/blob/main/skills/rust-anything/SKILL.md)) tells the agent how to run the loop: profile, plan and stop for approval, port one unit, prove it, ship it behind a shadow, and call it done only when `gate` passes. For a whole app it follows [`whole-app.md`](https://github.com/erkamyaman/rust-anything/blob/main/skills/rust-anything/whole-app.md); see [Port a whole app](/docs/whole-apps/overview).

## Claude Code plugin

```text
/plugin install rust-anything --marketplace erkamyaman/rust-anything
```

The plugin brings the MCP server and the skill; the CLI must be on your `PATH`.
