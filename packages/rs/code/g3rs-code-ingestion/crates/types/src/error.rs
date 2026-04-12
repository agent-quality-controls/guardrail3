use std::path::PathBuf;

/// Ingestion failure for `code`.
#[derive(Debug)]
pub enum G3RsCodeIngestionError {
    /// A selected Rust source file exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// Underlying IO error or readability reason.
        reason: String,
    },
    /// A Cargo.toml needed for target classification could not be parsed.
    ParseFailed {
        /// Absolute path to the malformed manifest.
        path: PathBuf,
        /// Parser failure details.
        reason: String,
    },
}

impl std::fmt::Display for G3RsCodeIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreadable { path, reason } => {
                write!(f, "cannot read {}: {reason}", path.display())
            }
            Self::ParseFailed { path, reason } => {
                write!(f, "cannot parse {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsCodeIngestionError {}
