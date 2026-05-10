//! Staged-file collection: reads `git diff --cached --name-only --diff-filter=ACM`.

use std::path::Path;
use std::process::{Command, Stdio};

/// Returns the list of staged files (Added, Copied, Modified) relative to the repo root.
///
/// Returns an empty list when git is unavailable or the call fails.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized staged-file reader"
)]
#[must_use]
pub fn read_staged_files(repo_root: &Path) -> Vec<String> {
    let output = Command::new("git")
        .args(["diff", "--cached", "--name-only", "--diff-filter=ACM"])
        .current_dir(repo_root)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();
    match output {
        Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout)
            .lines()
            .filter(|line| !line.is_empty())
            .map(str::to_owned)
            .collect(),
        _ => Vec::new(),
    }
}

/// Resolves the repository root via `git rev-parse --show-toplevel`. Falls back
/// to the given path if git is unavailable.
#[allow(
    clippy::disallowed_methods,
    reason = "this module IS the centralized git repo-root resolver"
)]
#[must_use]
pub fn resolve_repo_root(start: &Path) -> std::path::PathBuf {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .current_dir(start)
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output();
    match output {
        Ok(out) if out.status.success() => {
            let line = String::from_utf8_lossy(&out.stdout);
            let trimmed = line.trim_end_matches('\n').trim();
            if trimmed.is_empty() {
                start.to_path_buf()
            } else {
                std::path::PathBuf::from(trimmed)
            }
        }
        _ => start.to_path_buf(),
    }
}
