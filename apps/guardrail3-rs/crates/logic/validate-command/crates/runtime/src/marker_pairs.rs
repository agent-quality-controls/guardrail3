//! Repo-wide marker-pair completeness check for adopted Rust workspaces.
//!
//! Rust adoption pair: a directory must have BOTH `Cargo.toml` (with `[workspace]`)
//! AND `guardrail3-rs.toml`. Half-adopted dirs are rejected at repo level.

use std::path::{Path, PathBuf};

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::fs as local_fs;

/// Reports half-adopted directories: only one of (Cargo.toml with [workspace],
/// guardrail3-rs.toml) is present.
#[must_use]
pub fn check_repo(repo_root: &Path) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    let mut visited = Vec::new();
    walk(repo_root, repo_root, &mut visited);
    for dir in visited {
        let cargo_toml = dir.join("Cargo.toml");
        let g3rs_toml = dir.join("guardrail3-rs.toml");
        let has_workspace_cargo = is_workspace_cargo(&cargo_toml);
        let has_g3rs_toml = local_fs::is_file(&g3rs_toml);

        if has_g3rs_toml && !has_workspace_cargo {
            let rel = relative(repo_root, &dir);
            results.push(G3CheckResult::new(
                "g3rs-topology/marker-pair-incomplete".to_owned(),
                G3Severity::Error,
                "incomplete adoption marker pair".to_owned(),
                format!("{rel} has guardrail3-rs.toml but no Cargo.toml with [workspace]"),
                Some(format!("{rel}/guardrail3-rs.toml")),
                None,
            ));
        }
        if has_workspace_cargo && !has_g3rs_toml {
            let rel = relative(repo_root, &dir);
            results.push(G3CheckResult::new(
                "g3rs-topology/marker-pair-incomplete".to_owned(),
                G3Severity::Error,
                "incomplete adoption marker pair".to_owned(),
                format!("{rel} has Cargo.toml with [workspace] but no guardrail3-rs.toml"),
                Some(format!("{rel}/Cargo.toml")),
                None,
            ));
        }
    }
    results
}

/// Recursively collects directories under `dir` into `out`, skipping hidden and
/// build-output directories.
fn walk(repo_root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
    out.push(dir.to_path_buf());
    for path in local_fs::read_dir_paths(dir) {
        if !local_fs::is_dir(&path) {
            continue;
        }
        if is_behavior_fixture_path(repo_root, &path) {
            continue;
        }
        if is_legacy_archive_path(repo_root, &path) {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if matches!(
            name,
            ".git" | "target" | "node_modules" | ".cargo-target" | ".cache"
        ) || name.starts_with('.')
        {
            continue;
        }
        walk(repo_root, &path, out);
    }
}

/// Returns true for guardrail replay fixture paths that must not participate in
/// repo-wide adoption checks.
fn is_behavior_fixture_path(repo_root: &Path, path: &Path) -> bool {
    path.strip_prefix(repo_root)
        .ok()
        .and_then(Path::to_str)
        .is_some_and(|rel| rel == "behavior/fixtures" || rel.starts_with("behavior/fixtures/"))
}

/// Returns true for archived legacy code that is intentionally outside active
/// repo-wide marker-pair adoption checks.
fn is_legacy_archive_path(repo_root: &Path, path: &Path) -> bool {
    path.strip_prefix(repo_root)
        .ok()
        .and_then(Path::to_str)
        .is_some_and(|rel| rel == "legacy" || rel.starts_with("legacy/"))
}

/// Returns true when `path` exists and its content contains a `[workspace]`
/// section.
fn is_workspace_cargo(path: &Path) -> bool {
    if !local_fs::is_file(path) {
        return false;
    }
    let Ok(content) = local_fs::read_to_string(path) else {
        return false;
    };
    content.contains("[workspace]")
}

/// Returns the path relative to `root` as a display string, or `.` when the
/// path equals the root.
fn relative(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .ok()
        .and_then(|p| p.to_str())
        .map_or_else(
            || path.to_string_lossy().into_owned(),
            |s| {
                if s.is_empty() {
                    ".".to_owned()
                } else {
                    s.to_owned()
                }
            },
        )
}

#[cfg(test)]
#[path = "marker_pairs_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod marker_pairs_tests;
