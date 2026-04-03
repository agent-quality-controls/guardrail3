/// Errors that can occur when parsing a mutants.toml file.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    /// The TOML content could not be parsed.
    Toml(String),
    /// The file could not be read from disk.
    Io(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Toml(msg) => write!(f, "invalid mutants.toml: {msg}"),
            Self::Io(msg) => write!(f, "failed to read mutants.toml: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
