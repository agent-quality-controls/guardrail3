use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};

const ID: &str = "RS-CODE-18";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.ast) {
        if info.truth != CfgPredicateTruth::KnownTrue {
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "always-true cfg_attr bypass".to_owned(),
            format!(
                "`#[cfg_attr(..., {}({}))]` is effectively unconditional. Use a direct `#[{}]` with an explicit reason instead.",
                info.kind.attr_name(),
                info.lint,
                info.kind.attr_name()
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
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

// reason: test-only sidecar module wiring
mod rs_code_18_always_true_cfg_attr_bypass_tests;
