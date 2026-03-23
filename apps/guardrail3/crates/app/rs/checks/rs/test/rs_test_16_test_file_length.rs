use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestFileInput;
use super::parse::effective_non_comment_line_count;

const ID: &str = "RS-TEST-16";
const MAX_LINES: usize = 500;

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.file.is_integration_test_file && !input.file.is_test_sidecar_file {
        return;
    }
    let line_count = effective_non_comment_line_count(input.content);
    if line_count <= MAX_LINES {
        return;
    }
    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Warn,
        title: "test file too long".to_owned(),
        message: format!(
            "`{}` has {} effective lines (threshold: {}).",
            input.file.rel_path, line_count, MAX_LINES
        ),
        file: Some(input.file.rel_path.clone()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_test_16_test_file_length_tests.rs"]
mod tests;
