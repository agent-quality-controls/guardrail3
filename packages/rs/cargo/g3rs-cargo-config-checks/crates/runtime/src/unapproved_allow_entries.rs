use cargo_toml_parser::types::ToolLints;
use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_check_types::G3CheckResult;

use crate::support::{
    allow_selector, explicit_allow_entries, is_approved_allow, lints_table_is_well_formed,
    policy_override_lints, root_package_policy_lints, rust_policy_valid,
};

/// I D const.
const ID: &str = "g3rs-cargo/unapproved-allow-entries";

/// Counts of unapproved `allow` entries.
#[derive(Default)]
struct AllowCounts {
    /// Unapproved `allow` entries found.
    total: usize,
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

    let total = counts.total;
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
                "`{}` has {total} unapproved manifest allow entries.",
                root.cargo_rel_path
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
    counts.total = counts.total.saturating_add(1);
    results.push(
        crate::support::error(
            ID,
            "unapproved allow entry",
            format!(
                "`{}` explicitly allows `{lint_name}` in `{family}`. Remove this `allow` override or add `[[waivers]]` with rule = \"{ID}\", subject = \"{}\", selector = \"{selector}\", and a specific reason.",
                root.cargo_rel_path,
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        )
        .with_selector(selector),
    );
}
