//! Hook contract definition for the `g3rs-arch` family.

/// Hook contract definition for the `g3rs-arch` family.
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
