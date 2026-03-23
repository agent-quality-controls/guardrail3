use crate::domain::report::{CheckResult, Severity};

use super::inputs::AllowlistCoverageDepsInput;

const ID: &str = "RS-DEPS-08";

pub fn check(input: &AllowlistCoverageDepsInput<'_>, results: &mut Vec<CheckResult>) {
    if input.coverage.profile_name.as_deref() != Some("library") {
        return;
    }

    if input.coverage.has_allowlist {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "library allowlist present".to_owned(),
                message: format!(
                    "Library crate `{}` has an `allowed_deps` policy.",
                    input.coverage.crate_name
                ),
                file: Some(input.coverage.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "library allowlist missing".to_owned(),
            message: format!(
                "Library crate `{}` has no `allowed_deps` policy.",
                input.coverage.crate_name
            ),
            file: Some(input.coverage.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_deps_08_library_allowlist_present_tests.rs"]
mod tests;
