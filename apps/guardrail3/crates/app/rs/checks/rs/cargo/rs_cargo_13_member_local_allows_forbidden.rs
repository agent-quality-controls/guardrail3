use crate::domain::report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberCargoInput;
use super::lint_support::{explicit_allow_entries, member_lints};

const ID: &str = "RS-CARGO-13";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.member.lint_workspace_true || input.member.parse_error.is_some() {
        return;
    }
    let Some(parsed) = input.member.parsed.as_ref() else {
        return;
    };

    let mut violations = 0usize;
    for (family, lints) in [
        ("rust", member_lints(parsed, "rust")),
        ("clippy", member_lints(parsed, "clippy")),
    ] {
        for lint_name in explicit_allow_entries(lints) {
            violations += 1;
            results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "member-local allow entry forbidden".to_owned(),
                message: format!(
                    "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}`.",
                    input.member.cargo_rel_path
                ),
                file: Some(input.member.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            });
        }
    }

    if violations == 0 {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "no member-local allow entries".to_owned(),
                message: format!(
                    "`{}` does not add member-local allow entries on top of inherited policy.",
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

#[cfg(test)]
#[path = "rs_cargo_13_member_local_allows_forbidden_tests/mod.rs"]
mod tests;
