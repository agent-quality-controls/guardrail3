use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_item_allows, same_line_reason};

const ID: &str = "RS-CODE-04";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_item_allows(input.ast) {
        let Some(reason) = same_line_reason(input.content, line) else {
            continue;
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "item-level allow with reason".to_owned(),
            message: format!("#[allow({lint})] reason: {reason}"),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
            inventory: false,
        });
    }
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
#[path = "rs_code_04_item_level_allow_with_reason_tests/mod.rs"]
mod rs_code_04_item_level_allow_with_reason_tests;
