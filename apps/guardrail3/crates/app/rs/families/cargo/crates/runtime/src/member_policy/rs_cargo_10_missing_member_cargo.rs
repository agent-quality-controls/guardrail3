use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::{MissingMemberCargoInput, MissingMemberInventoryCargoInput};

const ID: &str = "RS-CARGO-10";

pub fn check(input: &MissingMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Warn,
        "declared workspace member missing Cargo.toml".to_owned(),
        format!(
            "`{}` is declared in `[workspace].members` but no `Cargo.toml` was discovered there.",
            input.missing.member_rel
        ),
        Some(input.missing.workspace_cargo_rel_path.clone()),
        None,
        false,
    ));
}

pub fn check_inventory(
    input: &MissingMemberInventoryCargoInput<'_>,
    results: &mut Vec<CheckResult>,
) {
    if input.has_missing_members || input.has_members_parse_error {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "all declared workspace members have Cargo.toml".to_owned(),
            format!(
                "{} declares only member directories that contain Cargo.toml.",
                input.workspace.kind.label()
            ),
            Some(input.workspace.cargo_rel_path.clone()),
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_cargo_10_missing_member_cargo_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_10_missing_member_cargo_tests;
