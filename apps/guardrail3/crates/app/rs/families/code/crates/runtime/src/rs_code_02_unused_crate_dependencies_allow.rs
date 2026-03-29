use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_crate_level_allows, find_inline_mod_allows};

const ID: &str = "RS-CODE-02";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.ast) {
        if lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line);
    }

    for info in find_inline_mod_allows(input.ast) {
        if info.lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, info.line);
    }
}

fn push_result(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>, line: usize) {
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

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
#[path = "rs_code_02_unused_crate_dependencies_allow_tests/mod.rs"] // reason: test-only sidecar module wiring
// reason: test-only sidecar module wiring
mod rs_code_02_unused_crate_dependencies_allow_tests;
