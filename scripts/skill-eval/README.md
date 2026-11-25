# Skill eval

Compares versions of `skills/rust-anything` on the same task. A run passes only when the agent
left a crate under `rust/` whose `rust-anything gate` exits 0, so the recordings grade the skill,
not the agent's own report. Turns, cost and time come from `claude -p --output-format json`.

```sh
scripts/skill-eval/run.py \
  --variant current=skills/rust-anything --variant draft=/path/to/changed/skill --runs 3
```

Uninstall `~/.claude/skills/rust-anything` first, or both variants see it. Agent runs cost money;
`--dry-run` lists the plan. Use several runs: one run says little.

What exists: one task (`tasks/py-slug`, a Python function with a test). What does not: more
tasks, other languages, a score for whole-app ports, and any published result. Add a task by
putting a fixture folder under `tasks/` and an entry in `tasks.json`.

`tests/skill.rs` is the cheap check that runs in CI: it does not measure effectiveness, only that
the skill names commands that exist.
