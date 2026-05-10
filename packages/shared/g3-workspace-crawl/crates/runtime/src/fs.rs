//! Filesystem boundary module for the workspace crawler.
//!
//! All `std::fs` operations are centralized here so that the rest of the
//! crate does not scatter direct filesystem calls.

use std::path::Path;

/// Returns `true` when `path` resolves to a file the current process can open.
#[allow(
    clippy::disallowed_types,
    reason = "fs.rs IS the centralized fs boundary for the crawl runtime"
)]
pub(crate) fn is_readable_file(path: &Path) -> bool {
    std::fs::File::open(path).is_ok()
}

/// Returns `true` when `path` resolves to a directory the current process can list.
pub(crate) fn is_readable_directory(path: &Path) -> bool {
    path.read_dir().is_ok()
}
