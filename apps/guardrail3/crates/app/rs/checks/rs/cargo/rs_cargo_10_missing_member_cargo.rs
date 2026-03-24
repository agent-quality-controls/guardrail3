use crate::domain::report::{CheckResult, Severity};

use super::inputs::MissingMemberCargoInput;

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

#[cfg(test)]
#[path = "rs_cargo_10_missing_member_cargo_tests/mod.rs"]
mod tests;
