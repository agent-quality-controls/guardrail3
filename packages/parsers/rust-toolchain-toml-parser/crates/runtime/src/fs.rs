use crate::Error;

/// Read a file through the parser crate's centralized filesystem boundary.
#[allow(
    clippy::disallowed_methods,
    reason = "this IS the centralized fs boundary module for this crate"
)]
pub(crate) fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    Ok(std::fs::read_to_string(path)?)
}
