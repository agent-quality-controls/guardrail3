use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;
use cargo_toml_parser::types::{CargoBoolFieldState, CargoLintTableState, ToolLints};

use crate::support::{
    has_valid_lint_level, is_valid_lint_level, is_weaker, lint_level_for_name,
    member_override_lints, member_override_lints_state, policy_override_lints,
};

const ID: &str = "RS-CARGO-CONFIG-09";

pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    results: &mut Vec<G3CheckResult>,
) {
    if !matches!(
        crate::support::member_lints_workspace_state(member),
        CargoBoolFieldState::Value(true)
    ) {
        return;
    }

    let workspace_rust_lints = policy_override_lints(root, "rust");
    let workspace_clippy_lints = policy_override_lints(root, "clippy");
    let workspace_policy_complete =
        workspace_rust_lints.is_some() && workspace_clippy_lints.is_some();

    let mut violations = 0usize;
    violations += check_family_shape(&member.cargo_rel_path, "rust", member_override_lints_state(member, "rust"), results);
    violations += check_family_shape(
        &member.cargo_rel_path,
        "clippy",
        member_override_lints_state(member, "clippy"),
        results,
    );
    violations += check_family(
        &member.cargo_rel_path,
        "rust",
        workspace_rust_lints,
        member_override_lints(member, "rust"),
        results,
    );
    violations += check_family(
        &member.cargo_rel_path,
        "clippy",
        workspace_clippy_lints,
        member_override_lints(member, "clippy"),
        results,
    );

    if violations == 0 && workspace_policy_complete {
        results.push(crate::support::info(
            ID,
            "no weakened overrides",
            format!(
                "`{}` does not weaken inherited workspace lint policy.",
                member.cargo_rel_path
            ),
            &member.cargo_rel_path,
        ));
    }
}

fn check_family_shape(
    file: &str,
    family: &str,
    member_lints_state: CargoLintTableState<'_>,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    if !matches!(member_lints_state, CargoLintTableState::WrongType(_)) {
        return 0;
    }
    results.push(crate::support::error(
        ID,
        format!("invalid member {family} override"),
        format!(
            "`{family}` overrides in `{file}` must be a table with valid lint levels (`allow`, `warn`, `deny`, or `forbid`)."
        ),
        file,
    ));
    1
}

fn check_family(
    file: &str,
    family: &str,
    workspace_lints: Option<&ToolLints>,
    member_lints: Option<&ToolLints>,
    results: &mut Vec<G3CheckResult>,
) -> usize {
    let (Some(workspace_lints), Some(member_lints)) = (workspace_lints, member_lints) else {
        return 0;
    };
    let mut violations = 0usize;
    for (lint_name, member_value) in member_lints {
        let Some(member_level) = lint_level_for_name(member_lints, lint_name) else {
            violations += 1;
            results.push(crate::support::error(
                ID,
                format!("invalid member {family} override"),
                format!(
                    "`{lint_name}` in `{file}` must use a valid lint level (`allow`, `warn`, `deny`, or `forbid`)."
                ),
                file,
            ));
            continue;
        };
        if !has_valid_lint_level(member_value) || !is_valid_lint_level(member_level.as_str()) {
            violations += 1;
            results.push(crate::support::error(
                ID,
                format!("invalid member {family} override"),
                format!(
                    "`{lint_name}` in `{file}` must use a valid lint level (`allow`, `warn`, `deny`, or `forbid`)."
                ),
                file,
            ));
            continue;
        }
        let Some(workspace_level) = lint_level_for_name(workspace_lints, lint_name) else {
            continue;
        };

        if is_weaker(workspace_level.as_str(), member_level.as_str()) {
            violations += 1;
            results.push(crate::support::error(
                ID,
                format!("weakened member {family} override"),
                format!(
                    "`{lint_name}` is `{member_level}` in the member but `{workspace_level}` in the workspace. Remove the member-level override or set it to `{workspace_level}` or stricter."
                ),
                file,
            ));
        }
    }
    violations
}

#[cfg(test)]
#[path = "rs_cargo_config_09_no_weakened_overrides_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod rs_cargo_config_09_no_weakened_overrides_tests;
