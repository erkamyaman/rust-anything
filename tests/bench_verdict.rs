//! The `bench` verdict for a unit with one function: its advice says "this function".

use rust_anything::harness::{BenchRow, verdict};
use rust_anything::lang::Lang;

fn row(name: &str, speedup: f64) -> BenchRow {
    BenchRow {
        name: name.into(),
        cases: 1,
        original_ns: speedup,
        rust_ns: 1.0,
        speedup,
        failing: false,
    }
}

#[test]
fn advice_for_one_function_says_this_function() {
    let v = verdict(&[row("twice", 0.38)], Lang::Ts);
    assert!(
        v.advice.contains("this function together with its callers"),
        "{}",
        v.advice
    );
    assert!(!v.advice.contains("these functions"), "{}", v.advice);
}
