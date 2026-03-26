use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestFileInput;

const ID: &str = "RS-TEST-04";

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.parsed.ignore_without_reason_lines {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "ignored test lacks reason".to_owned(),
            message: "`#[ignore]` requires an inline or previous-line `reason:` comment."
                .to_owned(),
            file: Some(input.file.rel_path.clone()),
            line: Some(*line),
            inventory: false,
        });
    }
}

#[cfg(test)]
#[allow(dead_code)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}

#[cfg(test)]
#[allow(dead_code)]
#[allow(dead_code)]
pub(crate) fn run_family_with_tool(
    root: &std::path::Path,
    cargo_mutants_installed: bool,
) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    let checker = if cargo_mutants_installed {
        test_support::StubToolChecker::with_tools(["cargo-mutants"])
    } else {
        test_support::StubToolChecker::default()
    };
    super::check_test_tree(&tree, &checker)
}

#[cfg(test)]
#[path = "rs_test_04_ignore_reason_tests/mod.rs"]
mod rs_test_04_ignore_reason_tests;