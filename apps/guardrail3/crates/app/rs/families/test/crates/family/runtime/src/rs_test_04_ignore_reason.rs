use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFileInput;

const ID: &str = "RS-TEST-04";

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.parsed.ignore_without_reason_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "ignored test lacks reason".to_owned(),
            message: "`#[ignore]` requires an inline or previous-line `reason:` comment.".to_owned(),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[path = "rs_test_04_ignore_reason_tests/mod.rs"]
mod rs_test_04_ignore_reason_tests;
