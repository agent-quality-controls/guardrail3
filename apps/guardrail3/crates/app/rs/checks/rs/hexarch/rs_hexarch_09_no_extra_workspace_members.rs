use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-09";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() {
        return;
    }

    for member in input.workspace_members {
        if !member.starts_with("crates/") {
            continue;
        }
        if input.discovered_crate_dirs.iter().any(|dir| dir == member) {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` has extra workspace member `{}`",
                input.app_name, member
            ),
            message: format!(
                "Service `{}` lists workspace member `{}` but no matching crate directory exists under the app boundary.",
                input.app_name, member
            ),
            file: Some(input.app_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_hexarch_09_no_extra_workspace_members_tests.rs"]
mod tests;
