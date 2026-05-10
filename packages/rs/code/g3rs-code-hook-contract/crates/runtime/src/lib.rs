//! Code hook contract runtime crate.

/// Code hook contract definition.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
