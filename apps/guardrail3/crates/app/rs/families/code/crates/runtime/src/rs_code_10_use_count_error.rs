use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::count_top_level_use_statements;

const ID: &str = "RS-CODE-10";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_statements(input.ast);
    if use_count <= 20 {
        return;
    }

    results.push(CheckResult {
        id: ID.to_owned(),
        severity: Severity::Error,
        title: "too many use statements".to_owned(),
        message: format!("{use_count} top-level use statements (max 20)."),
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
#[path = "rs_code_10_use_count_error_tests/mod.rs"]
mod rs_code_10_use_count_error_tests;
