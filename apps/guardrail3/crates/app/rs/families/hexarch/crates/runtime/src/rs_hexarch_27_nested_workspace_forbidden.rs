use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceCoverageHexarchInput;

const ID: &str = "RS-HEXARCH-27";

pub fn check(input: &WorkspaceCoverageHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if input.cargo_parse_error.is_some() || !input.is_workspace {
        return;
    }

    for cargo_root in &input.app_local_cargo_roots {
        if !cargo_root.is_workspace {
            continue;
        }

        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!(
                "Service `{}` nested workspace `{}` is forbidden",
                input.app_name, cargo_root.rel_dir
            ),
            message: format!(
                "Service `{}` contains nested workspace `{}` at `{}`. The app root Cargo.toml must be the only workspace root inside the app boundary.",
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
#[path = "rs_hexarch_27_nested_workspace_forbidden_tests/mod.rs"]
mod rs_hexarch_27_nested_workspace_forbidden_tests;
