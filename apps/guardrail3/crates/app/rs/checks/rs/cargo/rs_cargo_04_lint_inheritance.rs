use crate::domain::report::{CheckResult, Severity};

use super::inputs::{WorkspaceMemberInput, WorkspaceMembersSetInput};

const ID: &str = "RS-CARGO-04";

pub fn check(input: &WorkspaceMemberInput<'_>, results: &mut Vec<CheckResult>) {
    if let Some(parse_error) = &input.member.parse_error {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "member Cargo.toml parse error".to_owned(),
            message: format!(
                "{}: failed to parse member Cargo.toml: {parse_error}",
                input.member.member_rel
            ),
            file: Some(input.member.rel_path.clone()),
            line: None,
            inventory: false,
        });
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
                    "{}: `[lints] workspace = true` inherits workspace lint policy",
                    package_name
                ),
                file: Some(input.member.rel_path.clone()),
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
            file: Some(input.member.rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

pub fn check_missing_member_cargos(
    input: &WorkspaceMembersSetInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    for declared_member in input.declared_members {
        if !input.discovered_members.contains(declared_member) {
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Warn,
                title: "declared workspace member missing Cargo.toml".to_owned(),
                message: format!(
                    "`{declared_member}` is declared in `[workspace].members` but no `Cargo.toml` was discovered there."
                ),
                file: Some(input.workspace.rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }
}
