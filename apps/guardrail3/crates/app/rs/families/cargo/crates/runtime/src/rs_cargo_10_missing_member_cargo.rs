use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{MissingMemberCargoInput, MissingMemberInventoryCargoInput};

const ID: &str = "RS-CARGO-10";

pub fn check(input: &MissingMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "declared workspace member missing Cargo.toml".to_owned(),
        message: format!(
            "`{}` is declared in `[workspace].members` but no `Cargo.toml` was discovered there.",
            input.missing.member_rel
        ),
        file: Some(input.missing.workspace_cargo_rel_path.clone()),
        line: None,
        inventory: false,
    });
}

pub fn check_inventory(
    input: &MissingMemberInventoryCargoInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    if input.has_missing_members {
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "all declared workspace members have Cargo.toml".to_owned(),
            message: format!(
                "{} declares only member directories that contain Cargo.toml.",
                input.workspace.kind.label()
            ),
            file: Some(input.workspace.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_cargo_10_missing_member_cargo_tests/mod.rs"]
mod rs_cargo_10_missing_member_cargo_tests;
