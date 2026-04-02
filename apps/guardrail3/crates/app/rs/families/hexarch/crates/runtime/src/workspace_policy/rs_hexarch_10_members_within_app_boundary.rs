use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::WorkspaceCoverageHexarchInput;
use crate::inventory::push_success;

const ID: &str = "RS-HEXARCH-10";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    let before = results.len();
    for member in &input.workspace_members {
        if member.is_within_app_boundary() {
            continue;
        }
        results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!(
                "Service `{}` workspace member `{}` points outside app boundary",
                input.app_name, member.raw
            ),
    format!(
                "Service `{}` lists workspace member `{}` which resolves outside the app boundary. App workspaces must only contain crates inside the app.",
                input.app_name, member.raw
            ),
    Some(input.app_rel_dir.to_owned()),
    None,
    false,
        ));
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` workspace stays inside app boundary",
                input.app_name
            ),
            format!(
                "Service `{}` app workspace members all stay within `{}`.",
                input.app_name, input.app_rel_dir
            ),
            Some(format!("{}/Cargo.toml", input.app_rel_dir)),
        );
    }
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_10_members_within_app_boundary_tests/mod.rs"]
mod rs_hexarch_10_members_within_app_boundary_tests;
