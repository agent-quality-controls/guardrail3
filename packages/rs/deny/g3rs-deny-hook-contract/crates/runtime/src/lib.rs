//! Hook contract definition for the g3rs deny family.

/// Hook contract specification module.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
