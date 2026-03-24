use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-07";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    for crate_dir in input.discovered_crate_dirs {
        if input
            .workspace_members
            .iter()
            .any(|member| member.covers_dir(crate_dir))
        {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` crate `{}` is not a workspace member",
                input.app_name, crate_dir
            ),
            message: format!(
                "Service `{}` has crate directory `{}` but it is not listed in `[workspace].members` of the app Cargo.toml.",
                input.app_name, crate_dir
            ),
            file: Some(input.app_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_hexarch_07_workspace_members_match_crate_dirs_tests/mod.rs"]
mod tests;
