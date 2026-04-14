use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    allow_selector, explicit_allow_entries, has_valid_lint_level, raw_member_lints,
    raw_policy_lints, rust_policy_waivers, waiver_reason,
};

const ID: &str = "RS-CARGO-CONFIG-12";

pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    results: &mut Vec<G3CheckResult>,
) {
    if member.lint_workspace_invalid || !member.lint_workspace_true {
        return;
    }

    let workspace_policy_complete =
        raw_policy_lints(root, "rust").is_some() && raw_policy_lints(root, "clippy").is_some();
    let member_override_shapes_valid =
        [raw_member_lints(member, "rust"), raw_member_lints(member, "clippy")]
            .into_iter()
            .all(lints_are_well_formed);

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;
    for (family, lints) in [
        ("rust", raw_member_lints(member, "rust")),
        ("clippy", raw_member_lints(member, "clippy")),
    ] {
        for lint_name in explicit_allow_entries(lints) {
            let selector = allow_selector(family, &lint_name);
            match waiver_reason(
                rust_policy_waivers(root),
                ID,
                &member.cargo_rel_path,
                &selector,
            ) {
                None => {
                    missing_reason_count += 1;
                    results.push(crate::support::error(
                        ID,
                        "member-local allow entry missing reason",
                        format!(
                            "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}` without a matching waiver reason. Add a waiver entry in guardrail3-rs.toml for this lint with a reason.",
                            member.cargo_rel_path
                        ),
                        &member.cargo_rel_path,
                    ));
                }
                Some(reason) => match validate_reason_text(reason) {
                    Ok(()) => {
                        documented_count += 1;
                        results.push(crate::support::error(
                            ID,
                            "member-local allow entry forbidden",
                            format!(
                                "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}`. Remove this `allow` override from the member crate.",
                                member.cargo_rel_path
                            ),
                            &member.cargo_rel_path,
                        ));
                    }
                    Err(issue) => {
                        weak_reason_count += 1;
                        results.push(crate::support::error(
                            ID,
                            "member-local allow entry reason too weak",
                            format!(
                                "`{}` sets `{lint_name}` to `allow` in `{family}` with a weak reason: {}.",
                                member.cargo_rel_path,
                                issue.message()
                            ),
                            &member.cargo_rel_path,
                        ));
                    }
                },
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total == 0 && workspace_policy_complete && member_override_shapes_valid {
        results.push(crate::support::info(
            ID,
            "no member-local allow entries",
            format!(
                "`{}` does not add member-local allow entries on top of inherited policy.",
                member.cargo_rel_path
            ),
            &member.cargo_rel_path,
        ));
    } else if total > 0 {
        results.push(crate::support::warn(
            ID,
            "member-local allow count",
            format!(
                "`{}` has {total} member-local manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                member.cargo_rel_path
            ),
            &member.cargo_rel_path,
        ));
    }
}

fn lints_are_well_formed(lints: Option<&toml::Value>) -> bool {
    let Some(lints) = lints else {
        return true;
    };
    let Some(table) = lints.as_table() else {
        return false;
    };
    table.values().all(has_valid_lint_level)
}

#[cfg(test)]
#[path = "rs_cargo_config_12_member_local_allows_forbidden_tests/mod.rs"]
mod tests;
