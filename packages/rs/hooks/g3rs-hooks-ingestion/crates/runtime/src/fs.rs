#![expect(
    clippy::disallowed_methods,
    reason = "this module is the single sanctioned filesystem wrapper; downstream code calls these wrappers instead of std::fs."
)]

use std::path::{Path, PathBuf};

/// `read_to_string` function.
pub(crate) fn read_to_string(path: &Path) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// `metadata` function.
pub(crate) fn metadata(path: &Path) -> std::io::Result<std::fs::Metadata> {
    std::fs::metadata(path)
}

/// `path_exists` function.
pub(crate) fn path_exists(path: &Path) -> bool {
    metadata(path).is_ok()
}

/// Return absolute paths of direct child files of `dir`, sorted, ignoring
/// unreadable entries. Empty vector if the directory cannot be read.
pub(crate) fn list_direct_files(dir: &Path) -> Vec<PathBuf> {
    let Ok(read) = std::fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut paths = Vec::new();
    for entry in read.flatten() {
        let Ok(metadata) = entry.metadata() else {
            continue;
        };
        if !metadata.is_file() {
            continue;
        }
        paths.push(entry.path());
    }
    paths.sort();
    paths
}
