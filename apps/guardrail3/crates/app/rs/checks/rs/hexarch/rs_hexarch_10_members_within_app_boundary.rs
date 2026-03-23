use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-10";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() {
        return;
    }

    for member in input.workspace_members {
        if member.starts_with("crates/") {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` workspace member `{}` points outside app boundary",
                input.app_name, member
            ),
            message: format!(
                "Service `{}` lists workspace member `{}` which resolves outside the app boundary. App workspaces must only contain crates inside the app.",
                input.app_name, member
            ),
            file: Some(input.app_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_hexarch_10_members_within_app_boundary_tests/mod.rs"]
mod tests;
