use std::path::Path;

use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::FileSystem;

use super::helpers;

/// Rule 6: each container subdir must be a crate (Cargo.toml), a hex-in-hex (crates/ dir),
/// or a placeholder (.gitkeep only).
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
        // Use metadata to detect crates/ existence (not list_dir which can't
        // distinguish empty dir from nonexistent). An empty crates/ is a real
        // hex-in-hex scaffold — recurse into it so inner structural checks
        // report the actual problems.
        let has_crates = fs.metadata(&sub_path.join("crates"))
            .is_some_and(|m| m.is_dir());

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
            // .gitkeep-only subdirs are valid placeholders (reserving the name for later)
            if helpers::has_gitkeep(fs, &sub_path) {
                continue;
            }
            results.push(CheckResult {
                id: "R-ARCH-01".to_owned(),
                severity: Severity::Error,
                title: format!(
                    "Service `{name}` subdirectory {label}/{subdir}/ missing Cargo.toml"
                ),
                message: format!(
                    "Service `{name}` has `{label}/{subdir}/` but it has no `Cargo.toml` \
                     and no `crates/` directory. Every subdirectory in a container folder \
                     must be its own crate, a hex-in-hex with its own `crates/` structure, \
                     or a placeholder with `.gitkeep`."
                ),
                file: Some(sub_path.display().to_string()),
                line: None,
                inventory: false,
            });
        }
    }
}
