use cargo_toml_parser::types::{CargoBoolFieldState, CargoLintTableState};
use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    allow_selector, explicit_allow_entries, has_valid_lint_level, member_override_lints,
    member_override_lints_state, policy_override_lints, rust_policy_valid, rust_policy_waivers,
    waiver_reason,
};

/// I D const.
const ID: &str = "g3rs-cargo/member-local-allows-forbidden";

/// check fn.
pub(crate) fn check(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    results: &mut Vec<G3CheckResult>,
) {
    if !matches!(
        crate::support::member_lints_workspace_state(member),
        CargoBoolFieldState::Value(true)
    ) {
        return;
    }
    if !rust_policy_valid(root) {
        return;
    }

    let workspace_policy_complete = policy_override_lints(root, "rust").is_some()
        && policy_override_lints(root, "clippy").is_some();
    let member_override_shapes_valid = [
        member_override_lints_state(member, "rust"),
        member_override_lints_state(member, "clippy"),
    ]
    .into_iter()
    .all(lints_are_well_formed);

    let mut counts = AllowCounts::default();
    for (family, lints) in [
        ("rust", member_override_lints(member, "rust")),
        ("clippy", member_override_lints(member, "clippy")),
    ] {
        for lint_name in explicit_allow_entries(lints) {
            classify_member_allow(root, member, family, &lint_name, &mut counts, results);
        }
    }

    let total = counts
        .documented
        .saturating_add(counts.missing_reason.saturating_add(counts.weak_reason));
    let documented_count = counts.documented;
    let missing_reason_count = counts.missing_reason;
    let weak_reason_count = counts.weak_reason;
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

/// Counts of member-local `allow` entries classified by waiver-reason status.
#[derive(Default)]
struct AllowCounts {
    /// Member-local `allow` entries with a useful waiver reason.
    documented: usize,
    /// Member-local `allow` entries that lacked any waiver reason.
    missing_reason: usize,
    /// Member-local `allow` entries with a waiver reason judged too weak.
    weak_reason: usize,
}

/// Classify a single member-local `allow` entry and emit the matching finding.
fn classify_member_allow(
    root: &G3RsCargoPolicyRoot,
    member: &G3RsCargoWorkspaceMember,
    family: &str,
    lint_name: &str,
    counts: &mut AllowCounts,
    results: &mut Vec<G3CheckResult>,
) {
    let selector = allow_selector(family, lint_name);
    match waiver_reason(
        rust_policy_waivers(root),
        ID,
        &member.cargo_rel_path,
        &selector,
    ) {
        None => {
            counts.missing_reason = counts.missing_reason.saturating_add(1);
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
                counts.documented = counts.documented.saturating_add(1);
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
                counts.weak_reason = counts.weak_reason.saturating_add(1);
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

/// lints are well formed fn.
fn lints_are_well_formed(lints: CargoLintTableState<'_>) -> bool {
    match lints {
        CargoLintTableState::Missing => true,
        CargoLintTableState::Parsed(table) => table.values().all(has_valid_lint_level),
        CargoLintTableState::WrongType(_) => false,
    }
}

#[cfg(test)]
#[path = "member_local_allows_forbidden_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod member_local_allows_forbidden_tests;
