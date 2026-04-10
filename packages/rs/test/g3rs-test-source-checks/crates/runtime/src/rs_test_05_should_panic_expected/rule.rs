use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::TestFunctionInput;

const ID: &str = "RS-TEST-05";

pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    let Some(line) = input.function.should_panic_line else {
        return;
    };
    if input.function.should_panic_has_expected {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "should_panic expected string present".to_owned(),
                format!(
                    "Test `{}` keeps `#[should_panic]` paired with an explicit expected string.",
                    input.function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(line),
            )
            .into_inventory(),
        );
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "should_panic missing expected string".to_owned(),
        format!(
            "Test `{}` uses `#[should_panic]` without `expected = \"...\"`. Add `expected = \"...\"` with the expected panic message.",
            input.function.name
        ),
        Some(input.file.rel_path.clone()),
        Some(line),
    ));
}
