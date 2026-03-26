use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-06";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.tautological_assert_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "tautological assertion".to_owned(),
            message: format!(
                "Test `{}` compares only literals in an assertion and proves nothing.",
                input.function.name
            ),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_06_tautological_assertions_tests/mod.rs"]
mod tests;
