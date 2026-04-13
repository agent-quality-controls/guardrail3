use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsTopologyIngestionError {
    RootManifestNotWorkspace { path: PathBuf },
    Unreadable { path: PathBuf, reason: String },
    ParseFailed { path: PathBuf, reason: String },
}

impl std::fmt::Display for G3RsTopologyIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RootManifestNotWorkspace { path } => {
                write!(f, "{} is not a workspace Cargo.toml", path.display())
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

impl std::error::Error for G3RsTopologyIngestionError {}
