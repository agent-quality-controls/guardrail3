use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{PublicResultErrorKind, find_public_result_error_types};

const ID: &str = "RS-CODE-25";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    if input.profile_name != Some("library") {
        return;
    }

    for info in find_public_result_error_types(input.ast) {
        let problem = match info.kind {
            PublicResultErrorKind::String => "Result<_, String>",
            PublicResultErrorKind::BoxDynError => "Result<_, Box<dyn Error>>",
        };
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "weak public error type".to_owned(),
            message: format!(
                "Public function `{}` returns `{problem}`. Use a typed error instead.",
                info.fn_name
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
        profile_name: Some("library"),
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_25_public_result_error_type_tests/mod.rs"] // reason: test-only sidecar module wiring
// reason: test-only sidecar module wiring
mod rs_code_25_public_result_error_type_tests;
