use std::path::PathBuf;

/// Ingestion failure for deny config.
#[derive(Debug)]
pub enum G3RsDenyConfigIngestionError {
    /// No `deny.toml` or `.deny.toml` found at the workspace root.
    DenyTomlNotFound,
    /// The deny config file exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// The underlying IO error message.
        reason: String,
    },
    /// The deny config file could not be parsed.
    ParseFailed {
        /// Absolute path to the file that failed to parse.
        path: PathBuf,
        /// The underlying parse error message.
        reason: String,
    },
}

impl std::fmt::Display for G3RsDenyConfigIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DenyTomlNotFound => {
                f.write_str("no deny.toml or .deny.toml found at the workspace root")
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

impl std::error::Error for G3RsDenyConfigIngestionError {}
