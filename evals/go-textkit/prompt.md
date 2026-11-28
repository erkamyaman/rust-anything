This folder holds textkit, a small Go library: the `textkit` package (`textkit.go`), and `cmd/textkit/main.go`, a command-line driver that other services call and that imports it. The driver reads one JSON call per line on stdin, `{"fn": "<name>", "args": [...]}`, and prints one JSON line per call, `{"ok": <result>}` or `{"error": "<message>"}`.

Make textkit's functions run in Rust, with every caller working as it does today.

- Do not change `cmd/textkit/main.go`. `go run ./cmd/textkit` must keep printing exactly what it prints now for any input, error results and their messages included, not only for the sample calls in `examples.jsonl`.
- When the driver calls textkit's functions, the work must happen in Rust, not in Go.
- You may change textkit's own files and add new ones.
- Leave everything built, so `go run ./cmd/textkit` works as it is when you finish.

You have my go-ahead for the whole job, so do not stop to ask me anything. When you are done, tell me in a few lines what you checked and anything that may still differ.

This machine only lets you write inside this folder and `$TMPDIR`. If a tool cannot write its cache or home folder, point it at `$TMPDIR` (for example `CARGO_HOME=$TMPDIR/cargo` or `GOCACHE=$TMPDIR/go-build`).
