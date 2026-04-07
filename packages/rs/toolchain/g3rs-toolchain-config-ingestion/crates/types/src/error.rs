use std::path::PathBuf;

/// Ingestion failure for toolchain config.
#[derive(Debug)]
pub enum G3RsToolchainConfigIngestionError {
    /// No `rust-toolchain.toml` found at the workspace root.
    ToolchainTomlNotFound,
    /// AST ingestion is planned but not implemented yet.
    AstIngestionNotImplemented,
    /// File-tree ingestion is planned but not implemented yet.
    FileTreeIngestionNotImplemented,
    /// The toolchain config exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// The underlying IO error message.
        reason: String,
    },
    /// The config content could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
}

impl std::fmt::Display for G3RsToolchainConfigIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ToolchainTomlNotFound => {
                f.write_str("no rust-toolchain.toml found at the workspace root")
            }
            Self::AstIngestionNotImplemented => {
                f.write_str("toolchain AST ingestion is not implemented yet")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("toolchain file-tree ingestion is not implemented yet")
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

impl std::error::Error for G3RsToolchainConfigIngestionError {}
