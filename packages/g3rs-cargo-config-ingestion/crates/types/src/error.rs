use std::path::PathBuf;

/// Ingestion failure for Cargo config.
#[derive(Debug)]
pub enum G3RsCargoConfigIngestionError {
    /// No `Cargo.toml` found at the workspace root.
    CargoTomlNotFound,
    /// The `Cargo.toml` exists but cannot be read.
    Unreadable(PathBuf),
    /// The `Cargo.toml` content could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
}
