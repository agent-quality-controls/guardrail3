//! Garde-family hook contract runtime: declares the family hook contract.

/// Garde-family hook contract definition.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
