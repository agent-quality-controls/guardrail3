use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RequiredInputFailureInput;

const ID: &str = "RS-ARCH-07";

pub fn check(input: &RequiredInputFailureInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Rust architecture required input failed closed".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

pub fn check_success(has_required_input_failures: bool, results: &mut Vec<CheckResult>) {
    if has_required_input_failures {
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "Rust architecture required inputs are readable".to_owned(),
            message:
                "Required Rust architecture placement inputs resolved without unreadable-present or malformed failures."
                    .to_owned(),
            file: None,
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
pub(crate) fn check_results(
    tree: &guardrail3_domain_project_tree::ProjectTree,
) -> Vec<guardrail3_domain_report::CheckResult> {
    crate::check_test_tree(tree)
}

#[cfg(test)]
#[path = "rs_arch_07_required_inputs_fail_closed_tests/mod.rs"]
mod rs_arch_07_required_inputs_fail_closed_tests;
