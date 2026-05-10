/// `read_to_string` function.
#[expect(
    clippy::disallowed_methods,
    reason = "this module is the single sanctioned wrapper over std::fs::read_to_string; downstream code calls this wrapper instead."
)]
pub(crate) fn read_to_string(path: &std::path::Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
