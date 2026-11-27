package main

import (
	"bufio"
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"example.com/textkit"
)

type call struct {
	Fn   string            `json:"fn"`
	Args []json.RawMessage `json:"args"`
}

func arg[T any](args []json.RawMessage, i int) (T, error) {
	var v T
	if i >= len(args) {
		return v, fmt.Errorf("missing argument %d", i+1)
	}
	err := json.Unmarshal(args[i], &v)
	return v, err
}

func dispatch(c call) (any, error) {
	switch c.Fn {
	case "Humanize":
		n, err := arg[int64](c.Args, 0)
		if err != nil {
			return nil, err
		}
		return textkit.Humanize(n), nil
	case "Abbrev":
		s, err := arg[string](c.Args, 0)
		if err != nil {
			return nil, err
		}
		max, err := arg[int](c.Args, 1)
		if err != nil {
			return nil, err
		}
		return textkit.Abbrev(s, max)
	case "ParseRange":
		spec, err := arg[string](c.Args, 0)
		if err != nil {
			return nil, err
		}
		return textkit.ParseRange(spec)
	case "Percent":
		part, err := arg[float64](c.Args, 0)
		if err != nil {
			return nil, err
		}
		total, err := arg[float64](c.Args, 1)
		if err != nil {
			return nil, err
		}
		places, err := arg[int](c.Args, 2)
		if err != nil {
			return nil, err
		}
		return textkit.Percent(part, total, places)
	}
	return nil, fmt.Errorf("unknown function %s", c.Fn)
}

func main() {
	in := bufio.NewScanner(os.Stdin)
	in.Buffer(make([]byte, 1<<20), 1<<24)
	out := bufio.NewWriter(os.Stdout)
	defer out.Flush()
	enc := json.NewEncoder(out)
	enc.SetEscapeHTML(false)
	for in.Scan() {
		line := in.Text()
		if strings.TrimSpace(line) == "" {
			continue
		}
		var c call
		if err := json.Unmarshal([]byte(line), &c); err != nil {
			fmt.Fprintln(os.Stderr, err)
			os.Exit(2)
		}
		result, err := dispatch(c)
		if err != nil {
			enc.Encode(map[string]string{"error": err.Error()})
		} else {
			enc.Encode(map[string]any{"ok": result})
		}
	}
}
