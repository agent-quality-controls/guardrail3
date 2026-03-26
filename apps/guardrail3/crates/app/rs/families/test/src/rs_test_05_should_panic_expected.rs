use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-05";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    let Some(line) = input.function.should_panic_line else {
        return;
    };
    if input.function.should_panic_has_expected {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "should_panic missing expected string".to_owned(),
        message: format!(
            "Test `{}` uses `#[should_panic]` without `expected = \"...\"`.",
            input.function.name
        ),
        file: Some(input.file.rel_path.clone()),
        line: Some(line),
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_05_should_panic_expected_tests/mod.rs"]
mod tests;
