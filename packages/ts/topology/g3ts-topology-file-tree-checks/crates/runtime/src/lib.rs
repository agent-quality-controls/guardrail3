//! Runtime for the g3ts topology file-tree checks family.

/// Per-rule module: forbids nested adopted TS unit markers.
mod no_nested_guardrail3_ts_toml;
/// Top-level dispatch that runs every file-tree rule against an input.
mod run;
/// Internal formatting helpers shared by the file-tree rules.
mod support;

#[cfg(feature = "checks")]
pub use run::check;
