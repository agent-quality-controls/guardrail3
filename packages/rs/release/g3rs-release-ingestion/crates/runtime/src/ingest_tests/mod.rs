#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::wildcard_enum_match_arm,
    clippy::disallowed_methods,
    reason = "test code uses expect/panic for assertions and direct fs access for fixtures"
)]

use std::fs;
use std::path::Path;
use std::process::Command;

/// Covers the smallest happy-path and failure-path ingest cases.
mod basic;
/// Covers dependency-edge and publishability scenarios.
mod deps;
/// Covers end-to-end release ingestion wiring.
mod pipeline;

/// Creates a git repo so release ingestion can inspect VCS state.
fn git_init(path: &Path) {
    let _ = Command::new("git")
        .args(["init", "--quiet"])
        .current_dir(path)
        .status()
        .expect("git init should succeed");
}

/// Writes a fixture file, creating its parent directories first.
fn write(path: impl AsRef<Path>, content: &str) {
    if let Some(parent) = path.as_ref().parent() {
        fs::create_dir_all(parent).expect("create parent directory");
    }
    fs::write(path, content).expect("write fixture file");
}

/// Crawls a fixture tree into the shared workspace snapshot.
fn crawl(root: &Path) -> g3rs_workspace_crawl::G3RsWorkspaceCrawl {
    g3rs_workspace_crawl::crawl(root).expect("crawl should succeed")
}

/// Builds a README long enough to satisfy release README length checks.
fn long_readme(title: &str) -> String {
    format!("# {title}\n\n{}", "x".repeat(260))
}

#[cfg(unix)]
/// Makes a file unreadable and returns its original permissions for restoration.
fn make_unreadable(path: &Path) -> std::fs::Permissions {
    use std::os::unix::fs::PermissionsExt as _;

    let original = std::fs::metadata(path)
        .expect("read metadata before changing permissions")
        .permissions();
    let mut unreadable = original.clone();
    unreadable.set_mode(0o000);
    std::fs::set_permissions(path, unreadable).expect("make file unreadable");
    original
}

#[cfg(unix)]
/// Restores the permissions captured before a test made a file unreadable.
fn restore_permissions(path: &Path, permissions: std::fs::Permissions) {
    std::fs::set_permissions(path, permissions).expect("restore original permissions");
}
