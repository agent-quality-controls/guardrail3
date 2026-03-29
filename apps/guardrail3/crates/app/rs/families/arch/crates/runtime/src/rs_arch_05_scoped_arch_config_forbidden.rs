use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ScopedArchConfigInput;

const ID: &str = "RS-ARCH-05";

pub fn check(input: &ScopedArchConfigInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "Scoped `arch` config is forbidden".to_owned(),
        message: input.failure.message.clone(),
        file: Some(input.failure.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

pub fn check_success(
    has_scoped_failures: bool,
    config_input_failed_closed: bool,
    results: &mut Vec<CheckResult>,
) {
    if has_scoped_failures || config_input_failed_closed {
        return;
    }

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "arch config remains global-only".to_owned(),
            message:
                "No forbidden scoped `arch` configuration was found under app or package sections."
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
#[path = "rs_arch_05_scoped_arch_config_forbidden_tests/mod.rs"]
mod rs_arch_05_scoped_arch_config_forbidden_tests;
