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

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]

mod rs_code_09_file_length_tests;
