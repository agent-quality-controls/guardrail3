#[derive(Debug)]
pub enum Error {
    Io(String),
    Json(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(message) | Self::Json(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for Error {}
