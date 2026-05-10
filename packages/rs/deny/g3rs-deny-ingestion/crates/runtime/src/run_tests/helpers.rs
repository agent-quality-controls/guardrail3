#![expect(
    clippy::disallowed_methods,
    reason = "test fixtures need direct filesystem and process access to build temp workspaces"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn git_init(path: &Path) {
    let _status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed in test fixture setup");
    // g3rs-workspace-crawl::crawl requires a Cargo.toml at the workspace root.
    // Provide a minimal one so test fixtures need only write the deny-specific files
    // they care about. Tests that exercise Cargo.toml content can overwrite it.
    fs::write(path.join("Cargo.toml"), "[workspace]\nmembers = []\n")
        .expect("should write minimal Cargo.toml in test fixture root");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent)
            .expect("should create parent directories for test fixture files");
    }
    fs::write(path, content).expect("should write test fixture file to disk");
}

pub(super) fn make_unreadable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("fixture file should exist before chmod")
        .permissions();
    permissions.set_mode(0o000);
    fs::set_permissions(path, permissions).expect("should chmod fixture file unreadable");
}

pub(super) fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed on valid test workspace")
}
