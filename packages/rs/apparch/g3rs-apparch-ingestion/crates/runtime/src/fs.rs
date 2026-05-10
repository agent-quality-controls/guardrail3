//! Filesystem boundary module for the apparch ingestion runtime.
use std::path::Path;

/// Reads the file at `path` into a `String`, centralizing all direct `std::fs` access.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary for apparch ingestion"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
