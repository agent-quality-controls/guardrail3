//! Repo marker-pair checks for TypeScript adoption roots.

use std::path::Path;

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::fs;

/// Package metadata marker required next to a TypeScript guardrail marker.
const PACKAGE_JSON: &str = "package.json";
/// TypeScript guardrail marker file.
const GUARDRAIL3_TS_TOML: &str = "guardrail3-ts.toml";

/// Walks `repo_root` and reports any `guardrail3-ts.toml` without sibling
/// `package.json`.
#[must_use]
pub(crate) fn check_repo(repo_root: &Path) -> Vec<G3CheckResult> {
    let mut results = Vec::new();
    walk_marker_pair(repo_root, repo_root, &mut results);
    results.sort_by(|left, right| left.subject().cmp(right.subject()));
    results
}

/// Recursively scans one repo directory for incomplete TypeScript marker pairs.
fn walk_marker_pair(repo_root: &Path, dir: &Path, results: &mut Vec<G3CheckResult>) {
    let entries = fs::read_dir_paths(dir);
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
        if !path.is_dir() || repo_walk_should_skip_dir(name) {
            continue;
        }
        subdirs.push(path);
    }
    if has_g3ts_toml && !has_package_json {
        let subject = relative_to(repo_root, dir);
        results.push(G3CheckResult::new(
            "g3ts-topology/marker-pair-incomplete".to_owned(),
            G3Severity::Error,
            "TypeScript adoption marker pair incomplete".to_owned(),
            "guardrail3-ts.toml is present without sibling package.json.".to_owned(),
            Some(subject),
            None,
        ));
    }
    for sub in subdirs {
        walk_marker_pair(repo_root, sub.as_path(), results);
    }
}

/// Converts an absolute marker directory into repo-relative display text.
fn relative_to(base: &Path, target: &Path) -> String {
    target.strip_prefix(base).map_or_else(
        |_| target.to_string_lossy().replace('\\', "/"),
        |rel| {
            let value = rel.to_string_lossy().replace('\\', "/");
            if value.is_empty() {
                ".".to_owned()
            } else {
                value
            }
        },
    )
}

/// Returns true when the repo marker scan should not recurse into a directory.
pub(crate) fn repo_walk_should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".fixture3"
            | "behavior"
            | "node_modules"
            | "target"
            | ".git"
            | ".cargo-target"
            | "dist"
            | "build"
    )
}
