//! Shared hex arch structural helpers used by both RS-ARCH-01 and TS-ARCH-01.
//!
//! These are language-agnostic utilities for checking directory structure.
//! Language-specific concerns (app discovery, leaf validation, recursion markers)
//! stay in the respective rs/ts modules.

use std::path::Path;

use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::FileSystem;

/// List subdirectory names in a directory.
pub fn list_dir_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    let entries = fs.list_dir(dir);
    let mut names = Vec::new();
    for entry in &entries {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if ft.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names
}

/// List file names (non-directories) in a directory.
pub fn list_file_names(fs: &dyn FileSystem, dir: &Path) -> Vec<String> {
    let entries = fs.list_dir(dir);
    let mut names = Vec::new();
    for entry in &entries {
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names
}

/// Check if a directory contains a `.gitkeep` file.
pub fn has_gitkeep(fs: &dyn FileSystem, dir: &Path) -> bool {
    fs.read_file(&dir.join(".gitkeep")).is_some()
}

/// Check if a directory contains ONLY `.gitkeep` and nothing else (no other files, no subdirs).
/// A directory with `.gitkeep` + source files is a broken crate, not a placeholder.
pub fn is_gitkeep_only(fs: &dyn FileSystem, dir: &Path) -> bool {
    if !has_gitkeep(fs, dir) {
        return false;
    }
    let file_names = list_file_names(fs, dir);
    let dir_names = list_dir_names(fs, dir);
    file_names.len() == 1 && file_names[0] == ".gitkeep" && dir_names.is_empty()
}

/// Report loose files in a directory (only `.gitkeep` is allowed).
///
/// Parameters:
/// - `id`: Check ID (e.g., "R-ARCH-01" or "T-ARCH-01")
/// - `entity`: Entity label (e.g., "Service" or "TS app")
pub fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    let mut bad_files: Vec<String> = Vec::new();
    for entry in &fs.list_dir(dir) {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() && entry_name != ".gitkeep" {
            bad_files.push(entry_name);
        }
    }

    if !bad_files.is_empty() {
        results.push(CheckResult::from_parts(
            id.to_owned(),
            Severity::Error,
            format!("{entity} `{name}` has loose files in {label}/"),
            format!(
                "{entity} `{name}` has files in `{label}/` that don't belong: {}. \
                 Only `.gitkeep` is allowed in structural/container directories. \
                 Move code into module subdirectories.",
                bad_files.join(", ")
            ),
            Some(dir.display().to_string()),
            None,
            false,
        ));
    }
}

/// Check that a structural directory contains exactly the expected subdirectories.
///
/// Reports missing expected dirs, unexpected dirs, and loose files.
pub fn check_exact_subdirs(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    expected: &[&str],
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    let dir_names = list_dir_names(fs, dir);

    for exp in expected {
        if !dir_names.iter().any(|n| n == exp) {
            results.push(CheckResult::from_parts(
                id.to_owned(),
                Severity::Error,
                format!("{entity} `{name}` missing {label}/{exp}/ directory"),
                format!(
                    "{entity} `{name}` is missing `{label}/{exp}/`. \
                     Create it and add a `.gitkeep` if not needed yet."
                ),
                Some(dir.display().to_string()),
                None,
                false,
            ));
        }
    }

    for dir_name in &dir_names {
        if !expected.contains(&dir_name.as_str()) {
            results.push(CheckResult::from_parts(
                id.to_owned(),
                Severity::Error,
                format!("{entity} `{name}` has unexpected directory {label}/{dir_name}/"),
                format!(
                    "{entity} `{name}` has `{label}/{dir_name}/` which is not part of \
                     the hex arch template. Only `{{{}}}` directories are allowed in `{label}/`.",
                    expected.join(", ")
                ),
                Some(dir.join(dir_name).display().to_string()),
                None,
                false,
            ));
        }
    }

    check_loose_files(fs, name, dir, label, id, entity, results);
}

/// Check that a container is not empty (must have subdirs or .gitkeep).
/// Also calls `check_loose_files` on the container when it has subdirs.
///
/// Design decision: when a container has files but no subdirs and no .gitkeep,
/// we report only the "empty container" error, which already explains the files
/// that are present. `check_loose_files` only runs when the container has
/// subdirs and therefore represents a real structure plus stray files.
pub fn check_container_not_empty(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    id: &str,
    entity: &str,
    results: &mut Vec<CheckResult>,
) {
    if fs.metadata(dir).is_none() {
        return;
    }

    let dir_names = list_dir_names(fs, dir);
    let gitkeep = has_gitkeep(fs, dir);

    if dir_names.is_empty() && !gitkeep {
        let files = list_file_names(fs, dir);
        let detail = if files.is_empty() {
            "is empty".to_owned()
        } else {
            format!("contains files ({}) but no subdirectories", files.join(", "))
        };
        results.push(CheckResult::from_parts(
            id.to_owned(),
            Severity::Error,
            format!("{entity} `{name}` empty container {label}/"),
            format!(
                "{entity} `{name}` container `{label}/` {detail}. \
                 Add module subdirectories or a `.gitkeep` if this layer is not needed yet."
            ),
            Some(dir.display().to_string()),
            None,
            false,
        ));
        return;
    }

    check_loose_files(fs, name, dir, label, id, entity, results);
}
