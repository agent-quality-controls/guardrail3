use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;
use super::inventory::push_success;

const ID: &str = "RS-HEXARCH-08";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(cargo_error) = input.cargo_parse_error {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("Service `{}` Cargo.toml has invalid workspace config", input.app_name),
            format!(
                "Service `{}` app Cargo.toml must define a valid `[workspace]` manifest. Invalid workspace config: {cargo_error}",
                input.app_name,
            ),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        ));
        return;
    }

    if input.is_workspace {
        push_success(
            results,
            ID,
            format!("Service `{}` Cargo.toml is a workspace", input.app_name),
            format!(
                "Service `{}` uses `{}` as the app workspace root.",
                input.app_name, input.cargo_rel_path
            ),
            Some(input.cargo_rel_path.to_owned()),
        );
        return;
    }

    results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    format!("Service `{}` Cargo.toml must be a workspace", input.app_name),
    format!(
            "Service `{}` app Cargo.toml must define `[workspace]` so the app boundary owns its internal crates.",
            input.app_name
        ),
    Some(input.cargo_rel_path.to_owned()),
    None,
    false,
    ));
}

#[cfg(test)]
pub(super) fn results_for_test_root(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_tree(&test_support::walk(root))
}
#[cfg(test)]
#[path = "rs_hexarch_08_app_cargo_is_workspace_tests/mod.rs"]
mod rs_hexarch_08_app_cargo_is_workspace_tests;
