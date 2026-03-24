use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberCargoInput;
use super::lint_support::{is_weaker, lint_level, member_lints, policy_lints};

const ID: &str = "RS-CARGO-06";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.member.lint_workspace_true || input.member.parse_error.is_some() {
        return;
    }

    let Some(member_parsed) = input.member.parsed.as_ref() else {
        return;
    };

    let mut violations = 0usize;
    violations += check_family(
        &input.member.cargo_rel_path,
        "rust",
        policy_lints(input.workspace, "rust"),
        member_lints(member_parsed, "rust"),
        results,
    );
    violations += check_family(
        &input.member.cargo_rel_path,
        "clippy",
        policy_lints(input.workspace, "clippy"),
        member_lints(member_parsed, "clippy"),
        results,
    );

    if violations == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "no weakened overrides".to_owned(),
                message: format!(
                    "`{}` does not weaken inherited workspace lint policy.",
                    input.member.cargo_rel_path
                ),
                file: Some(input.member.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    }
}

fn check_family(
    file: &str,
    family: &str,
    workspace_lints: Option<&toml::Value>,
    member_lints: Option<&toml::Value>,
    results: &mut Vec<CheckResult>,
) -> usize {
    let (Some(workspace_lints), Some(member_lints)) = (workspace_lints, member_lints) else {
        return 0;
    };
    let Some(member_table) = member_lints.as_table() else {
        return 0;
    };

    let mut violations = 0usize;
    for lint_name in member_table.keys() {
        let Some(workspace_level) = lint_level(workspace_lints, lint_name) else {
            continue;
        };
        let Some(member_level) = lint_level(member_lints, lint_name) else {
            continue;
        };

        if is_weaker(workspace_level.as_str(), member_level.as_str()) {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: format!("weakened member {family} override"),
                message: format!(
                    "`{lint_name}` is `{member_level}` in the member but `{workspace_level}` in the workspace."
                ),
                file: Some(file.to_owned()),
                line: None,
                inventory: false,
            });
        }
    }
    violations
}

#[cfg(test)]
#[path = "rs_cargo_06_no_weakened_overrides_tests/mod.rs"]
mod tests;
