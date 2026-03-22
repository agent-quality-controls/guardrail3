use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMembersSetInput;

const ID: &str = "RS-CARGO-10";

pub fn check(input: &WorkspaceMembersSetInput<'_>, results: &mut Vec<CheckResult>) {
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

#[cfg(test)]
#[path = "rs_cargo_10_missing_member_cargo_tests.rs"]
mod tests;
