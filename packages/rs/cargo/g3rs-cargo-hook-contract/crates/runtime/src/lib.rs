//! Cargo hook contract runtime crate.

/// Cargo hook contract definition.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
