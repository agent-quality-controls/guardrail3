/// Filesystem boundary module for this crate.
///
/// All `std::fs` operations are centralized here so that the rest of the crate
/// does not scatter direct filesystem calls.
use crate::Error;

/// Read a file to a string, returning the crate's [`Error`] type on failure.
#[allow(clippy::disallowed_methods)] // reason: this IS the centralized fs boundary module for this crate
#[allow(clippy::redundant_pub_crate)] // reason: pub(crate) is intentional — this is internal-only API
pub(crate) fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    Ok(std::fs::read_to_string(path)?)
}
