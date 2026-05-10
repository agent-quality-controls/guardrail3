/// Read a UTF-8 file at `path`. Centralized fs entry point.
///
/// # Errors
///
/// Returns an `std::io::Error` when the file cannot be read.
#[expect(
    clippy::disallowed_methods,
    reason = "this module IS the centralized filesystem access boundary"
)]
pub(crate) fn read_to_string(path: &std::path::Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
