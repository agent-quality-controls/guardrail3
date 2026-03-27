use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-07";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    for cargo_root in &input.app_local_cargo_roots {
        if input
            .workspace_members
            .iter()
            .any(|member| member.covers_dir(cargo_root.rel_dir))
        {
            continue;
        }

        let parse_suffix = cargo_root
            .cargo_parse_error
            .map_or_else(String::new, |message| {
                format!(" The nested Cargo.toml is malformed: {message}")
            });
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` Cargo root `{}` is not a workspace member",
                input.app_name, cargo_root.rel_dir
            ),
            message: format!(
                "Service `{}` has live Cargo root `{}` at `{}` but it is not listed in `[workspace].members` of the app Cargo.toml. Every live app-local Cargo root must be owned by the app workspace.{parse_suffix}",
                input.app_name,
                cargo_root.rel_dir,
                cargo_root.cargo_rel_path
            ),
            file: Some(cargo_root.cargo_rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_07_workspace_members_match_crate_dirs_tests/mod.rs"]
mod rs_hexarch_07_workspace_members_match_crate_dirs_tests;
