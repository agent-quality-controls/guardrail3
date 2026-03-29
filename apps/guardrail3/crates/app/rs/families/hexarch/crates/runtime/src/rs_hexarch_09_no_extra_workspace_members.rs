use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-09";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    let before = results.len();
    for member in &input.workspace_members {
        if !member.is_within_app_boundary() {
            continue;
        }

        let all_matches_are_real_crates = !member.resolved_dirs.is_empty()
            && member.resolved_dirs.iter().all(|resolved| {
                input
                    .app_local_cargo_roots
                    .iter()
                    .any(|cargo_root| cargo_root.rel_dir == resolved)
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
                "Service `{}` lists workspace member `{}` but no matching live Cargo root exists under the app boundary.",
                input.app_name, member.raw
            ),
            file: Some(input.app_rel_dir.to_owned()),
            line: None,
            inventory: false,
        });
    }

    if results.len() == before {
        push_success(
            results,
            ID,
            format!(
                "Service `{}` has no extra workspace members",
                input.app_name
            ),
            format!(
                "Service `{}` app workspace members all map to live app-local Cargo roots.",
                input.app_name
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
#[path = "rs_hexarch_09_no_extra_workspace_members_tests/mod.rs"]
mod rs_hexarch_09_no_extra_workspace_members_tests;
