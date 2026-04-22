use std::path::PathBuf;

/// Ingestion failure for release.
#[derive(Debug)]
pub enum G3RsReleaseIngestionError {
    /// Repo-root release ingestion exists in API only for now.
    RepoRootChecksNotImplemented,
    /// No `Cargo.toml` found at the workspace root.
    CargoTomlNotFound,
    /// The crawl root is not a pointed workspace root.
    NormalizationFailed {
        /// Path related to the normalization failure.
        path: PathBuf,
        /// Human-readable explanation.
        reason: String,
    },
    /// A config file exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// The underlying IO error message.
        reason: String,
    },
    /// A config file could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
}

impl std::fmt::Display for G3RsReleaseIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RepoRootChecksNotImplemented => {
                f.write_str("repo-root release ingestion is not implemented yet")
            }
            Self::CargoTomlNotFound => f.write_str("no Cargo.toml found at the workspace root"),
            Self::NormalizationFailed { path, reason } => {
                write!(f, "cannot normalize {}: {reason}", path.display())
            }
            Self::Unreadable { path, reason } => {
                write!(f, "cannot read {}: {reason}", path.display())
            }
            Self::ParseFailed { path, reason } => {
                write!(f, "cannot parse {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsReleaseIngestionError {}
