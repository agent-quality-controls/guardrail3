//! Hook contract for the g3rs test family.

/// Hook contract definition.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
