use crate::domain::report::{CheckResult, Severity};
use crate::ports::outbound::ToolChecker;

const ID: &str = "HOOK-RS-14";

pub fn check(
    rel_path: &str,
    guardrail_validation_expected: bool,
    guardrail_validation_path_qualified: bool,
    tc: &dyn ToolChecker,
    results: &mut Vec<CheckResult>,
) {
    if !guardrail_validation_expected {
        return;
    }

    if guardrail_validation_path_qualified || tc.is_installed("guardrail3") {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "guardrail3 binary available".to_owned(),
                message: "guardrail3 is available for fail-closed Rust hook validation.".to_owned(),
                file: Some(rel_path.to_owned()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "guardrail3 binary missing".to_owned(),
            message: "Hook requires guardrail3, but it is not available on PATH.".to_owned(),
            file: Some(rel_path.to_owned()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "hook_rs_14_guardrail_binary_available_tests.rs"]
mod tests;
