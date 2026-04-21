pub(super) fn write(root: &std::path::Path, rel_path: &str, content: &str) {
    let path = root.join(rel_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create temporary test directory");
    }
    std::fs::write(path, content).expect("write temporary test file");
}
