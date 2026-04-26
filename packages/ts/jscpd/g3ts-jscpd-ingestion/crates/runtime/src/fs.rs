use std::fs::File;
use std::path::Path;

pub(crate) fn is_readable_file(path: &Path) -> bool {
    File::open(path).is_ok()
}
