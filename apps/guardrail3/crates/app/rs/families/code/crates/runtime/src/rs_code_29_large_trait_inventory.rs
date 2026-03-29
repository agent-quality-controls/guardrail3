use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::find_large_traits;

const ID: &str = "RS-CODE-29";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") {
        return;
    }

    for info in find_large_traits(input.ast) {
        let severity = if info.method_count > 12 {
            Severity::Error
        } else {
            Severity::Warn
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity,
            title: "large trait surface".to_owned(),
            message: format!(
                "Trait `{}` has {} methods (warn above 8, error above 12).",
                info.trait_name, info.method_count
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
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
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
#[path = "rs_code_29_large_trait_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_29_large_trait_inventory_tests;
