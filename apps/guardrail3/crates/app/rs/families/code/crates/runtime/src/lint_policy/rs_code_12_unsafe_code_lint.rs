use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::UnsafeCodeLintInput;

const ID: &str = "RS-CODE-12";

pub fn check(input: &UnsafeCodeLintInput<'_>, results: &mut Vec<CheckResult>) {
    match input.lint_level {
        Some("forbid") => results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "unsafe_code = forbid".to_owned(),
                "unsafe_code is set to forbid in workspace lints.".to_owned(),
                Some(input.cargo_rel_path.to_owned()),
                None,
                false,
            )
            .as_inventory(),
        ),
        Some("deny") => results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "unsafe_code should be forbid".to_owned(),
            "unsafe_code = deny can be overridden; use forbid in workspace lints.".to_owned(),
            Some(input.cargo_rel_path.to_owned()),
            None,
            false,
        )),
        _ => {}
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
pub(crate) fn check_unsafe_code_lint(
    cargo_rel_path: &str,
    lint_level: Option<&str>,
) -> Vec<CheckResult> {
    let input = super::inputs::UnsafeCodeLintInput {
        cargo_rel_path,
        lint_level,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_12_unsafe_code_lint_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_12_unsafe_code_lint_tests;
