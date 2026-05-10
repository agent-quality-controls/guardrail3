//! Public hook-contract surface for g3ts style hooks.

/// Hook contract definition exported via `hook_contract`.
#[cfg(feature = "api")]
mod contract;

#[cfg(feature = "api")]
pub use contract::hook_contract;
