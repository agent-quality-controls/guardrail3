use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::inputs::PolicyRootCargoInput;
use crate::lint_support::{
    allow_selector, escape_hatch_reason, explicit_allow_entries, is_approved_allow,
    lints_table_is_well_formed, policy_lints,
};

const ID: &str = "RS-CARGO-12";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let rust_lints = policy_lints(root, "rust");
    let clippy_lints = policy_lints(root, "clippy");
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
            match escape_hatch_reason(
                &root.escape_hatches,
                "cargo",
                &root.cargo_rel_path,
                "lint_allow",
                &selector,
            ) {
                None => {
                    missing_reason_count += 1;
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Error,
                        "unapproved allow entry missing reason".to_owned(),
                        format!(
                            "`{}` explicitly allows `{lint_name}` in `{family}` without a matching escape-hatch reason.",
                            root.cargo_rel_path
                        ),
                        Some(root.cargo_rel_path.clone()),
                        None,
                        false,
                    ));
                }
                Some(reason) => match validate_reason_text(reason) {
                    Ok(()) => {
                        documented_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Error,
                            "unapproved allow entry".to_owned(),
                            format!(
                                "`{}` explicitly allows `{lint_name}` in `{family}`. The entry is documented but still forbidden.",
                                root.cargo_rel_path
                            ),
                            Some(root.cargo_rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                    Err(issue) => {
                        weak_reason_count += 1;
                        results.push(CheckResult::from_parts(
                            ID.to_owned(),
                            Severity::Error,
                            "unapproved allow entry reason too weak".to_owned(),
                            format!(
                                "`{}` explicitly allows `{lint_name}` in `{family}` with a weak reason: {}.",
                                root.cargo_rel_path,
                                issue.message()
                            ),
                            Some(root.cargo_rel_path.clone()),
                            None,
                            false,
                        ));
                    }
                },
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total == 0 && lint_tables_well_formed && !root.guardrail_parse_error {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "no unapproved allow entries".to_owned(),
                format!(
                    "`{}` does not introduce manifest-level allow entries outside the approved inventory.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else if total > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "unapproved allow count".to_owned(),
            format!(
                "`{}` has {total} unapproved manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                root.cargo_rel_path
            ),
            None,
            None,
            false,
        ));
    }
}

#[cfg(test)]
#[path = "rs_cargo_12_unapproved_allow_entries_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_cargo_12_unapproved_allow_entries_tests;
