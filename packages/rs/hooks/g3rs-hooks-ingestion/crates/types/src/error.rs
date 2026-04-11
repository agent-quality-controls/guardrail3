use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsHooksIngestionError {
    ConfigIngestionNotImplemented,
    FileTreeIngestionNotImplemented,
    Unreadable {
        path: PathBuf,
        reason: String,
    },
    ParseFailed {
        path: PathBuf,
        reason: String,
    },
}

impl std::fmt::Display for G3RsHooksIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigIngestionNotImplemented => {
                f.write_str("hooks config ingestion is not implemented")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("hooks file-tree ingestion is not implemented")
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

impl std::error::Error for G3RsHooksIngestionError {}
