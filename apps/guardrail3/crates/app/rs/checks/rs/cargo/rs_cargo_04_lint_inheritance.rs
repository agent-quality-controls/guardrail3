use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberCargoInput;

const ID: &str = "RS-CARGO-04";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if input.member.parse_error.is_some() {
        return;
    }

    if input.member.lint_workspace_true {
        let package_name = input.member.package_name.as_deref().unwrap_or("unknown");
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "workspace lints inherited".to_owned(),
                message: format!(
                    "{package_name}: `[lints] workspace = true` inherits workspace lint policy"
                ),
                file: Some(input.member.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "workspace lints not inherited".to_owned(),
            message: format!(
                "{}: missing `[lints] workspace = true` in member Cargo.toml",
                input.member.member_rel
            ),
            file: Some(input.member.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_cargo_04_lint_inheritance_tests/mod.rs"]
mod tests;
