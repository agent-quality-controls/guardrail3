use cargo_toml_parser::types::{CargoBoolFieldState, CargoLintTableState};
use g3rs_cargo_types::{G3RsCargoPolicyRoot, G3RsCargoWorkspaceMember};
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    allow_selector, explicit_allow_entries, has_valid_lint_level, member_override_lints,
    member_override_lints_state, policy_override_lints, rust_policy_valid,
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
            classify_member_allow(member, family, &lint_name, &mut counts, results);
        }
    }

    let total = counts.total;
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
                "`{}` has {total} member-local manifest allow entries.",
                member.cargo_rel_path
            ),
            &member.cargo_rel_path,
        ));
    }
}

/// Counts of member-local `allow` entries.
#[derive(Default)]
struct AllowCounts {
    /// Member-local `allow` entries found.
    total: usize,
}

/// Classify a single member-local `allow` entry and emit the matching finding.
fn classify_member_allow(
    member: &G3RsCargoWorkspaceMember,
    family: &str,
    lint_name: &str,
    counts: &mut AllowCounts,
    results: &mut Vec<G3CheckResult>,
) {
    let selector = allow_selector(family, lint_name);
    counts.total = counts.total.saturating_add(1);
    results.push(
        crate::support::error(
            ID,
            "member-local allow entry forbidden",
            format!(
                "`{}` uses `[lints] workspace = true` but still sets `{lint_name}` to `allow` in `{family}`. Remove this `allow` override from the member crate or add `[[waivers]]` with rule = \"{ID}\", subject = \"{}\", selector = \"{selector}\", and a specific reason.",
                member.cargo_rel_path,
                member.cargo_rel_path
            ),
            &member.cargo_rel_path,
        )
        .with_selector(selector),
    );
}

/// lints are well formed fn.
fn lints_are_well_formed(lints: CargoLintTableState<'_>) -> bool {
    match lints {
        CargoLintTableState::Missing => true,
        CargoLintTableState::Parsed(table) => table.values().all(has_valid_lint_level),
        CargoLintTableState::WrongType(_) => false,
    }
}
