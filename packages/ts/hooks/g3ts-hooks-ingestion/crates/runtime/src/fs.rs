use std::ffi::OsStr;
use std::path::{Path, PathBuf};

#[must_use]
pub(crate) fn read_to_string(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
}

#[must_use]
pub(crate) fn executable(path: &Path) -> Option<bool> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt as _;
        let mode = std::fs::metadata(path).ok()?.permissions().mode();
        Some(mode & 0o111 != 0)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        None
    }
}

#[must_use]
pub(crate) fn direct_files(root: &Path) -> Vec<PathBuf> {
    let Ok(read_dir) = std::fs::read_dir(root) else {
        return Vec::new();
    };
    read_dir
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter(|path| {
            path.file_name()
                .and_then(OsStr::to_str)
                .is_some_and(|file_name| !file_name.contains('/'))
        })
        .collect()
}
