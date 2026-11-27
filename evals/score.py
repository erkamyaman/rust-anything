#!/usr/bin/env python3
"""Score kept eval workspaces: do the callers still work, and is it Rust that runs?

For each workspace:
  callers    the driver files are byte for byte the original's
  held-out   the workspace's own driver against a pristine copy of the original, one process
             per call in evals/heldout/<case>.jsonl, compared as parsed JSON
  rust       a probe (evals/probes/) imports the module the driver imports and counts the
             held-out calls that entered a Rust library (native addon, wasm, extension
             module, ctypes or cgo)
  gate       `rust-anything gate --json` on every crate with a rust-anything.json

  python3 evals/score.py --results evals/results/run.json
  python3 evals/score.py --case ts-textkit --workspace /path/to/one/workspace
"""

import argparse
import hashlib
import json
import os
import platform
import shutil
import stat
import subprocess
import sys
import tempfile
from pathlib import Path

EVALS = Path(__file__).resolve().parent
PROBES = EVALS / "probes"
CASES = {
    "ts-textkit": {"typed_numbers": False, "callers": ["cli.ts"]},
    "py-textkit": {"typed_numbers": True, "callers": ["cli.py"]},
    "go-textkit": {"typed_numbers": False, "callers": ["cmd/textkit/main.go"]},
}
RUST_MARKERS = [b"rust_begin_unwind", b"__rust_alloc", b"/rustc/", b"rust_panic"]
NOT_SCANNED = {"target", "node_modules", ".git", "__pycache__"}


class Obj(list):
    pass


def load(text):
    return json.loads(text, object_pairs_hook=Obj)


def same(a, b, typed):
    if isinstance(a, bool) or isinstance(b, bool):
        return type(a) is type(b) and a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        if typed and type(a) is not type(b):
            return False
        return a == b
    if isinstance(a, Obj) or isinstance(b, Obj):
        return (
            isinstance(a, Obj)
            and isinstance(b, Obj)
            and len(a) == len(b)
            and all(ka == kb and same(va, vb, typed) for (ka, va), (kb, vb) in zip(a, b))
        )
    if isinstance(a, list) and isinstance(b, list):
        return len(a) == len(b) and all(same(x, y, typed) for x, y in zip(a, b))
    return type(a) is type(b) and a == b


def kind(result):
    if isinstance(result, Obj) and len(result) == 1 and result[0][0] in ("ok", "error"):
        return result[0][0]
    return None


def run(cmd, cwd, timeout, stdin=None):
    env = dict(os.environ)
    env.pop("CARGO_TARGET_DIR", None)
    try:
        return subprocess.run(cmd, cwd=cwd, input=stdin, capture_output=True, text=True, timeout=timeout, env=env)
    except subprocess.TimeoutExpired:
        return None


def run_line(cmd, cwd, line):
    p = run(cmd, cwd, 30, stdin=line + "\n")
    if p is None:
        return {"status": "timeout"}
    lines = [l for l in p.stdout.split("\n") if l.strip()]
    if len(lines) != 1:
        return {"status": "bad-output", "stdout": p.stdout[-300:], "stderr": p.stderr[-300:]}
    try:
        return {"status": "ok", "value": load(lines[0]), "text": lines[0]}
    except ValueError:
        return {"status": "bad-json", "stdout": lines[0][:300]}


def writable_copy(src, dest):
    shutil.copytree(src, dest, symlinks=True, ignore=shutil.ignore_patterns(".git"), ignore_dangling_symlinks=True)
    for root, dirs, files in os.walk(dest):
        for name in dirs + files:
            p = os.path.join(root, name)
            if not os.path.islink(p):
                os.chmod(p, os.stat(p).st_mode | stat.S_IRUSR | stat.S_IWUSR | (stat.S_IXUSR if name in dirs else 0))


def heldout_lines(case):
    text = (EVALS / "heldout" / f"{case}.jsonl").read_text(encoding="utf-8")
    return [l for l in text.split("\n") if l.strip()]


def driver_command(case, ws, scratch, tag):
    if case == "ts-textkit":
        return ["node", "cli.ts"], None
    if case == "py-textkit":
        return ["python3", "-B", "cli.py"], None
    exe = scratch / f"textkit-{tag}"
    p = run(["go", "build", "-o", str(exe), "./cmd/textkit"], ws, 600)
    if p is None:
        return None, "go build timed out"
    if p.returncode != 0:
        return None, "go build failed: " + p.stderr.strip()[-600:]
    return [str(exe)], None


