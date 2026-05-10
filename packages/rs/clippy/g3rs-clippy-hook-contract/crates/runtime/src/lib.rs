//! Clippy hook contract runtime crate.

/// Clippy hook contract definition.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
