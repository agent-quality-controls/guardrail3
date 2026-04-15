use g3rs_cargo_types::G3RsCargoPolicyRoot;
use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{
    allow_selector, explicit_allow_entries, is_approved_allow, lints_table_is_well_formed,
    raw_policy_lints, rust_policy_valid, rust_policy_waivers, waiver_reason,
};

const ID: &str = "RS-CARGO-CONFIG-11";

pub(crate) fn check(root: &G3RsCargoPolicyRoot, results: &mut Vec<G3CheckResult>) {
    if !rust_policy_valid(root) {
        return;
    }
    let rust_lints = raw_policy_lints(root, "rust");
    let clippy_lints = raw_policy_lints(root, "clippy");
    let lint_tables_well_formed =
        lints_table_is_well_formed(rust_lints) && lints_table_is_well_formed(clippy_lints);

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;
    for (family, lints) in [("rust", rust_lints), ("clippy", clippy_lints)] {
        for lint_name in explicit_allow_entries(lints) {
            if family == "clippy" && is_approved_allow(&lint_name) {
                continue;
            }
            let selector = allow_selector(family, &lint_name);
            match waiver_reason(
                rust_policy_waivers(root),
                ID,
                &root.cargo_rel_path,
                &selector,
            ) {
                None => {
                    missing_reason_count += 1;
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
                        documented_count += 1;
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
                        weak_reason_count += 1;
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
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
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
                "`{}` has {total} unapproved manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                root.cargo_rel_path
            ),
            &root.cargo_rel_path,
        ));
    }
}
