use std::path::Path;

#[allow(
    clippy::disallowed_methods,
    reason = "fs.rs IS the centralized fs boundary for the astro config parser"
)]
pub(crate) fn read_to_string(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
