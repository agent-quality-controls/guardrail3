use std::fmt;

/// Error type for rustfmt.toml parsing failures.
#[derive(Debug)]
pub enum Error {
    /// TOML deserialization failed.
    Toml(toml::de::Error),
    /// File I/O failed.
    Io(std::io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Toml(e) => write!(f, "invalid rustfmt.toml: {e}"),
            Self::Io(e) => write!(f, "could not read rustfmt.toml: {e}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Toml(e) => Some(e),
            Self::Io(e) => Some(e),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(e: toml::de::Error) -> Self {
        Self::Toml(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}
