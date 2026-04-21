#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    Io(String),
    Json(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "failed to read package.json file: {msg}"),
            Self::Json(msg) => write!(f, "failed to parse package.json: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}
