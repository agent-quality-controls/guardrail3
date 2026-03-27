use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_always_true_cfg_attr_allows;

const ID: &str = "RS-CODE-18";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_always_true_cfg_attr_allows(input.ast) {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "always-true cfg_attr bypass".to_owned(),
            message: format!(
                "`#[cfg_attr(..., allow({}))]` is effectively unconditional. Use a direct `#[allow]` with an explicit reason instead.",
                info.lint
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
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
#[path = "rs_code_18_always_true_cfg_attr_bypass_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_18_always_true_cfg_attr_bypass_tests;
