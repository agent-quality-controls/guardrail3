use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;

use tempfile::{TempDir, tempdir};

pub(super) fn temp_workspace() -> TempDir {
    tempdir().expect("temp workspace should be created")
}

pub(super) fn write_file(workspace: &Path, rel_path: &str, content: &str) {
    write(workspace.join(rel_path), content);
}

pub(super) fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

pub(super) fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

pub(super) fn write_executable(path: impl AsRef<Path>, content: &str) {
    write(path.as_ref(), content);
    let mut permissions = fs::metadata(path.as_ref())
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path.as_ref(), permissions).expect("chmod should succeed");
}
