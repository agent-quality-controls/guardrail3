//! Repo package-root adoption checks.

use std::path::{Path, PathBuf};

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::fs;

/// Reports package roots that have not been adopted by G3TS.
#[must_use]
pub(crate) fn check_repo(repo_root: &Path, inventory: bool) -> Vec<G3CheckResult> {
    let mut findings = Vec::new();
    for rel_path in package_root_candidates(repo_root) {
        let abs_path = repo_root.join(&rel_path);
        let display_path = display_rel_path(&rel_path);
        if abs_path.join("guardrail3-ts.toml").is_file() {
            if inventory {
                findings.push(
                    G3CheckResult::new(
                        "g3ts-repo/workspace-adoption-inventory".to_owned(),
                        G3Severity::Info,
                        "TypeScript package root is adopted".to_owned(),
                        format!("package root `{display_path}` is adopted."),
                        Some(display_path),
                        None,
                    )
                    .into_inventory(),
                );
            }
        } else {
            findings.push(G3CheckResult::new(
                "g3ts-repo/unadopted-workspace".to_owned(),
                G3Severity::Error,
                "TypeScript package root is not adopted".to_owned(),
                format!(
                    "`{display_path}` has package.json but no guardrail3-ts.toml. Run: g3ts init workspace --path {display_path}"
                ),
                Some(display_path),
                None,
            ));
        }
    }
    findings
}

/// Finds root, apps/*, and packages/* package roots under a repo.
fn package_root_candidates(repo_root: &Path) -> Vec<PathBuf> {
    let mut candidates = Vec::new();
    if repo_root.join("package.json").is_file() {
        candidates.push(PathBuf::new());
    }
    for parent in ["apps", "packages"] {
        for path in fs::read_dir_paths(&repo_root.join(parent)) {
            let Some(candidate) = package_root_child(parent, &path) else {
                continue;
            };
            candidates.push(candidate);
        }
    }
    candidates
}

/// Returns the app-relative package root when a child directory has package.json.
fn package_root_child(parent: &str, path: &Path) -> Option<PathBuf> {
    if !path.join("package.json").is_file() {
        return None;
    }
    path.file_name()
        .map(|name| PathBuf::from(parent).join(name))
}

/// Converts a repo-relative path into the CLI display form.
fn display_rel_path(path: &Path) -> String {
    if path.as_os_str().is_empty() {
        ".".to_owned()
    } else {
        path.to_string_lossy().replace('\\', "/")
    }
}