def expected_results(case, scratch):
    orig = scratch / "original"
    writable_copy(EVALS / case / "original", orig)
    cmd, err = driver_command(case, orig, scratch, "original")
    if cmd is None:
        sys.exit(f"the original does not build: {err}")
    expected = []
    for line in heldout_lines(case):
        r = run_line(cmd, orig, line)
        if r["status"] != "ok":
            sys.exit(f"the original failed on a held-out line ({r['status']}): {line}")
        expected.append(r)
    return expected


def callers_unchanged(case, ws):
    changed = []
    for rel in CASES[case]["callers"]:
        mine = ws / rel
        pristine = EVALS / case / "original" / rel
        if not mine.is_file() or hashlib.sha256(mine.read_bytes()).digest() != hashlib.sha256(pristine.read_bytes()).digest():
            changed.append(rel)
    return changed


def linked_libraries(binary):
    if platform.system() == "Darwin":
        p = run(["otool", "-L", binary], None, 60)
        lines = p.stdout.split("\n")[1:] if p else []
        return [l.strip().split(" (")[0] for l in lines if l.strip()]
    p = run(["ldd", binary], None, 60)
    libs = []
    for l in (p.stdout.split("\n") if p else []):
        parts = l.split("=>")
        path = (parts[1] if len(parts) > 1 else parts[0]).strip().split(" (")[0]
        if path.startswith("/"):
            libs.append(path)
    return libs


def rust_markers(path):
    try:
        data = Path(path).read_bytes()
    except OSError:
        return []
    return [m.decode() for m in RUST_MARKERS if m in data]


def probe(case, ws, scratch):
    heldout = str(EVALS / "heldout" / f"{case}.jsonl")
    if case == "ts-textkit":
        p = run(["node", str(PROBES / "ts.mjs"), "textkit.ts", heldout], ws, 300)
    elif case == "py-textkit":
        p = run(["python3", "-B", str(PROBES / "py.py"), str(ws), "textkit", heldout], ws, 300)
    else:
        probe_dir = ws / "cmd" / "rust-anything-probe"
        probe_dir.mkdir(parents=True, exist_ok=True)
        shutil.copy(PROBES / "go" / "main.go", probe_dir / "main.go")
        exe = scratch / "rust-probe"
        b = run(["go", "build", "-o", str(exe), "./cmd/rust-anything-probe"], ws, 600)
        if b is None or b.returncode != 0:
            return {"error": "probe build failed: " + (b.stderr.strip()[-600:] if b else "timeout")}
        p = run([str(exe), heldout], ws, 300)
    if p is None or p.returncode != 0:
        return {"error": "probe failed: " + (p.stderr.strip()[-600:] if p else "timeout")}
    try:
        report = json.loads(p.stdout.strip().split("\n")[-1])
    except (ValueError, IndexError):
        return {"error": "probe printed no report"}
    if case == "go-textkit":
        report["natives"] = [
            {"kind": "cgo", "file": f, "rust": rust_markers(f)} for f in [str(exe)] + linked_libraries(str(exe))
        ]
    return report


def runs_in_rust(case, report):
    if "error" in report:
        return False
    rust_loaded = any(n.get("rust") for n in report.get("natives", []))
    fns = report.get("functions", {})
    return rust_loaded and bool(fns) and all(r["native"] == r["calls"] for r in fns.values())


def gate_crates(ws, rust_anything):
    out = []
    for manifest in sorted(ws.rglob("rust-anything.json")):
        if NOT_SCANNED & set(manifest.relative_to(ws).parts):
            continue
        crate = manifest.parent
        p = run([rust_anything, "gate", "--json", str(crate)], crate, 1800)
        passed, failed = False, ["timeout"]
        if p is not None:
            start = p.stdout.find("{")
            try:
                report = json.loads(p.stdout[start:]) if start >= 0 else {}
                passed = bool(report.get("passed"))
                failed = [c.get("name") for c in report.get("checks", []) if not c.get("passed")]
            except ValueError:
                failed = ["unreadable gate output"]
            if not failed and not passed:
                failed = [f"gate exited {p.returncode}: " + p.stderr.strip()[-300:]]
        out.append({"crate": str(crate.relative_to(ws)), "passed": passed, "failed_checks": failed})
    return out


