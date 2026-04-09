use std::path::PathBuf;

/// Ingestion failure for deps.
#[derive(Debug)]
pub enum G3RsDepsIngestionError {
    /// No `Cargo.toml` found at the workspace root.
    CargoTomlNotFound,
    /// No `guardrail3-rs.toml` found at the workspace root.
    Guardrail3RsTomlNotFound,
    /// AST ingestion is planned but not implemented yet.
    AstIngestionNotImplemented,
    /// File-tree ingestion is planned but not implemented yet.
    FileTreeIngestionNotImplemented,
    /// A required file exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// The underlying IO error message.
        reason: String,
    },
    /// A required file content could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
    /// Dependency normalization failed after parsing succeeded.
    NormalizationFailed {
        /// Absolute path to the file whose data could not be normalized.
        path: PathBuf,
        /// The normalization failure message.
        reason: String,
    },
}

impl std::fmt::Display for G3RsDepsIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CargoTomlNotFound => f.write_str("no Cargo.toml found at the workspace root"),
            Self::Guardrail3RsTomlNotFound => {
                f.write_str("no guardrail3-rs.toml found at the workspace root")
            }
            Self::AstIngestionNotImplemented => {
                f.write_str("deps AST ingestion is not implemented yet")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("deps file-tree ingestion is not implemented yet")
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

impl std::error::Error for G3RsDepsIngestionError {}
