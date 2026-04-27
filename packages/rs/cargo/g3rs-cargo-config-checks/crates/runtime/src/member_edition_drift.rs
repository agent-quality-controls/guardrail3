use cargo_toml_parser::types::CargoStringFieldState;
use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "g3rs-cargo/member-edition-drift";

pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    results: &mut Vec<G3CheckResult>,
) {
    if matches!(
        crate::support::root_edition_state(root),
        CargoStringFieldState::WrongType(_)
    ) {
        return;
    }
    let CargoStringFieldState::Value(workspace_edition) = crate::support::root_edition_state(root)
    else {
        return;
    };
    let Some(workspace_rank) = edition_rank(workspace_edition) else {
        return;
    };

    match crate::support::member_edition_state(member) {
        CargoStringFieldState::WrongType(_) => {
            results.push(crate::support::error(
                ID,
                "member edition invalid",
                format!(
                    "{} must declare edition as a string value or inherit it from the workspace.",
                    member.member_rel
                ),
                &member.cargo_rel_path,
            ));
        }
        CargoStringFieldState::Value(member_edition) => {
            let Some(member_rank) = edition_rank(member_edition) else {
                results.push(crate::support::error(
                    ID,
                    "member edition unrecognized",
                    format!(
                        "{} declares unknown edition `{member_edition}`. Use a supported edition like `2021` or `2024`.",
                        member.member_rel
                    ),
                    &member.cargo_rel_path,
                ));
                return;
            };
            if member_rank < workspace_rank {
                results.push(crate::support::warn(
                    ID,
                    "member edition older than workspace",
                    format!(
                        "{} sets edition `{member_edition}` while workspace uses `{workspace_edition}`. Update the member edition to `{workspace_edition}` or remove the override to inherit.",
                        member.member_rel
                    ),
                    &member.cargo_rel_path,
                ));
            } else {
                results.push(crate::support::info(
                    ID,
                    "member edition aligns with workspace",
                    format!(
                        "{} does not downgrade the workspace edition.",
                        member.member_rel
                    ),
                    &member.cargo_rel_path,
                ));
            }
        }
        CargoStringFieldState::Missing | CargoStringFieldState::Inherit => {
            results.push(crate::support::info(
                ID,
                "member inherits workspace edition",
                format!(
                    "{} inherits workspace edition `{workspace_edition}`.",
                    member.member_rel
                ),
                &member.cargo_rel_path,
            ));
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
#[path = "member_edition_drift_tests/mod.rs"] // reason: owned sidecar tests for file module.
mod member_edition_drift_tests;
