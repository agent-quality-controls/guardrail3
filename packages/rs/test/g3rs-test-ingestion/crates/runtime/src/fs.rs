use std::path::Path;

/// Centralized filesystem access for source ingestion.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Centralized directory creation for ingestion fixtures.
#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn create_dir_all(path: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(path)
}

/// Centralized file write for ingestion fixtures.
#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn write(path: &Path, content: &str) -> std::io::Result<()> {
    std::fs::write(path, content)
}

/// Centralized metadata read for ingestion fixtures.
#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn metadata(path: &Path) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

/// Centralized permission updates for ingestion fixtures.
#[cfg(test)]
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn set_permissions(
    path: &Path,
    permissions: std::fs::Permissions,
) -> std::io::Result<()> {
    std::fs::set_permissions(path, permissions)
}
