/// Filesystem boundary module for this crate.
///
/// All `std::fs` operations are centralized here so that the rest of the crate
/// does not scatter direct filesystem calls.
use crate::Error;

#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
/// Reads the `tsconfig.json` at `path` into a string through the centralized fs boundary.
pub(crate) fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, Error> {
    Ok(std::fs::read_to_string(path)?)
}
