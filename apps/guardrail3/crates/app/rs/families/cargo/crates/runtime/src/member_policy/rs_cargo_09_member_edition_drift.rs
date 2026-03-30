use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberCargoInput;

const ID: &str = "RS-CARGO-09";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if input.member.parse_error.is_some() {
        return;
    }
    let Some(workspace_edition) = input.workspace.edition.as_deref() else {
        return;
    };
    match input.member.edition.as_deref() {
        Some(member_edition) if edition_rank(member_edition) < edition_rank(workspace_edition) => {
            results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Warn,
    "member edition older than workspace".to_owned(),
    format!(
                    "{} sets edition `{member_edition}` while workspace uses `{workspace_edition}`.",
                    input.member.member_rel
                ),
    Some(input.member.cargo_rel_path.clone()),
    None,
    false,
            ));
        }
        Some(_) => {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "member edition aligns with workspace".to_owned(),
                    format!(
                        "{} does not downgrade the workspace edition.",
                        input.member.member_rel
                    ),
                    Some(input.member.cargo_rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
        None => {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "member inherits workspace edition".to_owned(),
                    format!(
                        "{} inherits workspace edition `{workspace_edition}`.",
                        input.member.member_rel
                    ),
                    Some(input.member.cargo_rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}

fn edition_rank(edition: &str) -> usize {
    match edition {
        "2015" => 0,
        "2018" => 1,
        "2021" => 2,
        "2024" => 3,
        _ => 0,
    }
}

#[cfg(test)]
#[path = "rs_cargo_09_member_edition_drift_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_09_member_edition_drift_tests;
