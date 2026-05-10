/// Filesystem boundary module for this crate.
///
/// All `std::fs` operations are centralized here so that the rest of the crate
/// does not scatter direct filesystem calls.
use crate::Error;

/// Reads the file at `path` into a String, surfacing IO failures as [`Error::Io`].
///
/// # Errors
/// Returns [`Error::Io`] when the underlying `std::fs::read_to_string` call fails.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub(crate) fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    Ok(std::fs::read_to_string(path)?)
}
