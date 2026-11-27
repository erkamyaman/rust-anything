// go run ./cmd/rust-anything-probe <held-out jsonl>, copied into the workspace by score.py.
// Counts, per function, the held-out calls during which the process made a cgo call.
package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"runtime"

	"example.com/textkit"
)

type call struct {
	Fn   string            `json:"fn"`
	Args []json.RawMessage `json:"args"`
}

func arg[T any](args []json.RawMessage, i int) T {
	var v T
	if i < len(args) {
		_ = json.Unmarshal(args[i], &v)
	}
	return v
}

func invoke(c call) {
	switch c.Fn {
	case "Humanize":
		textkit.Humanize(arg[int64](c.Args, 0))
	case "Abbrev":
		textkit.Abbrev(arg[string](c.Args, 0), arg[int](c.Args, 1))
	case "ParseRange":
		textkit.ParseRange(arg[string](c.Args, 0))
	case "Percent":
		textkit.Percent(arg[float64](c.Args, 0), arg[float64](c.Args, 1), arg[int](c.Args, 2))
	}
}

type row struct {
	Calls  int `json:"calls"`
	Native int `json:"native"`
}

func main() {
	f, err := os.Open(os.Args[1])
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
		os.Exit(2)
	}
	functions := map[string]*row{}
	in := bufio.NewScanner(f)
	in.Buffer(make([]byte, 1<<20), 1<<24)
	for in.Scan() {
		var c call
		if json.Unmarshal(in.Bytes(), &c) != nil {
			continue
		}
		r := functions[c.Fn]
		if r == nil {
			r = &row{}
			functions[c.Fn] = r
		}
		r.Calls++
		before := runtime.NumCgoCall()
		invoke(c)
		if runtime.NumCgoCall() > before {
			r.Native++
		}
	}
	json.NewEncoder(os.Stdout).Encode(map[string]any{"functions": functions})
}
