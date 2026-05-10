//! Runtime rules for the `g3rs-rust-family-contracts` family.

/// Rule implementation for `contract`.
mod contract;

#[cfg(feature = "api")]
pub use contract::{RustFamily, family_hook_contract};
