pub(crate) fn read_to_string(path: &std::path::Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path)
}
