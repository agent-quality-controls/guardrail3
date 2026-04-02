use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
const ID: &str = "RS-CODE-25";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    let _ = (input, results);
    let _ = (ID, Severity::Warn);
    // `RS-CODE-33` now owns weak public error-form findings to avoid double-firing.
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
        profile_name: Some("library"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_25_public_result_error_type_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_code_25_public_result_error_type_tests;
