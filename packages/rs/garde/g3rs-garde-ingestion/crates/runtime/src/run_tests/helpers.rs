use std::fs;
use std::path::Path;
use std::process::Command;

use tempfile::{TempDir, tempdir};

pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

pub(super) fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}

#[cfg(unix)]
pub(super) fn make_unreadable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("fixture file metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(path, permissions).expect("should make fixture file unreadable");
}

pub(super) fn new_root() -> TempDir {
    let temp = tempdir().expect("should create temporary directory for test workspace");
    git_init(temp.path());
    temp
}
