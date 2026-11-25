from slug import count_words, slugify


def test_slugify():
    assert slugify("Hello, World!") == "hello-world"
    assert slugify("Crème brûlée  recipe") == "creme-brulee-recipe"
    assert slugify("a" * 50, 5) == "aaaaa"


def test_count_words():
    assert count_words("a b  c") == 3
