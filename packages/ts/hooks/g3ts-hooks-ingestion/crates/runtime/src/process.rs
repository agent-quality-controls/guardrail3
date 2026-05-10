#![expect(
    clippy::disallowed_methods,
    reason = "centralized process module: this is the single approved boundary for std::process::Command calls in this crate to read git config; no shell pipeline involved"
)]

use std::path::{Path, PathBuf};

#[must_use]
/// Internal function `read_hooks_path`.
pub(crate) fn read_hooks_path(root: &Path) -> Option<String> {
    let output = std::process::Command::new("git")
        .args(["config", "--get", "core.hooksPath"])
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

#[must_use]
/// Internal function `git_root`.
pub(crate) fn git_root(root: &Path) -> Option<PathBuf> {
    let output = std::process::Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(root)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| PathBuf::from(String::from_utf8_lossy(&output.stdout).trim().to_owned()))
}
