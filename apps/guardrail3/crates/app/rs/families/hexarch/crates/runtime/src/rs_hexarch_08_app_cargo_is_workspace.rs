use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;

const ID: &str = "RS-HEXARCH-08";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(cargo_error) = input.cargo_parse_error {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("Service `{}` Cargo.toml has invalid workspace config", input.app_name),
            message: format!(
                "Service `{}` app Cargo.toml must define a valid `[workspace]` manifest. Invalid workspace config: {cargo_error}",
                input.app_name,
            ),
            file: Some(input.cargo_rel_path.to_owned()),
            line: None,
            inventory: false,
        });
        return;
    }

    if input.is_workspace {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: format!("Service `{}` Cargo.toml must be a workspace", input.app_name),
        message: format!(
            "Service `{}` app Cargo.toml must define `[workspace]` so the app boundary owns its internal crates.",
            input.app_name
        ),
        file: Some(input.cargo_rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}

#[cfg(test)]
#[allow(dead_code)]
pub(super) fn results_for_test_tree(tree: &guardrail3_domain_project_tree::ProjectTree) -> Vec<CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_hexarch_08_app_cargo_is_workspace_tests/mod.rs"]
mod rs_hexarch_08_app_cargo_is_workspace_tests;
