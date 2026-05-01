use crate::Error;

#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    Ok(std::fs::read_to_string(path)?)
}
