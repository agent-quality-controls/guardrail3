use std::path::Path;

/// Centralized filesystem access for source ingestion.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}
