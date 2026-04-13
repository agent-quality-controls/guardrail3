use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;

const ID: &str = "RS-CARGO-CONFIG-10";

pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    results: &mut Vec<G3CheckResult>,
) {
    if root.edition_invalid {
        return;
    }
    if member.edition_invalid {
        results.push(crate::support::error(
            ID,
            "member edition invalid",
            format!(
                "{} must declare edition as a string value or inherit it from the workspace.",
                member.member_rel
            ),
            &member.cargo_rel_path,
        ));
        return;
    }
    let Some(workspace_edition) = root.edition.as_deref() else {
        return;
    };
    let Some(workspace_rank) = edition_rank(workspace_edition) else {
        return;
    };

    match member.edition.as_deref() {
        Some(member_edition) => {
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
        None => {
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
#[path = "rs_cargo_config_10_member_edition_drift_tests/mod.rs"]
mod tests;
