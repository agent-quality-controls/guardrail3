use std::{fs::File, path::Path};

pub(crate) fn is_readable_file(path: &Path) -> bool {
    File::open(path).is_ok()
}

pub(crate) fn is_readable_directory(path: &Path) -> bool {
    path.read_dir().is_ok()
}
