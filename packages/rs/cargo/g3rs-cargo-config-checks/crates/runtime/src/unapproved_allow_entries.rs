use cargo_toml_parser::types::ToolLints;
use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    allow_selector, explicit_allow_entries, is_approved_allow, lints_table_is_well_formed,
    policy_override_lints, root_package_policy_lints, rust_policy_valid, rust_policy_waivers,
    waiver_reason,
};

/// I D const.
const ID: &str = "g3rs-cargo/unapproved-allow-entries";

/// Counts of unapproved `allow` entries classified by waiver-reason status.
#[derive(Default)]
struct AllowCounts {
    /// Unapproved `allow` entries with a useful waiver reason.
    documented: usize,
    /// Unapproved `allow` entries that lacked any waiver reason.
    missing_reason: usize,
    /// Unapproved `allow` entries with a waiver reason judged too weak.
    weak_reason: usize,
}

/// check fn.
pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    let rust_lints = policy_override_lints(root, "rust");
    let clippy_lints = policy_override_lints(root, "clippy");
    let lint_tables_well_formed =
        lints_table_is_well_formed(rust_lints) && lints_table_is_well_formed(clippy_lints);

    let mut counts = AllowCounts::default();
    for (family, lints) in [("rust", rust_lints), ("clippy", clippy_lints)] {
        if let Some(lints) = lints {
            inspect_family_lints(root, family, lints, &mut counts, results);
        }

        if let Some(root_package_lints) = root_package_policy_lints(root, family) {
            if !matches!(lints, Some(existing) if core::ptr::eq(existing, root_package_lints)) {
                inspect_family_lints(root, family, root_package_lints, &mut counts, results);
            }
        }
    }

    let total = counts
        .documented
        .saturating_add(counts.missing_reason.saturating_add(counts.weak_reason));
    if total == 0 && lint_tables_well_formed && rust_policy_valid(root) {
        results.push(crate::support::info(
            ID,
            "no unapproved allow entries",
            format!(
                "`{}` does not introduce manifest-level allow entries outside the approved inventory.",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        ));
    } else if total > 0 {
        results.push(crate::support::warn(
            ID,
            "unapproved allow count",
            format!(
                "`{}` has {total} unapproved manifest allow entries ({} documented, {} missing reasons, {} weak reasons).",
                root.cargo_rel_path, counts.documented, counts.missing_reason, counts.weak_reason
            ),
            &root.cargo_rel_path,
        ));
    }
}

/// Inspect every `allow` entry in `lints` for `family`, updating counts and emitting findings.
fn inspect_family_lints(
    root: &G3RsCargoPolicyRoot,
    family: &str,
    lints: &ToolLints,
    counts: &mut AllowCounts,
    results: &mut Vec<G3CheckResult>,
) {
    for lint_name in explicit_allow_entries(Some(lints)) {
        if family == "clippy" && is_approved_allow(&lint_name) {
            continue;
        }
        classify_unapproved_allow(root, family, &lint_name, counts, results);
    }
}

/// Classify a single unapproved-allow entry based on its waiver reason.
fn classify_unapproved_allow(
    root: &G3RsCargoPolicyRoot,
    family: &str,
    lint_name: &str,
    counts: &mut AllowCounts,
    results: &mut Vec<G3CheckResult>,
) {
    let selector = allow_selector(family, lint_name);
    match waiver_reason(
        rust_policy_waivers(root),
        ID,
        &root.cargo_rel_path,
        &selector,
    ) {
        None => {
            counts.missing_reason = counts.missing_reason.saturating_add(1);
            results.push(crate::support::error(
                ID,
                "unapproved allow entry missing reason",
                format!(
                    "`{}` explicitly allows `{lint_name}` in `{family}` without a matching waiver reason. Add a waiver entry in guardrail3-rs.toml for this lint with a reason.",
                    root.cargo_rel_path
                ),
                &root.cargo_rel_path,
            ));
        }
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => {
                counts.documented = counts.documented.saturating_add(1);
                results.push(crate::support::error(
                    ID,
                    "unapproved allow entry",
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `{family}`. Remove this `allow` override or get it added to the approved inventory.",
                        root.cargo_rel_path
                    ),
                    &root.cargo_rel_path,
                ));
            }
            Err(issue) => {
                counts.weak_reason = counts.weak_reason.saturating_add(1);
                results.push(crate::support::error(
                    ID,
                    "unapproved allow entry reason too weak",
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `{family}` with a weak reason: {}.",
                        root.cargo_rel_path,
                        issue.message()
                    ),
                    &root.cargo_rel_path,
                ));
            }
        },
    }
}

#[cfg(test)]
#[path = "unapproved_allow_entries_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod unapproved_allow_entries_tests;