def score_workspace(case, workspace, arm, expected_by_case, rust_anything, keep, place_at=None):
    scratch = Path(tempfile.mkdtemp(prefix="rust-anything-score-"))
    ws = place_at if place_at is not None else scratch / "ws"
    try:
        if case not in expected_by_case:
            expected_by_case[case] = expected_results(case, scratch)
        expected = expected_by_case[case]
        lines = heldout_lines(case)
        writable_copy(workspace, ws)
        row = {"case": case, "arm": arm, "workspace": str(workspace), "heldout": len(lines)}
        row["callers_changed"] = callers_unchanged(case, ws)
        cmd, err = driver_command(case, ws, scratch, "workspace")
        row["driver_error"] = err
        exact = outcome = 0
        mismatches = []
        if cmd is not None:
            typed = CASES[case]["typed_numbers"]
            for line, want in zip(lines, expected):
                got = run_line(cmd, ws, line)
                ok_exact = got["status"] == "ok" and same(want["value"], got["value"], typed)
                ok_outcome = ok_exact or (
                    got["status"] == "ok" and kind(want["value"]) == "error" and kind(got["value"]) == "error"
                )
                exact += ok_exact
                outcome += ok_outcome
                if not ok_exact:
                    mismatches.append({"call": line, "expected": want["text"], "got": got.get("text") or got})
        row["exact"] = exact
        row["outcome"] = outcome
        row["mismatches"] = mismatches
        report = probe(case, ws, scratch)
        row["probe"] = report
        row["runs_in_rust"] = runs_in_rust(case, report)
        row["gate"] = gate_crates(ws, rust_anything)
        return row
    finally:
        if keep:
            print(f"kept scoring copy: {ws}", file=sys.stderr)
        else:
            shutil.rmtree(scratch, ignore_errors=True)
            if place_at is not None:
                shutil.rmtree(place_at.parent, ignore_errors=True)


def kept_run(trace):
    root = Path(trace).parent.parent
    try:
        os.chmod(root, 0o700)
        sealed = root / "sealed"
        if sealed.exists():
            os.chmod(sealed, 0o700)
            ws = sealed / "home" / "cwd"
            return (ws, root / "home" / "cwd") if ws.is_dir() and not (root / "home").exists() else (None, None)
    except OSError:
        return None, None
    ws = root / "home" / "cwd"
    return (ws, None) if ws.is_dir() else (None, None)


def runs_from_results(path):
    data = json.loads(Path(path).read_text())
    for case in data.get("cases") or []:
        name = case.get("name")
        for arm, runs in sorted((case.get("arms") or {}).items()):
            for i, r in enumerate(runs):
                ws, place_at = kept_run(r["tracePath"]) if r.get("tracePath") else (None, None)
                if ws is None:
                    print(f"skip {name} {arm} run {i + 1}: no kept working folder (run with --keep-temp)", file=sys.stderr)
                    continue
                yield name, arm, ws, place_at


def summarize(rows):
    groups = {}
    for r in rows:
        groups.setdefault((r["case"], r["arm"]), []).append(r)
    print(f"{'case':<12} {'arm':<8} {'runs':>4} {'callers':>8} {'rust':>5} {'exact':>13} {'outcome':>13} {'gate':>6}")
    for (case, arm), rs in sorted(groups.items()):
        total = sum(r["heldout"] for r in rs)
        exact = sum(r["exact"] for r in rs)
        outcome = sum(r["outcome"] for r in rs)
        gates = [g for r in rs for g in r["gate"]]
        gate = f"{sum(g['passed'] for g in gates)}/{len(gates)}" if gates else "-"
        print(
            f"{case:<12} {arm:<8} {len(rs):>4} {sum(not r['callers_changed'] for r in rs):>8}"
            f" {sum(r['runs_in_rust'] for r in rs):>5} {exact:>5}/{total:<4} {100 * exact / total:>3.0f}%"
            f" {outcome:>5}/{total:<4} {100 * outcome / total:>3.0f}% {gate:>6}"
        )


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--results", help="aggregate-result.json, or the file from `claude plugin eval --json <file>` (runs kept with --keep-temp)")
    ap.add_argument("--case", choices=sorted(CASES))
    ap.add_argument("--workspace", help="one run's working folder")
    ap.add_argument("--arm", default="manual")
    ap.add_argument("--rust-anything", default="rust-anything", help="the CLI that gate runs with")
    ap.add_argument("--out", help="write every scored run, mismatches and probe reports included, to this JSON file")
    ap.add_argument("--keep", action="store_true", help="keep the scoring copies")
    args = ap.parse_args()

    if args.results:
        targets = list(runs_from_results(args.results))
    elif args.case and args.workspace:
        targets = [(args.case, args.arm, Path(args.workspace).resolve(), None)]
    else:
        ap.error("give --results, or --case with --workspace")
    if not targets:
        sys.exit("nothing to score")

    expected_by_case = {}
    rows = []
    for case, arm, ws, place_at in targets:
        if case not in CASES:
            print(f"skip unknown case {case}", file=sys.stderr)
            continue
        print(f"scoring {case} ({arm}): {ws}", file=sys.stderr)
        rows.append(score_workspace(case, ws, arm, expected_by_case, args.rust_anything, args.keep, place_at))
    if args.out:
        Path(args.out).write_text(json.dumps(rows, indent=2, ensure_ascii=False) + "\n")
    summarize(rows)


if __name__ == "__main__":
    main()
