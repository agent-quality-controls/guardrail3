use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

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

/// Report loose files in a directory (only `.gitkeep` is allowed).
pub fn check_loose_files(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    let entries = fs.list_dir(dir);
    let mut bad_files: Vec<String> = Vec::new();

    for entry in &entries {
        let entry_name = entry.file_name().to_string_lossy().into_owned();
        let Ok(ft) = entry.file_type() else {
            continue;
        };
        if !ft.is_dir() && entry_name != ".gitkeep" {
            bad_files.push(entry_name);
        }
    }

    if !bad_files.is_empty() {
        results.push(CheckResult {
            id: "R-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("Service `{name}` has loose files in {label}/"),
            message: format!(
                "Service `{name}` has files in `{label}/` that don't belong: {}. \
                 Only `.gitkeep` is allowed in structural/container directories. \
                 Move code into crate subdirectories.",
                bad_files.join(", ")
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }
}
