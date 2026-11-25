#!/usr/bin/env python3
"""A/B the rust-anything skill: same tasks, different SKILL.md, scored by the tool's own gate.

    scripts/skill-eval/run.py --variant current=skills/rust-anything \
                              --variant draft=/path/to/other-skill-dir --runs 3

For each task, variant and run: copy the fixture into a scratch folder, install the variant's
skill folder as that project's `.claude/skills/rust-anything`, let the agent do the task headless,
then run `rust-anything gate --json` on every crate under `rust/`. A run passes only when a crate
exists and its gate exits 0, so the wording is graded by the recordings and not by what the agent
says. Turns, cost and wall time come from the agent's JSON result.

Uninstall the user-level skill first (`~/.claude/skills/rust-anything`) or both variants see it.
The agent runs with edits allowed and no prompts, in the scratch folder only.
"""
import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent


def run_one(task, name, skill, agent, bin_path, timeout):
    work = Path(tempfile.mkdtemp(prefix=f"skill-eval-{task['id']}-{name}-"))
    shutil.copytree(HERE / task["fixture"], work, dirs_exist_ok=True)
    shutil.copytree(skill, work / ".claude/skills/rust-anything")
    result = {"task": task["id"], "variant": name, "work": str(work)}
    started = time.time()
    try:
        proc = subprocess.run(
            [*agent, task["prompt"], "--output-format", "json",
             "--permission-mode", "bypassPermissions"],
            cwd=work, capture_output=True, text=True, timeout=timeout,
            env={**os.environ, "PATH": f"{Path(bin_path).parent}:{os.environ['PATH']}"},
        )
        try:
            out = json.loads(proc.stdout)
            result["turns"] = out.get("num_turns")
            result["cost_usd"] = out.get("total_cost_usd")
        except json.JSONDecodeError:
            result["agent_error"] = (proc.stdout + proc.stderr)[-300:]
    except subprocess.TimeoutExpired:
        result["agent_error"] = f"timed out after {timeout}s"
    result["seconds"] = round(time.time() - started, 1)
    crates = sorted(p.parent for p in (work / "rust").glob("*/Cargo.toml"))
    result["crate"] = bool(crates)
    result["gate"] = bool(crates) and all(
        subprocess.run([bin_path, "gate", "--json", str(c)], capture_output=True).returncode == 0
        for c in crates
    )
    return result


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawTextHelpFormatter)
    ap.add_argument("--variant", action="append", required=True, metavar="NAME=SKILL_DIR")
    ap.add_argument("--runs", type=int, default=3)
    ap.add_argument("--task", action="append", help="task id (default: all)")
    ap.add_argument("--agent", default="claude -p", help="headless agent command")
    ap.add_argument("--bin", default=shutil.which("rust-anything"), help="rust-anything binary")
    ap.add_argument("--timeout", type=int, default=1800)
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()
    variants = {}
    for v in args.variant:
        name, _, path = v.partition("=")
        if not (Path(path) / "SKILL.md").is_file():
            sys.exit(f"{v}: no SKILL.md in {path}")
        variants[name] = Path(path).resolve()
    tasks = [t for t in json.loads((HERE / "tasks.json").read_text())
             if not args.task or t["id"] in args.task]
    plan = [(t, n) for t in tasks for n in variants for _ in range(args.runs)]
    if args.dry_run:
        for t, n in plan:
            print(f"{t['id']}  {n}")
        return
    if not args.bin:
        sys.exit("rust-anything is not on PATH; pass --bin")
    results = [run_one(t, n, variants[n], args.agent.split(), args.bin, args.timeout) for t, n in plan]
    print(json.dumps(results, indent=1))
    print(f"\n{'task':<12}{'variant':<12}{'gate':>6}{'turns':>8}{'cost $':>8}{'secs':>8}", file=sys.stderr)
    for t in tasks:
        for n in variants:
            rs = [r for r in results if r["task"] == t["id"] and r["variant"] == n]
            ok = sum(r["gate"] for r in rs)
            avg = lambda k: sum(r.get(k) or 0 for r in rs) / len(rs)
            print(f"{t['id']:<12}{n:<12}{ok}/{len(rs):>3}{avg('turns'):>8.1f}{avg('cost_usd'):>8.2f}{avg('seconds'):>8.0f}", file=sys.stderr)


if __name__ == "__main__":
    main()
