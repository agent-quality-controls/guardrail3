use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 6: each container subdir must be a crate (Cargo.toml) or hex-in-hex (crates/ dir).
/// If hex-in-hex, calls the provided callback to recurse structural checks.
pub fn check(
    fs: &dyn FileSystem,
    name: &str,
    dir: &Path,
    label: &str,
    results: &mut Vec<CheckResult>,
    recurse: &dyn Fn(&dyn FileSystem, &str, &Path, &str, &mut Vec<CheckResult>),
) {
    if fs.metadata(dir).is_none() {
        return; // missing dir already reported elsewhere
    }

    let dir_names = helpers::list_dir_names(fs, dir);

    for subdir in &dir_names {
        let sub_path = dir.join(subdir);
        let has_cargo = fs.read_file(&sub_path.join("Cargo.toml")).is_some();
        let has_crates = !fs.list_dir(&sub_path.join("crates")).is_empty();

        if has_crates && has_cargo {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` subdirectory {label}/{subdir}/ has both Cargo.toml and crates/"
                ),
                message: format!(
                    "Service `{name}` has `{label}/{subdir}/` with both `Cargo.toml` and `crates/`. \
                     A subdirectory must be either a crate (Cargo.toml) or a hex-in-hex (crates/), not both."
                ),
                file: Some(sub_path.display().to_string()),
                line: None,
                inventory: false,
            });
        } else if has_crates {
            let inner_label = format!("{label}/{subdir}/crates");
            recurse(fs, name, &sub_path, &inner_label, results);
        } else if !has_cargo {
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` subdirectory {label}/{subdir}/ missing Cargo.toml"
                ),
                message: format!(
                    "Service `{name}` has `{label}/{subdir}/` but it has no `Cargo.toml` \
                     and no `crates/` directory. Every subdirectory in a container folder \
                     must be its own crate or a hex-in-hex with its own `crates/` structure."
                ),
                file: Some(sub_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
