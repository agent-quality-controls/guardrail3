/// Filesystem boundary module for this crate.
///
/// All `std::fs` operations are centralized here so the parser does not scatter
/// direct filesystem calls.
#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary"
)]
pub fn read_to_string(path: impl AsRef<std::path::Path>) -> Result<String, crate::error::Error> {
    std::fs::read_to_string(path).map_err(|err| crate::error::Error::Io(err.to_string()))
}
