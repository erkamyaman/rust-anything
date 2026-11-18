//! The JavaScript semantics json-server's observable behavior depends on, so its Rust port can
//! answer byte for byte what the Node server answers.
//!
//! Values are `serde_json::Value` with `preserve_order`: insertion order is kept, and
//! [`value::ordered_keys`] gives the order JavaScript enumerates an object's keys in (array-index
//! keys first, ascending, then the rest in insertion order). Numbers are always compared and
//! printed as `f64`, as JavaScript has them.

#![forbid(unsafe_code)]

pub mod collate;
pub mod dotprop;
pub mod inflection;
mod inflection_rules;
pub mod json;
pub mod mime;
pub mod number;
pub mod sirv;
pub mod url;
pub mod value;
