#![allow(
    clippy::multiple_crate_versions,
    reason = "transitive duplicate from tempfile dev-dep: toml depends on indexmap->hashbrown 0.17.1 while wasmparser (pulled in by getrandom/wasi backend chain) depends on hashbrown 0.15.5; same chain produces wit-bindgen 0.51 vs 0.57"
)]

use rustfmt_toml_parser_runtime as _;

#[cfg(feature = "checks")]
pub mod parser;
