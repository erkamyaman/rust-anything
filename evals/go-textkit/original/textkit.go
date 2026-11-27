package textkit

import (
	"errors"
	"fmt"
	"sort"
	"strconv"
	"strings"
	"unicode"
)

func Humanize(n int64) string {
	neg := n < 0
	u := uint64(n)
	if neg {
		u = -u
	}
	digits := strconv.FormatUint(u, 10)
	var b strings.Builder
	head := len(digits) % 3
	if head > 0 {
		b.WriteString(digits[:head])
	}
	for i := head; i < len(digits); i += 3 {
		if b.Len() > 0 {
			b.WriteByte(',')
		}
		b.WriteString(digits[i : i+3])
	}
	if neg {
		return "-" + b.String()
	}
	return b.String()
}

func Abbrev(s string, max int) (string, error) {
	if max < 4 {
		return "", fmt.Errorf("abbrev: max must be at least 4, got %d", max)
	}
	runes := []rune(s)
	if len(runes) <= max {
		return s, nil
	}
	return strings.TrimRightFunc(string(runes[:max-3]), unicode.IsSpace) + "...", nil
}

func ParseRange(spec string) ([]int, error) {
	seen := map[int]bool{}
	out := []int{}
	for _, part := range strings.Split(spec, ",") {
		part = strings.TrimSpace(part)
		if part == "" {
			continue
		}
		lo, hi, isRange := strings.Cut(part, "-")
		start, err := strconv.Atoi(strings.TrimSpace(lo))
		if err != nil {
			return nil, fmt.Errorf("parse range %q: %w", part, err)
		}
		end := start
		if isRange {
			end, err = strconv.Atoi(strings.TrimSpace(hi))
			if err != nil {
				return nil, fmt.Errorf("parse range %q: %w", part, err)
			}
		}
		if end < start {
			return nil, fmt.Errorf("parse range %q: end before start", part)
		}
		if end-start > 10000 {
			return nil, errors.New("parse range: span too large")
		}
		for i := start; i <= end; i++ {
			if !seen[i] {
				seen[i] = true
				out = append(out, i)
			}
		}
	}
	sort.Ints(out)
	return out, nil
}

func Percent(part, total float64, places int) (string, error) {
	if total == 0 {
		return "", errors.New("percent: total is zero")
	}
	if places < 0 || places > 6 {
		return "", fmt.Errorf("percent: places %d out of range 0..6", places)
	}
	return strconv.FormatFloat(part/total*100, 'f', places, 64) + "%", nil
}
