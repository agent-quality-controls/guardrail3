use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::WorkspaceMemberCargoInput;
use crate::lint_support::{
    has_valid_lint_level, is_valid_lint_level, is_weaker, lint_level, member_lints, policy_lints,
};

const ID: &str = "RS-CARGO-06";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.member.lint_workspace_true || input.member.parse_error.is_some() {
        return;
    }

    let Some(member_parsed) = input.member.parsed.as_ref() else {
        return;
    };
    let workspace_rust_lints = policy_lints(input.workspace, "rust");
    let workspace_clippy_lints = policy_lints(input.workspace, "clippy");
    let workspace_policy_complete =
        workspace_rust_lints.is_some() && workspace_clippy_lints.is_some();

    let mut violations = 0usize;
    violations += check_family(
        &input.member.cargo_rel_path,
        "rust",
        workspace_rust_lints,
        member_lints(member_parsed, "rust"),
        results,
    );
    violations += check_family(
        &input.member.cargo_rel_path,
        "clippy",
        workspace_clippy_lints,
        member_lints(member_parsed, "clippy"),
        results,
    );

    if violations == 0 && workspace_policy_complete {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no weakened overrides".to_owned(),
                format!(
                    "`{}` does not weaken inherited workspace lint policy.",
                    input.member.cargo_rel_path
                ),
                Some(input.member.cargo_rel_path.clone()),
                None,
                false,
            )
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
    if member_lints.as_table().is_none() {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            format!("invalid member {family} lint table"),
            format!(
                "`{file}` uses `[lints] workspace = true` but defines `[lints.{family}]` with an invalid shape."
            ),
            Some(file.to_owned()),
            None,
            false,
        ));
        return 1;
    }
    let Some(member_table) = member_lints.as_table() else {
        return 0;
    };

    let mut violations = 0usize;
    for (lint_name, member_value) in member_table {
        let Some(member_level) = lint_level(member_lints, lint_name) else {
            violations += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("invalid member {family} override"),
                format!(
                    "`{lint_name}` in `{file}` must use a valid lint level (`allow`, `warn`, `deny`, or `forbid`)."
                ),
                Some(file.to_owned()),
                None,
                false,
            ));
            continue;
        };
        if !has_valid_lint_level(member_value) || !is_valid_lint_level(member_level.as_str()) {
            violations += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("invalid member {family} override"),
                format!(
                    "`{lint_name}` in `{file}` must use a valid lint level (`allow`, `warn`, `deny`, or `forbid`)."
                ),
                Some(file.to_owned()),
                None,
                false,
            ));
            continue;
        }
        let Some(workspace_level) = lint_level(workspace_lints, lint_name) else {
            continue;
        };

        if is_weaker(workspace_level.as_str(), member_level.as_str()) {
            violations += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                format!("weakened member {family} override"),
                format!(
                    "`{lint_name}` is `{member_level}` in the member but `{workspace_level}` in the workspace. Remove the member-level override or set it to `{workspace_level}` or stricter."
                ),
                Some(file.to_owned()),
                None,
                false,
            ));
        }
    }
    violations
}

// reason: test-only sidecar module wiring
