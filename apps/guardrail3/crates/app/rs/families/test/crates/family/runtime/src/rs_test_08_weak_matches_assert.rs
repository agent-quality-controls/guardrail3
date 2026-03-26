use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-08";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.weak_matches_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "weak matches assertion".to_owned(),
            message: format!(
                "Test `{}` uses `assert!(matches!(...))` with `_` wildcards in payload positions.",
                input.function.name
            ),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_08_weak_matches_assert_tests/mod.rs"]
mod rs_test_08_weak_matches_assert_tests;
