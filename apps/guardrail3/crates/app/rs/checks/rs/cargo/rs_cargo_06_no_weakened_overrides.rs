use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberInput;
use super::lint_support::{is_weaker, lint_level, member_lints, workspace_lints};

const ID: &str = "RS-CARGO-06";

pub fn check(input: &WorkspaceMemberInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(workspace_parsed) = input.workspace.parsed.as_ref() else {
        return;
    };
    let Some(member_parsed) = input.member.parsed.as_ref() else {
        return;
    };

    check_family(
        &input.member.rel_path,
        "rust",
        workspace_lints(workspace_parsed, "rust"),
        member_lints(member_parsed, "rust"),
        results,
    );
    check_family(
        &input.member.rel_path,
        "clippy",
        workspace_lints(workspace_parsed, "clippy"),
        member_lints(member_parsed, "clippy"),
        results,
    );
}

fn check_family(
    file: &str,
    family: &str,
    workspace_lints: Option<&toml::Value>,
    member_lints: Option<&toml::Value>,
    results: &mut Vec<CheckResult>,
) {
    let (Some(workspace_lints), Some(member_lints)) = (workspace_lints, member_lints) else {
        return;
    };

    let Some(member_table) = member_lints.as_table() else {
        return;
    };

    for lint_name in member_table.keys() {
        let Some(workspace_level) = lint_level(workspace_lints, lint_name) else {
            continue;
        };
        let Some(member_level) = lint_level(member_lints, lint_name) else {
            continue;
        };

        if is_weaker(workspace_level.as_str(), member_level.as_str()) {
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
}
