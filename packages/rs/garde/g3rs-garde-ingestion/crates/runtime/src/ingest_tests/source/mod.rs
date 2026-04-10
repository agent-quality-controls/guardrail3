use std::fs;
use std::path::Path;
use std::process::Command;

mod coexistence;
mod failures;
mod gating;
mod selection;

use tempfile::tempdir;

fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directories");
    }
    fs::write(path, content).expect("write fixture file");
}

fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed")
}

#[cfg(unix)]
fn make_unreadable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("fixture file metadata should exist")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(path, permissions).expect("should make fixture file unreadable");
}

fn new_root() -> tempfile::TempDir {
    let temp = tempdir().expect("create tempdir");
    git_init(temp.path());
    temp
}
