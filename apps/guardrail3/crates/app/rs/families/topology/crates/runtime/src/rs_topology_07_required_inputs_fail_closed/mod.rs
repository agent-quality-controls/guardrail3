use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RequiredInputFailureInput;

const ID: &str = "RS-TOPOLOGY-07";

pub fn check(input: &RequiredInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "Rust topology required input failed closed".to_owned(),
        input.failure.message.clone(),
        Some(input.failure.rel_path.clone()),
        None,
        false,
    ));
}

pub fn check_success(has_required_input_failures: bool, results: &mut Vec<CheckResult>) {
    if has_required_input_failures {
        return;
    }

    results.push(
        CheckResult::from_parts(
            ID.to_owned(),
            Severity::Info,
            "Rust topology required inputs are readable".to_owned(),
            "Required Rust topology placement inputs resolved without unreadable-present or malformed failures."
                    .to_owned(),
            None,
            None,
            false,
        )
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_app_rs_family_view::FamilyView,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]

mod tests;
