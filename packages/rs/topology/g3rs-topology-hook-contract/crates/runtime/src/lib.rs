//! Hook contract definition for the g3rs topology family.

/// Declarative description of the topology family's hook contract.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
