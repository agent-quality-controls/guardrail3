use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsTestIngestionError {
    SourceIngestionNotImplemented,
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

impl std::fmt::Display for G3RsTestIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceIngestionNotImplemented => f.write_str("test source ingestion is not implemented yet"),
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("test file-tree ingestion is not implemented yet")
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

impl std::error::Error for G3RsTestIngestionError {}
