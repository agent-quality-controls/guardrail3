//! Centralized filesystem access for the workspace crawl runtime.

#[allow(
    clippy::disallowed_types,
    reason = "fs.rs is the centralized fs boundary"
)]
use std::fs::File;
use std::path::Path;

/// Returns true when the path can be opened for reading.
#[allow(
    clippy::disallowed_types,
    reason = "fs.rs is the centralized fs boundary"
)]
pub(crate) fn is_readable_file(path: &Path) -> bool {
    File::open(path).is_ok()
}

/// Returns true when the directory entries can be enumerated.
pub(crate) fn is_readable_directory(path: &Path) -> bool {
    path.read_dir().is_ok()
}
