use std::fmt;

/// Error type for guardrail3-ts.toml parsing failures.
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
            Self::Toml(e) => write!(f, "invalid guardrail3-ts.toml: {e}"),
            Self::Io(e) => write!(f, "could not read guardrail3-ts.toml: {e}"),
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

/// Implement `From<$from_ty>` for `Error` by wrapping the value in `$variant`.
macro_rules! impl_from {
    ($from_ty:ty, $variant:ident) => {
        impl From<$from_ty> for Error {
            fn from(value: $from_ty) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_from!(toml::de::Error, Toml);
impl_from!(std::io::Error, Io);
