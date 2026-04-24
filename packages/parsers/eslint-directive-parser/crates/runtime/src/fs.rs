use std::path::Path;

#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs is the centralized filesystem boundary for this parser"
)]
pub(crate) fn read_to_string(path: impl AsRef<Path>) -> Result<String, crate::error::Error> {
    std::fs::read_to_string(path).map_err(crate::error::Error::Io)
}
