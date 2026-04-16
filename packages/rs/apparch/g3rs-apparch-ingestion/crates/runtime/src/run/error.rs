use std::path::PathBuf;

#[derive(Debug)]
pub enum G3RsApparchIngestionError {
    CargoTomlNotFound,
    Unreadable { path: PathBuf, reason: String },
    ParseFailed { path: PathBuf, reason: String },
    NormalizationFailed { path: PathBuf, reason: String },
}

impl std::fmt::Display for G3RsApparchIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CargoTomlNotFound => f.write_str("no Cargo.toml found at the workspace root"),
            Self::Unreadable { path, reason } => write!(f, "cannot read {}: {reason}", path.display()),
            Self::ParseFailed { path, reason } => write!(f, "cannot parse {}: {reason}", path.display()),
            Self::NormalizationFailed { path, reason } => {
                write!(f, "cannot normalize {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3RsApparchIngestionError {}
