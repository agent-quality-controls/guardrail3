use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-17";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.weak_matches_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "weak matches assertion".to_owned(),
            message: format!("Test `{}` uses `assert!(matches!(...))` with `_` wildcards.", input.function.name),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_17_weak_matches_assert_tests.rs"]
mod tests;
