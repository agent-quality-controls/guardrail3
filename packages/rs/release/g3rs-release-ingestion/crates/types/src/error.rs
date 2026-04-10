use std::path::PathBuf;

/// Ingestion failure for release.
#[derive(Debug)]
pub enum G3RsReleaseIngestionError {
    /// No `Cargo.toml` found at the workspace root.
    CargoTomlNotFound,
    /// source ingestion is not implemented yet.
    SourceIngestionNotImplemented,
    /// File-tree ingestion is planned but not implemented yet.
    FileTreeIngestionNotImplemented,
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
            Self::CargoTomlNotFound => {
                f.write_str("no Cargo.toml found at the workspace root")
            }
            Self::SourceIngestionNotImplemented => {
                f.write_str("release source ingestion is not implemented yet")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("release file-tree ingestion is not implemented yet")
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
