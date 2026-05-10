//! Error type returned by the ingestion runtime.

use std::path::PathBuf;

/// Failure modes that can be returned by topology ingestion.
#[derive(Debug)]
pub enum G3TsTopologyIngestionError {
    /// The configured unit root does not exist as a directory.
    UnitRootMissing {
        /// Path that was probed.
        path: PathBuf,
    },
    /// The unit root exists but is missing one of the adoption-pair markers.
    UnitRootNotAdopted {
        /// Path of the candidate unit root.
        path: PathBuf,
        /// Human-readable explanation of the missing marker.
        reason: String,
    },
    /// The unit root or one of its files could not be read.
    Unreadable {
        /// Path that failed to read.
        path: PathBuf,
        /// Underlying I/O error description.
        reason: String,
    },
}

impl std::fmt::Display for G3TsTopologyIngestionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnitRootMissing { path } => {
                write!(f, "unit root `{}` does not exist", path.display())
            }
            Self::UnitRootNotAdopted { path, reason } => write!(
                f,
                "unit root `{}` is not an adopted TS unit: {reason}",
                path.display()
            ),
            Self::Unreadable { path, reason } => {
                write!(f, "unable to read `{}`: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for G3TsTopologyIngestionError {}
