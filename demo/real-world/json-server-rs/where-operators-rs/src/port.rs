//! The Rust port of `src/where-operators.ts` (json-server, MIT).

#![forbid(unsafe_code)]

/// `WHERE_OPERATORS`, in the order `matchesWhere` checks them.
pub const WHERE_OPERATORS: [&str; 10] = [
    "lt",
    "lte",
    "gt",
    "gte",
    "eq",
    "ne",
    "in",
    "contains",
    "startsWith",
    "endsWith",
];

/// The operator named `value`, if it is one.
pub fn operator(value: &str) -> Option<&'static str> {
    WHERE_OPERATORS.iter().copied().find(|op| *op == value)
}

/// Port of `isWhereOperator` (`src/where-operators.ts:16`).
pub fn is_where_operator(value: String) -> Result<bool, String> {
    Ok(operator(&value).is_some())
}
