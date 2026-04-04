use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::inputs::PolicyRootCargoInput;
use crate::lint_support::{
    EXPECTED_CLIPPY_REQUIRED_ALLOW, allow_selector, escape_hatch_reason, explicit_allow_entries,
    policy_lints,
};

const ID: &str = "RS-CARGO-03";

pub fn check(input: &PolicyRootCargoInput<'_>, results: &mut Vec<CheckResult>) {
    let root = input.root;
    let Some(_parsed) = root.parsed.as_ref() else {
        return;
    };
    let Some(clippy_lints) = policy_lints(root, "clippy") else {
        return;
    };
    if clippy_lints.as_table().is_none() {
        return;
    }

    let mut documented_count = 0usize;
    let mut missing_reason_count = 0usize;
    let mut weak_reason_count = 0usize;

    for lint_name in explicit_allow_entries(Some(clippy_lints)) {
        // Skip lints that CARGO-01 requires to be allowed — those are mandated policy, not escape hatches.
        if EXPECTED_CLIPPY_REQUIRED_ALLOW
            .iter()
            .any(|required| required.name == lint_name)
        {
            continue;
        }
        let selector = allow_selector("clippy", &lint_name);
        let Some(reason) = escape_hatch_reason(
            &root.escape_hatches,
            "cargo",
            &root.cargo_rel_path,
            "lint_allow",
            &selector,
        ) else {
            missing_reason_count += 1;
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "approved allow entry missing reason".to_owned(),
                format!(
                    "`{}` explicitly allows `{lint_name}` in `clippy` without a matching escape-hatch reason. Add an escape-hatch entry in guardrail3.toml for this lint with a reason.",
                    root.cargo_rel_path
                ),
                Some(root.cargo_rel_path.clone()),
                None,
                false,
            ));
            continue;
        };

        match validate_reason_text(reason) {
            Ok(()) => {
                documented_count += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Warn,
                    "approved allow entry".to_owned(),
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `clippy` with documented reason `{reason}`.",
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
                    "approved allow entry reason too weak".to_owned(),
                    format!(
                        "`{}` explicitly allows `{lint_name}` in `clippy` with a weak reason: {}. Provide a more specific reason.",
                        root.cargo_rel_path,
                        issue.message()
                    ),
                    Some(root.cargo_rel_path.clone()),
                    None,
                    false,
                ));
            }
        }
    }

    let total = documented_count + missing_reason_count + weak_reason_count;
    if total > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "approved allow count".to_owned(),
            format!(
                "`{}` has {total} approved manifest allow entries ({documented_count} documented, {missing_reason_count} missing reasons, {weak_reason_count} weak reasons).",
                root.cargo_rel_path
            ),
            None,
            None,
            false,
        ));
    }
}
