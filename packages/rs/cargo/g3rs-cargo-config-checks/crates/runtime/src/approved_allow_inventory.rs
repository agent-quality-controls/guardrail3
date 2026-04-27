use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    EXPECTED_CLIPPY_REQUIRED_ALLOW, allow_selector, explicit_allow_entries, policy_override_lints,
    root_package_policy_lints, rust_policy_valid, rust_policy_waivers, waiver_reason, warn,
};

const ID: &str = "g3rs-cargo/approved-allow-inventory";

pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    let Some(clippy_lints) = policy_override_lints(root, "clippy") else {
        return;
    };

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

    let mut inspect_lints = |lints| {
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
                missing_reason_count += 1;
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
                    documented_count += 1;
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
                    weak_reason_count += 1;
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
    };

    inspect_lints(clippy_lints);
    if let Some(root_package_clippy_lints) = root_package_policy_lints(root, "clippy") {
        if !core::ptr::eq(clippy_lints, root_package_clippy_lints) {
            inspect_lints(root_package_clippy_lints);
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total > 0 {
        results.push(warn(
            ID,
            "approved allow count",
            format!(
                "`{}` has {total} approved manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        ));
    }
}

#[cfg(test)]
#[path = "approved_allow_inventory_tests/mod.rs"]
// reason: owned sidecar tests for file module.
mod approved_allow_inventory_tests;
