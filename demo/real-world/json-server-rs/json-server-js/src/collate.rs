//! `String.prototype.localeCompare` as Node runs it (ICU, `en-US`, default options), through
//! ICU4X's collator.

use std::cmp::Ordering;
use std::sync::OnceLock;

use icu_collator::options::CollatorOptions;
use icu_collator::{Collator, CollatorBorrowed};

fn collator() -> &'static CollatorBorrowed<'static> {
    static COLLATOR: OnceLock<CollatorBorrowed<'static>> = OnceLock::new();
    COLLATOR.get_or_init(|| {
        Collator::try_new(Default::default(), CollatorOptions::default())
            .expect("the compiled root collation data")
    })
}

/// `a.localeCompare(b)`: -1, 0 or 1.
pub fn locale_compare(a: &str, b: &str) -> f64 {
    match collator().compare(a, b) {
        Ordering::Less => -1.0,
        Ordering::Equal => 0.0,
        Ordering::Greater => 1.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn collation_orders_like_icu() {
        assert_eq!(locale_compare("a", "B"), -1.0);
        assert_eq!(locale_compare("a", "A"), -1.0);
        assert_eq!(locale_compare("é", "f"), -1.0);
        assert_eq!(locale_compare("Zebra", "apple"), 1.0);
        assert_eq!(locale_compare("x", "x"), 0.0);
    }
}
