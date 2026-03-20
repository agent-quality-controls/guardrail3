use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 5: container dirs must have `.gitkeep` or at least one subdir.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    if fs.metadata(dir).is_none() {
        return; // missing dir already reported elsewhere
    }

    let dir_names = helpers::list_dir_names(fs, dir);
    let has_gitkeep = helpers::has_gitkeep(fs, dir);

    if dir_names.is_empty() && !has_gitkeep {
        let files = helpers::list_file_names(fs, dir);
        let detail = if files.is_empty() {
            "is empty".to_owned()
        } else {
            format!(
                "contains files ({}) but no crate subdirectories",
                files.join(", ")
            )
        };
        results.push(CheckResult {
            id: "R-ARCH-01".to_owned(),
            severity: Severity::Error,
            title: format!("Service `{name}` empty container {label}/"),
            message: format!(
                "Service `{name}` container `{label}/` {detail}. \
                 Each subdirectory must be a crate with its own `Cargo.toml`, \
                 or add a `.gitkeep` if this layer is not needed yet."
            ),
            file: Some(dir.display().to_string()),
            line: None,
            inventory: false,
        });
    }

    helpers::check_loose_files(fs, name, dir, label, results);
}
