#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Error {
    NodeLaunch(String),
    Helper(String),
    Json(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NodeLaunch(msg) => write!(f, "failed to launch stylelint parser helper: {msg}"),
            Self::Helper(msg) => write!(f, "failed to evaluate stylelint config: {msg}"),
            Self::Json(msg) => write!(f, "invalid stylelint parser helper JSON: {msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::NodeLaunch(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err.to_string())
    }
}
