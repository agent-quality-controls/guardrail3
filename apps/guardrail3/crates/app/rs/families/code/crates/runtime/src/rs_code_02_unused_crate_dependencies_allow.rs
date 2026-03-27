use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_crate_level_allows;

const ID: &str = "RS-CODE-02";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.ast) {
        if lint != "unused_crate_dependencies" {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "unused_crate_dependencies exemption".to_owned(),
            message: "unused_crate_dependencies is an approved universal exemption.".to_owned(),
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
#[path = "rs_code_02_unused_crate_dependencies_allow_tests/mod.rs"]
mod rs_code_02_unused_crate_dependencies_allow_tests;
