use std::fmt;

/// Error type for rust-toolchain.toml parsing failures.
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
            Self::Toml(error) => write!(f, "invalid rust-toolchain.toml: {error}"),
            Self::Io(error) => write!(f, "could not read rust-toolchain.toml: {error}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Toml(error) => Some(error),
            Self::Io(error) => Some(error),
        }
    }
}

impl From<toml::de::Error> for Error {
    fn from(error: toml::de::Error) -> Self {
        Self::Toml(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
