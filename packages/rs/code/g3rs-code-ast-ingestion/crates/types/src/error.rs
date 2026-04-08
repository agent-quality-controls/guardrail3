use std::path::PathBuf;

/// Ingestion failure for `code` AST input.
#[derive(Debug)]
pub enum G3RsCodeAstIngestionError {
    /// A selected Rust source file exists but cannot be read.
    Unreadable {
        /// Absolute path to the unreadable file.
        path: PathBuf,
        /// Underlying IO error or readability reason.
        reason: String,
    },
}

impl std::fmt::Display for G3RsCodeAstIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unreadable { path, reason } => {
                write!(f, "cannot read {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsCodeAstIngestionError {}
