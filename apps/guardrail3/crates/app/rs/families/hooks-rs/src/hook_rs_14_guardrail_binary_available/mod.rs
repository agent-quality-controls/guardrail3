use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_outbound_traits::ToolChecker;

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
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "guardrail3 binary available".to_owned(),
                "guardrail3 is available for fail-closed Rust hook validation.".to_owned(),
                Some(rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "guardrail3 binary missing".to_owned(),
            "Hook requires guardrail3, but it is not available on PATH.".to_owned(),
            Some(rel_path.to_owned()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_case(
    guardrail_validation_expected: bool,
    guardrail_validation_path_qualified: bool,
    installed: &[&'static str],
) -> Vec<CheckResult> {
    let mut results = Vec::new();
    check(
        ".githooks/pre-commit",
        guardrail_validation_expected,
        guardrail_validation_path_qualified,
        &test_support::StubToolChecker::new(installed),
        &mut results,
    );
    results
}

#[cfg(test)]

mod tests;
