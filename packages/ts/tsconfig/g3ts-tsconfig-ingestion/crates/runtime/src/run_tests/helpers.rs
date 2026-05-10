use std::fs;
use std::path::Path;

/// Write `contents` to `root.join(rel_path)`, creating any missing parent
/// directories. Used by ingestion runtime tests.
///
/// # Panics
///
/// Panics if the parent directory cannot be created or the file cannot be
/// written.
#[expect(
    clippy::disallowed_methods,
    reason = "Test fixture helper writes to a tempdir; routing through the production fs port \
              would require the test sidecar to call a sibling module, which is forbidden by \
              the runtime-assertions-split rule"
)]
pub(super) fn write(root: &Path, rel_path: &str, contents: &str) {
    let abs_path = root.join(rel_path);
    if let Some(parent) = abs_path.parent() {
        fs::create_dir_all(parent).expect("parent directories should be created");
    }
    fs::write(abs_path, contents).expect("fixture file should be written");
}
