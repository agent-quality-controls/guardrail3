use std::path::PathBuf;

/// Ingestion failure for Cargo.
#[derive(Debug)]
pub enum G3RsCargoIngestionError {
    /// No `Cargo.toml` found at the workspace root.
    CargoTomlNotFound,
    /// source ingestion is not implemented yet.
    SourceIngestionNotImplemented,
    /// File-tree ingestion is planned but not implemented yet.
    FileTreeIngestionNotImplemented,
    /// The `Cargo.toml` exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// The underlying IO error message.
        reason: String,
    },
    /// The `Cargo.toml` content could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
    /// The discovered Cargo input surface was inconsistent or semantically invalid.
    NormalizationFailed {
        /// Absolute path to the input responsible for the normalization failure.
        path: PathBuf,
        /// The normalization failure reason.
        reason: String,
    },
}

impl std::fmt::Display for G3RsCargoIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CargoTomlNotFound => {
                f.write_str("no Cargo.toml found at the workspace root")
            }
            Self::SourceIngestionNotImplemented => {
                f.write_str("Cargo source ingestion is not implemented yet")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("Cargo file-tree ingestion is not implemented yet")
            }
            Self::Unreadable { path, reason } => {
                write!(f, "cannot read {}: {reason}", path.display())
            }
            Self::ParseFailed { path, reason } => {
                write!(f, "cannot parse {}: {reason}", path.display())
            }
            Self::NormalizationFailed { path, reason } => {
                write!(f, "cannot normalize {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsCargoIngestionError {}
