use cargo_toml_parser::types::ToolLints;
use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    EXPECTED_CLIPPY_REQUIRED_ALLOW, allow_selector, explicit_allow_entries, policy_override_lints,
    root_package_policy_lints, rust_policy_valid, rust_policy_waivers, waiver_reason, warn,
};

/// I D const.
const ID: &str = "g3rs-cargo/approved-allow-inventory";

/// Counts of approved-allow entries classified by waiver-reason status.
#[derive(Default)]
struct AllowCounts {
    /// Approved allow entries with a useful waiver reason.
    documented: usize,
    /// Approved allow entries that lacked any waiver reason.
    missing_reason: usize,
    /// Approved allow entries with a waiver reason judged too weak.
    weak_reason: usize,
}

/// check fn.
pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    let Some(clippy_lints) = policy_override_lints(root, "clippy") else {
        return;
    };

    let mut counts = AllowCounts::default();
    inspect_clippy_lints(root, clippy_lints, &mut counts, results);
    if let Some(root_package_clippy_lints) = root_package_policy_lints(root, "clippy") {
        if !core::ptr::eq(clippy_lints, root_package_clippy_lints) {
            inspect_clippy_lints(root, root_package_clippy_lints, &mut counts, results);
        }
    }

    let total = counts
        .documented
        .saturating_add(counts.missing_reason.saturating_add(counts.weak_reason));
    if total > 0 {
        results.push(warn(
            ID,
            "approved allow count",
            format!(
                "`{}` has {total} approved manifest allow entries ({} documented, {} missing reasons, {} weak reasons).",
                root.cargo_rel_path, counts.documented, counts.missing_reason, counts.weak_reason
            ),
            &root.cargo_rel_path,
        ));
    }
}

/// Walk `lints` and report each non-required `allow` entry's waiver-reason status.
fn inspect_clippy_lints(
    root: &G3RsCargoPolicyRoot,
    lints: &ToolLints,
    counts: &mut AllowCounts,
    results: &mut Vec<G3CheckResult>,
) {
    for lint_name in explicit_allow_entries(Some(lints)) {
        if EXPECTED_CLIPPY_REQUIRED_ALLOW
            .iter()
            .any(|required| required.name == lint_name)
        {
            continue;
        }
        let selector = allow_selector("clippy", &lint_name);
        let Some(reason) = waiver_reason(
            rust_policy_waivers(root),
            ID,
            &root.cargo_rel_path,
            &selector,
        ) else {
            counts.missing_reason = counts.missing_reason.saturating_add(1);
            results.push(crate::support::error(
                ID,
                "approved allow entry missing reason",
                format!(
                    "`{}` explicitly allows `{lint_name}` in `clippy` without a matching waiver reason. Add a waiver entry in guardrail3-rs.toml for this lint with a reason.",
                    root.cargo_rel_path
                ),
                &root.cargo_rel_path,
            ));
            continue;
        };

        match validate_reason_text(reason) {
            Ok(()) => {
                counts.documented = counts.documented.saturating_add(1);
                results.push(warn(
                    ID,
                    "approved allow entry",
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `clippy` with documented reason `{reason}`.",
                        root.cargo_rel_path
                    ),
                    &root.cargo_rel_path,
                ));
            }
            Err(issue) => {
                counts.weak_reason = counts.weak_reason.saturating_add(1);
                results.push(crate::support::error(
                    ID,
                    "approved allow entry reason too weak",
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `clippy` with a weak reason: {}. Provide a more specific reason.",
                        root.cargo_rel_path,
                        issue.message()
                    ),
                    &root.cargo_rel_path,
                ));
            }
        }
    }
}
