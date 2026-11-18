//! inflection 3.0.2 (MIT): `singularize` and `pluralize`.
//!
//! The rules are JS regexes with the `gi` flags. Without the `u` flag, `i` matches an ASCII
//! letter only against ASCII letters, while Rust's case-insensitive matching also folds `ſ` (long
//! s) to `s` and the Kelvin sign to `k`; those two are kept away from the rules.

use std::sync::OnceLock;

use regex::Regex;

use crate::inflection_rules::{PLURAL, SINGULAR, UNCOUNTABLE};

type Rules = Vec<(Regex, Option<&'static str>)>;

fn compile(rules: &'static [(&'static str, Option<&'static str>)]) -> Rules {
    rules
        .iter()
        .map(|(pattern, replacement)| {
            let re = Regex::new(&format!("(?i){pattern}")).expect("inflection rule");
            (re, *replacement)
        })
        .collect()
}

const FOLDING: [(char, char); 2] = [('\u{17F}', '\u{F8FE}'), ('\u{212A}', '\u{F8FF}')];

fn apply(word: &str, rules: &Rules) -> String {
    if UNCOUNTABLE.contains(&word.to_lowercase().as_str()) {
        return word.to_string();
    }
    let guarded: String = word
        .chars()
        .map(|c| {
            FOLDING
                .iter()
                .find(|(from, _)| *from == c)
                .map_or(c, |(_, to)| *to)
        })
        .collect();
    for (re, replacement) in rules {
        if re.is_match(&guarded) {
            let Some(replacement) = replacement else {
                return word.to_string();
            };
            let out = re.replace_all(&guarded, *replacement);
            return out
                .chars()
                .map(|c| {
                    FOLDING
                        .iter()
                        .find(|(_, to)| *to == c)
                        .map_or(c, |(from, _)| *from)
                })
                .collect();
        }
    }
    word.to_string()
}

/// `inflection.pluralize(word)`.
pub fn pluralize(word: &str) -> String {
    static RULES: OnceLock<Rules> = OnceLock::new();
    apply(word, RULES.get_or_init(|| compile(PLURAL)))
}

/// `inflection.singularize(word)`.
pub fn singularize(word: &str) -> String {
    static RULES: OnceLock<Rules> = OnceLock::new();
    apply(word, RULES.get_or_init(|| compile(SINGULAR)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn words_inflect_as_in_inflection() {
        assert_eq!(singularize("people"), "person");
        assert_eq!(pluralize("person"), "people");
        assert_eq!(singularize("posts"), "post");
        assert_eq!(singularize("statuses"), "status");
        assert_eq!(pluralize("status"), "statuses");
        assert_eq!(singularize("news"), "news");
        assert_eq!(pluralize("Hat"), "Hats");
        assert_eq!(singularize("comments"), "comment");
    }
}
