//! Walks the repo and reports any directory with one half of the TS marker
//! pair but not the other. The unit pair is `package.json` paired with
//! `guardrail3-ts.toml`.

use std::path::Path;

use crate::fs as g3ts_fs;
use crate::topology::{GUARDRAIL3_TS_TOML, PACKAGE_JSON, repo_walk_should_skip_dir};

/// Walks `repo_root` and returns one finding line per directory that has
/// `guardrail3-ts.toml` without a sibling `package.json`.
pub(crate) fn check_marker_pair_completeness(repo_root: &Path) -> Vec<String> {
    let mut findings = Vec::new();
    walk_marker_pair(repo_root, repo_root, &mut findings);
    findings.sort();
    findings.dedup();
    findings
}

/// Depth-first walk of `dir` (under `repo_root`) that appends one finding to
/// `findings` for each directory with `guardrail3-ts.toml` but no sibling
/// `package.json`. Skips well-known generated/vendor directories.
fn walk_marker_pair(repo_root: &Path, dir: &Path, findings: &mut Vec<String>) {
    let entries = g3ts_fs::read_dir_paths(dir);
    if entries.is_empty() {
        return;
    }
    let mut subdirs = Vec::new();
    let mut has_package_json = false;
    let mut has_g3ts_toml = false;
    for path in entries {
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path.is_file() {
            if name == PACKAGE_JSON {
                has_package_json = true;
            } else if name == GUARDRAIL3_TS_TOML {
                has_g3ts_toml = true;
            }
            continue;
        }
        if !path.is_dir() {
            continue;
        }
        if repo_walk_should_skip_dir(name) {
            continue;
        }
        subdirs.push(path);
    }
    if has_g3ts_toml && !has_package_json {
        findings.push(format!(
            "g3ts-topology/marker-pair-incomplete {} guardrail3-ts.toml present without sibling package.json",
            relative_to(repo_root, dir)
        ));
    }
    for sub in subdirs {
        walk_marker_pair(repo_root, sub.as_path(), findings);
    }
}

/// Returns `target` as a forward-slash path relative to `base` when `target`
/// is inside `base`; otherwise returns the full `target` path with backslashes
/// normalized.
fn relative_to(base: &Path, target: &Path) -> String {
    target.strip_prefix(base).map_or_else(
        |_| target.to_string_lossy().replace('\\', "/"),
        |rel| rel.to_string_lossy().replace('\\', "/"),
    )
}
