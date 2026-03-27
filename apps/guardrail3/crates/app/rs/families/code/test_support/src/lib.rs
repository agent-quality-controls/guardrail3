use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct TempDir {
    path: PathBuf,
}

impl TempDir {
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

#[must_use]
pub fn temp_root(slug: &str) -> PathBuf {
    let unique = format!(
        "{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos())
    );
    let path = std::env::temp_dir().join(unique);
    create_dir_all(&path);
    path
}

#[must_use]
pub fn copy_tree(src: &Path) -> TempDir {
    let path = temp_root("copy-tree");
    copy_dir_recursive(src, &path);
    TempDir { path }
}

#[must_use]
pub fn read_path(path: &Path) -> String {
    guardrail3_shared_fs::read_file_err(path).unwrap_or_default()
}

#[must_use]
pub fn read_file(root: &Path, rel: &str) -> String {
    read_path(&root.join(rel))
}

#[must_use]
pub fn read_path_or_default(path: &Path) -> String {
    guardrail3_shared_fs::read_file(path).unwrap_or_default()
}

#[must_use]
pub fn read_file_or_default(root: &Path, rel: &str) -> String {
    read_path_or_default(&root.join(rel))
}

pub fn write_path(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        create_dir_all(parent);
    }
    assert!(
        guardrail3_shared_fs::write_file(path, content).is_ok(),
        "write file"
    );
}

pub fn write_file(root: &Path, rel: &str, content: &str) {
    write_path(&root.join(rel), content);
}

pub fn create_dir_all(path: &Path) {
    assert!(
        guardrail3_shared_fs::create_dir_all(path).is_ok(),
        "create dir"
    );
}

pub fn remove_dir_all(path: &Path) {
    assert!(
        guardrail3_shared_fs::remove_dir_all(path).is_ok(),
        "remove dir"
    );
}

#[must_use]
pub fn line_number(content: &str, needle: &str) -> usize {
    content
        .lines()
        .position(|line| line.contains(needle))
        .map_or(0, |index| index + 1)
}

fn copy_dir_recursive(src: &Path, dst: &Path) {
    for entry in guardrail3_shared_fs::list_dir(src) {
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if guardrail3_shared_fs::metadata(&src_path).is_some_and(|metadata| metadata.is_dir()) {
            create_dir_all(&dst_path);
            copy_dir_recursive(&src_path, &dst_path);
        } else {
            let content = read_path(&src_path);
            write_path(&dst_path, &content);
        }
    }
}
