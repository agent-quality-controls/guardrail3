use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static TEMP_ROOT_COUNTER: AtomicU64 = AtomicU64::new(0);

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
pub fn create_temp_dir(slug: &str) -> TempDir {
    TempDir {
        path: temp_root(slug),
    }
}

#[must_use]
pub fn copy_tree(src: &Path) -> TempDir {
    let temp_dir = create_temp_dir("copy-tree");
    copy_dir_recursive(src, temp_dir.path());
    temp_dir
}

#[must_use]
pub fn read_path(path: &Path) -> String {
    guardrail3_shared_fs::read_file_err(path).unwrap_or_default()
}

#[must_use]
pub fn read_file(root: &Path, rel: &str) -> String {
    read_path(&root.join(rel))
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

fn temp_root(slug: &str) -> PathBuf {
    let unique_counter = TEMP_ROOT_COUNTER.fetch_add(1, Ordering::Relaxed);
    let unique = format!(
        "{}-{}-{}-{}",
        slug,
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos()),
        unique_counter,
    );
    let path = std::env::temp_dir().join(unique);
    create_dir_all(&path);
    path
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
