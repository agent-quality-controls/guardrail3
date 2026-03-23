use crate::domain::report::{CheckResult, Severity};

use super::inputs::AppHexarchInput;

const ID: &str = "RS-HEXARCH-08";

pub fn check(input: &AppHexarchInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(parse_error) = input.cargo_parse_error {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: format!("Service `{}` Cargo.toml parse error", input.app_name),
            message: format!(
                "Service `{}` has invalid TOML in app Cargo.toml: {parse_error}",
                input.app_name
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
#[path = "rs_hexarch_08_app_cargo_is_workspace_tests.rs"]
mod tests;
