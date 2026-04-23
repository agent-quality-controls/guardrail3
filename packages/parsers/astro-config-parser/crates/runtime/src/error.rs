#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    Io(String),
    Parse(String),
    Json(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(msg) => write!(f, "failed to read Astro config file: {msg}"),
            Self::Parse(msg) => write!(f, "failed to parse Astro config source: {msg}"),
            Self::Json(msg) => write!(f, "failed to normalize Astro config payload: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}
