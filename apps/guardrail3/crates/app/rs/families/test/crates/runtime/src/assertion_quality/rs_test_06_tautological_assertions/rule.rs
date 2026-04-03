use crate::{CheckResult, Severity};

use crate::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-06";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.tautological_assert_lines {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "tautological assertion".to_owned(),
            format!(
                "Test `{}` compares only literals in an assertion and proves nothing. Use values derived from the code under test.",
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

