//! Assertions consumed by tests of the g3ts topology file-tree checks runtime.

/// Shared finding-construction and matching helpers used by per-rule modules.
mod common;

#[cfg(feature = "checks")]
pub mod no_nested_guardrail3_ts_toml;
#[cfg(feature = "checks")]
pub mod run;
