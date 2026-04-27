use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn write_fixture(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture");
    }
    fs::write(path, content).expect("write fixture file");
}

pub(super) fn git_init(path: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("run `git init --quiet` for test repository");
    assert!(status.success(), "git init must exit successfully");
}

pub(super) fn repo_root(temp_dir: &tempfile::TempDir) -> &Path {
    let root = temp_dir.path();
    git_init(root);
    root
}

pub(super) fn git_config_hooks_path(path: &Path, hooks_path: &str) {
    let status = Command::new("git")
        .args(["config", "core.hooksPath", hooks_path])
        .current_dir(path)
        .status()
        .expect("run `git config core.hooksPath` for test repository");
    assert!(
        status.success(),
        "git config core.hooksPath must exit successfully"
    );
}

pub(super) fn break_git_dir(path: &Path) {
    fs::rename(path.join(".git"), path.join(".git-real"))
        .expect("rename .git directory to break gitdir resolution");
    fs::write(path.join(".git"), "gitdir: /missing/hooks-test-gitdir\n")
        .expect("write broken gitdir pointer file");
}

#[cfg(unix)]
pub(super) fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt as _;

    let mut permissions = fs::metadata(path)
        .expect("read fixture metadata before setting executable bit")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("set executable bit on fixture");
}
