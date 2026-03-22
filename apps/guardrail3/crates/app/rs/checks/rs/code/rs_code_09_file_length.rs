use crate::domain::report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::effective_non_comment_line_count;

const ID: &str = "RS-CODE-09";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test {
        return;
    }

    let effective_lines = effective_non_comment_line_count(input.content);
    if effective_lines <= 500 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "file too long".to_owned(),
        message: format!(
            "{effective_lines} effective lines (max 500). Long files are hard to review and maintain."
        ),
        file: Some(input.rel_path.to_owned()),
        line: None,
        inventory: false,
    });
}

#[cfg(test)]
#[path = "rs_code_09_file_length_tests.rs"]
mod tests;
