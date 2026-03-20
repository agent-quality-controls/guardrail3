use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 2: `crates/` must contain exactly `{adapters, app, domain, ports}`.
/// No unexpected dirs, no loose files.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    crates_dir: &Path,
    label_prefix: &str,
    results: &mut Vec<CheckResult>,
) {
    let dir_names = helpers::list_dir_names(fs, crates_dir);
    let expected = ["adapters", "app", "domain", "ports"];

    for exp in &expected {
        if !dir_names.iter().any(|n| n == exp) {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!("Service `{name}` missing {label_prefix}/{exp}/ directory"),
                message: format!(
                    "Service `{name}` is missing `{label_prefix}/{exp}/`. Create it and add a \
                     `.gitkeep` if not needed yet."
                ),
                file: Some(crates_dir.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    for dir_name in &dir_names {
        if !expected.contains(&dir_name.as_str()) {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` has unexpected directory {label_prefix}/{dir_name}/"
                ),
                message: format!(
                    "Service `{name}` has `{label_prefix}/{dir_name}/` which is not part of the hex \
                     arch template. Only `{{adapters, app, domain, ports}}` directories are \
                     allowed in `{label_prefix}/`."
                ),
                file: Some(crates_dir.join(dir_name).display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }

    helpers::check_loose_files(fs, name, crates_dir, label_prefix, results);
}
