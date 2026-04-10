use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsHooksRsIngestionError {
    ConfigIngestionNotImplemented,
    FileTreeIngestionNotImplemented,
    Unreadable {
        path: PathBuf,
        reason: String,
    },
}

impl std::fmt::Display for G3RsHooksRsIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConfigIngestionNotImplemented => {
                f.write_str("hooks-rs config ingestion is not implemented")
            }
            Self::FileTreeIngestionNotImplemented => {
                f.write_str("hooks-rs file-tree ingestion is not implemented")
            }
            Self::Unreadable { path, reason } => {
                write!(f, "cannot read {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsHooksRsIngestionError {}
