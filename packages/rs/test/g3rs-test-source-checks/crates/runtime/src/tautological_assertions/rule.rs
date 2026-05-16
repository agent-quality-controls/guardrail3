use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::TestFunctionInput;

/// `ID` constant.
const ID: &str = "g3rs-test/tautological-assertions";

/// `check` function.
pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    for line in &input.function.harness.tautological_assert_lines {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "tautological assertion".to_owned(),
            format!(
                "Test `{}` compares only literals in an assertion and proves nothing. Use values derived from the code under test.",
                input.function.name
            ),
            Some(input.file.rel_path.clone()),
            Some(*line),
        ));
    }
    if input.function.harness.tautological_assert_lines.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "tautological assertions absent".to_owned(),
                format!(
                    "Test `{}` uses real values in its assertions.",
                    input.function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
            )
            .into_inventory(),
        );
    }
}
