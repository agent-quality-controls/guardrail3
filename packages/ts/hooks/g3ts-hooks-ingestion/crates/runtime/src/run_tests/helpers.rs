use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) fn temp_root(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after unix epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("g3ts-hooks-ingestion-{test_name}-{unique}"));
    fs::create_dir_all(&path).expect("create temp fixture root directory");
    path
}

pub(super) fn write(root: &Path, rel_path: &str, content: &str) {
    let path = root.join(rel_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent directory for fixture file");
    }
    fs::write(path, content).expect("write fixture file content");
}

pub(super) fn git_init(root: &Path) {
    let status = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(root)
        .status()
        .expect("run git init for fixture repository");
    assert!(status.success(), "git init should succeed for fixture root");
}
