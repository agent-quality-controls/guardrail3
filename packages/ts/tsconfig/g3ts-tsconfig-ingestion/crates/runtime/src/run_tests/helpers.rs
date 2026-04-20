use std::fs;
use std::path::Path;

pub(super) fn write(root: &Path, rel_path: &str, contents: &str) {
    let abs_path = root.join(rel_path);
    if let Some(parent) = abs_path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(abs_path, contents).expect("fixture file should be written");
}
