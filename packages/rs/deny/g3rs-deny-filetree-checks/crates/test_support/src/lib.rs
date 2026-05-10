//! Test support helpers for the g3rs deny file-tree checks crate.

/// Builders that synthesize check inputs for the deny family's tests.
mod input;

#[cfg(feature = "support")]
pub use input::input;
