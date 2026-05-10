/// Centralized filesystem access for the ingestion runtime.
use std::path::Path;

/// Read a file's contents as a UTF-8 string.
#[expect(
    clippy::disallowed_methods,
    reason = "this function IS the centralized filesystem boundary that the disallowed_methods lint requires; it intentionally calls std::fs::read_to_string so the rest of the crate can route through this single seam"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
