#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    Io(String),
    Jsonc(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "failed to read tsconfig file: {msg}"),
            Self::Jsonc(msg) => write!(f, "failed to parse tsconfig JSONC: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
