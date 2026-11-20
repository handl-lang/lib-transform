//! `transform` — Handl stdlib port (`src/index.hnl`).
//!
//! `map.hnl`/`filter.hnl`/`reduce.hnl` are the pure-handl arrows;
//! `ops.rs` holds the native `Op` impls backing the `transform.*` ops
//! (moved verbatim from the retired `kernel/crates/combinators`
//! floor-lib crate).

pub mod ops;

pub use ops::{TRANSFORM_OPS, transform_op_registry};

pub const TRANSFORM_HNL: &str = include_str!("index.hnl");
pub const FILTER_HNL: &str = include_str!("filter.hnl");
pub const MAP_HNL: &str = include_str!("map.hnl");
pub const REDUCE_HNL: &str = include_str!("reduce.hnl");
