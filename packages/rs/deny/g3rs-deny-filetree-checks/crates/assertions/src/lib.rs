//! Reusable assertion helpers for the g3rs deny file-tree checks crate.

use g3rs_deny_filetree_checks_runtime as _;

/// Shared helpers for matching emitted findings against expected snapshots.
mod common;

#[cfg(feature = "checks")]
pub mod coverage;
#[cfg(feature = "checks")]
pub mod run;
#[cfg(feature = "checks")]
pub mod shadowing;
