//! Centralized filesystem access for the arch ingestion runtime.
//!
//! `disallowed_methods` bans direct `std::fs` calls across the workspace; this
//! module is the single permitted call site, so the bans are explicitly allowed
//! here with an inline reason citing that contract.
use std::path::Path;

/// Read a file's contents as a UTF-8 string.
///
/// # Errors
///
/// Returns the underlying I/O error from the filesystem read.
#[expect(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem facade; the disallowed-methods ban on std::fs::* exists to route all callers through this single entry point"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
