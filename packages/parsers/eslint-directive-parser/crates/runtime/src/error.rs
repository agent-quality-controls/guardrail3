use std::fmt;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Parse(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "eslint directive parser I/O error: {err}"),
            Self::Parse(reason) => write!(f, "invalid ESLint directive input: {reason}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
