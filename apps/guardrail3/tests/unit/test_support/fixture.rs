use std::path::Path;

/// Copy a fixture directory into a fresh tempdir.
/// `fixture_rel` is relative to the crate root, e.g. "tests/fixtures/r_arch_01/golden".
pub fn copy_golden(fixture_rel: &str) -> tempfile::TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    copy_dir_recursive(Path::new(fixture_rel), tmp.path());
    tmp
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in std::fs::read_dir(src).expect("read fixture dir") {
        let entry = entry.expect("read entry");
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            std::fs::create_dir_all(&dst_path).expect("create dir");
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let _bytes = std::fs::copy(&src_path, &dst_path).expect("copy file");
        }
    }
}
