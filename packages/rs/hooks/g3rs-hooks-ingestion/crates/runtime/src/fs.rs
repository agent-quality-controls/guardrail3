use std::path::Path;

pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

pub(crate) fn metadata(path: &Path) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

pub(crate) fn path_exists(path: &Path) -> bool {
    metadata(path).is_ok()
}
