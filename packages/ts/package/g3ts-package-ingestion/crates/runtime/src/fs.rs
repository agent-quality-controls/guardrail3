use std::path::Path;

/// Read a file's contents as a UTF-8 string.
///
/// # Errors
///
/// Returns the underlying I/O error from the filesystem read.
#[expect(
    clippy::disallowed_methods,
    reason = "this module is the package-ingestion filesystem facade; callers must use this single entry point instead of direct std::fs calls"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
