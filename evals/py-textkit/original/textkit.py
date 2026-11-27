SMALL_WORDS = {"a", "an", "and", "of", "the", "to", "in"}


def title_case(text):
    words = text.split()
    out = []
    for i, word in enumerate(words):
        lower = word.lower()
        if 0 < i < len(words) - 1 and lower in SMALL_WORDS:
            out.append(lower)
        else:
            out.append(word[:1].upper() + word[1:].lower())
    return " ".join(out)


def format_number(value, places=2, sep=","):
    if isinstance(value, bool) or not isinstance(value, (int, float)):
        raise TypeError(f"format_number: expected a number, got {type(value).__name__}")
    if not 0 <= places <= 20:
        raise ValueError(f"format_number: places must be between 0 and 20, got {places}")
    if isinstance(value, int) and places == 0:
        digits = f"{abs(value):,}"
    else:
        digits = f"{abs(value):,.{places}f}"
    if sep != ",":
        digits = digits.replace(",", sep)
    return ("-" if value < 0 else "") + digits


def parse_kv(text):
    result = {}
    for lineno, raw in enumerate(text.splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            raise ValueError(f"line {lineno}: expected key=value")
        key, _, value = line.partition("=")
        key = key.strip()
        if not key.isidentifier():
            raise KeyError(f"line {lineno}: bad key {key!r}")
        value = value.strip()
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
            value = value[1:-1]
        result[key] = value
    return result


def wrap(text, width):
    if width < 1:
        raise ValueError("wrap: width must be at least 1")
    lines = []
    current = ""
    for word in text.split():
        while len(word) > width:
            if current:
                lines.append(current)
                current = ""
            lines.append(word[:width])
            word = word[width:]
        if not current:
            current = word
        elif len(current) + 1 + len(word) <= width:
            current += " " + word
        else:
            lines.append(current)
            current = word
    if current:
        lines.append(current)
    return lines
