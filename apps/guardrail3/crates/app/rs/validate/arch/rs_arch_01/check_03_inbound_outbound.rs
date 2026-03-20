use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 3: `adapters/` and `ports/` must contain exactly `{inbound, outbound}`.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
) {
    let entries = fs.list_dir(dir);
    if entries.is_empty() {
        return; // missing dir already reported by rule 2
    }

    let dir_names = helpers::list_dir_names(fs, dir);
    for expected in &["inbound", "outbound"] {
        if !dir_names.iter().any(|n| n == expected) {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!("Service `{name}` missing {label}/{expected}/ directory"),
                message: format!(
                    "Service `{name}` is missing `{label}/{expected}/`. \
                     Create it and add a `.gitkeep` if not needed yet."
                ),
                file: Some(dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    for dir_name in &dir_names {
        if dir_name != "inbound" && dir_name != "outbound" {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` has unexpected directory {label}/{dir_name}/"
                ),
                message: format!(
                    "Service `{name}` has `{label}/{dir_name}/` which is not part of \
                     the hex arch template. Only `{{inbound, outbound}}` directories are \
                     allowed in `{label}/`."
                ),
                file: Some(dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    helpers::check_loose_files(fs, name, dir, label, results);
}
