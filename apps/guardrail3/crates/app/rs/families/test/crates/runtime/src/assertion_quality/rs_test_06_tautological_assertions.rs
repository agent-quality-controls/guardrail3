use crate::{CheckResult, Severity};

use super::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-06";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.tautological_assert_lines {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "tautological assertion".to_owned(),
            format!(
                "Test `{}` compares only literals in an assertion and proves nothing.",
                input.function.name
            ),
            Some(input.file.rel_path.clone()),
            Some(*line),
            false,
        ));
    }
    if input.function.tautological_assert_lines.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "tautological assertions absent".to_owned(),
                format!(
                    "Test `{}` uses real values in its assertions.",
                    input.function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
                false,
            )
            .as_inventory(),
        );
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_06_tautological_assertions_tests/mod.rs"]
mod rs_test_06_tautological_assertions_tests;
