use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_crate_level_allows, find_inline_mod_allows};

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
    results.push(CheckResult::from_parts(
        ID.to_owned(),
        Severity::Info,
        "unused_crate_dependencies exemption".to_owned(),
        "unused_crate_dependencies is an approved universal exemption.".to_owned(),
        Some(input.rel_path.to_owned()),
        Some(line),
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

// reason: test-only sidecar module wiring
mod tests;
