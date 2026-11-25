import re
import unicodedata


def slugify(text, max_len=40):
    """Lowercase ASCII slug: letters and digits joined by single hyphens, cut at max_len."""
    folded = unicodedata.normalize("NFKD", text).encode("ascii", "ignore").decode()
    words = re.findall(r"[a-z0-9]+", folded.lower())
    return "-".join(words)[:max_len].strip("-")


def count_words(text):
    return len(text.split())
