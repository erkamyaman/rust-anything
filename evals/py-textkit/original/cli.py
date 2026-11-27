import json
import sys

import textkit

FUNCTIONS = {
    "title_case": textkit.title_case,
    "format_number": textkit.format_number,
    "parse_kv": textkit.parse_kv,
    "wrap": textkit.wrap,
}


def main():
    for line in sys.stdin:
        if not line.strip():
            continue
        call = json.loads(line)
        try:
            fn = FUNCTIONS.get(call["fn"])
            if fn is None:
                raise LookupError(f"unknown function {call['fn']}")
            out = {"ok": fn(*call["args"])}
        except Exception as e:
            out = {"error": f"{type(e).__name__}: {e}"}
        sys.stdout.write(json.dumps(out) + "\n")


if __name__ == "__main__":
    main()
