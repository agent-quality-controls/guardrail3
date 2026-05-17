//! Test support utilities for the `g3rs-deny-config-checks` family.

/// Rule implementation for `input`.
mod input;

#[cfg(feature = "support")]
pub use input::{input, run, run_with_rust_policy};
