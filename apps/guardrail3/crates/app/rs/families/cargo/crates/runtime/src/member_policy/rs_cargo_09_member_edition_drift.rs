use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::WorkspaceMemberCargoInput;

const ID: &str = "RS-CARGO-09";

pub fn check(input: &WorkspaceMemberCargoInput<'_>, results: &mut Vec<CheckResult>) {
    if input.member.parse_error.is_some() {
        return;
    }
    if input.workspace.edition_invalid {
        return;
    }
    if input.member.edition_invalid {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "member edition invalid".to_owned(),
            format!(
                "{} must declare edition as a string value or inherit it from the workspace.",
                input.member.member_rel
            ),
            Some(input.member.cargo_rel_path.clone()),
            None,
            false,
        ));
        return;
    }
    let Some(workspace_edition) = input.workspace.edition.as_deref() else {
        return;
    };
    let Some(workspace_rank) = edition_rank(workspace_edition) else {
        return;
    };
    match input.member.edition.as_deref() {
        Some(member_edition) => {
            let Some(member_rank) = edition_rank(member_edition) else {
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "member edition unrecognized".to_owned(),
                    format!(
                        "{} declares unknown edition `{member_edition}`.",
                        input.member.member_rel
                    ),
                    Some(input.member.cargo_rel_path.clone()),
                    None,
                    false,
                ));
                return;
            };
            if member_rank < workspace_rank {
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
            } else {
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

fn edition_rank(edition: &str) -> Option<usize> {
    match edition {
        "2015" => Some(0),
        "2018" => Some(1),
        "2021" => Some(2),
        "2024" => Some(3),
        _ => None,
    }
}

#[cfg(test)]
#[path = "rs_cargo_09_member_edition_drift_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_cargo_09_member_edition_drift_tests;
