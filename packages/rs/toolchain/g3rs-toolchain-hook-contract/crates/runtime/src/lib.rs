//! Hook contract definition for the g3rs toolchain family.

/// Declarative description of the toolchain family's hook contract.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
