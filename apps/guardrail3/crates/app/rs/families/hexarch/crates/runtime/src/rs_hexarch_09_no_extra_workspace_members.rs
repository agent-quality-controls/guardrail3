use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-09";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    for member in &input.workspace_members {
        if !member.is_within_app_boundary() {
            continue;
        }

        let all_matches_are_real_crates = !member.resolved_dirs.is_empty()
            && member.resolved_dirs.iter().all(|resolved| {
                input
                    .discovered_crate_dirs
                    .iter()
                    .any(|dir| dir == resolved)
            });
        if all_matches_are_real_crates {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` has extra workspace member `{}`",
                input.app_name, member.raw
            ),
            message: format!(
                "Service `{}` lists workspace member `{}` but no matching crate directory exists under the app boundary.",
                input.app_name, member.raw
            ),
            file: Some(input.app_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_hexarch_09_no_extra_workspace_members_tests/mod.rs"]
mod rs_hexarch_09_no_extra_workspace_members_tests;
