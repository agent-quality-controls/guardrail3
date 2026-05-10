//! Centralized filesystem boundary for the CLI runtime.
//!
//! Every other module in this crate must route filesystem reads through
//! these helpers instead of calling `std::fs::*` directly. This keeps the
//! filesystem touchpoint in one auditable place.

use std::path::{Path, PathBuf};

/// Lists the immediate child paths of `dir`. Returns an empty vector when
/// the directory cannot be read (missing, permission denied, etc.) so that
/// callers can treat unreadable directories as "no entries" without
/// duplicating error handling.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem boundary for the CLI runtime"
)]
pub(crate) fn read_dir_paths(dir: &Path) -> Vec<PathBuf> {
    let Ok(iter) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    iter.flatten().map(|entry| entry.path()).collect()
}
