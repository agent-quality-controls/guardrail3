use guardrail3_domain_report::{CheckResult, Severity};

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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> tempfile::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content).expect("valid rust");
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_09_file_length_tests/mod.rs"]
mod rs_code_09_file_length_tests;
