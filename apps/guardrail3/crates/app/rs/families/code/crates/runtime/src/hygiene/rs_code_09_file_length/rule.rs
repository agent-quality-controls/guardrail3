use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::effective_non_comment_line_count;

const ID: &str = "RS-CODE-09";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test_root {
        return;
    }

    let effective_lines = effective_non_comment_line_count(input.content);
    if effective_lines <= 500 {
        return;
    }

    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Error,
        "file too long".to_owned(),
        format!(
            "{effective_lines} effective code-bearing lines (max 500). Long files are hard to review and maintain."
        ),
        Some(input.rel_path.to_owned()),
        None,
        false,
    ));
}

