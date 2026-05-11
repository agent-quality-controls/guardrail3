use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("run `git init --quiet` for test repository");
    assert!(status.success(), "git init must exit successfully");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

pub(super) fn crawl(root: &Path) -> g3_workspace_crawl::G3WorkspaceCrawl {
    g3_workspace_crawl::crawl(root).expect("crawl should succeed")
}

pub(super) fn long_readme(title: &str) -> String {
    format!("# {title}\n\n{}", "x".repeat(260))
}

#[cfg(unix)]
pub(super) fn make_unreadable(path: &Path) -> std::fs::Permissions {
    use std::os::unix::fs::PermissionsExt as _;

    let original = fs::metadata(path)
        .expect("read metadata before changing permissions")
        .permissions();
    let mut unreadable = original.clone();
    unreadable.set_mode(0o000);
    fs::set_permissions(path, unreadable).expect("make file unreadable");
    original
}

#[cfg(unix)]
pub(super) fn restore_permissions(path: &Path, permissions: std::fs::Permissions) {
    fs::set_permissions(path, permissions).expect("restore original permissions");
}
