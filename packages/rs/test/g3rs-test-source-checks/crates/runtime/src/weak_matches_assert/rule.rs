use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::TestFunctionInput;

/// `ID` constant.
const ID: &str = "g3rs-test/weak-matches-assert";

/// `check` function.
pub(crate) fn check(input: &TestFunctionInput<'_>, results: &mut Vec<G3CheckResult>) {
    for line in &input.function.harness.weak_matches_lines {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "weak matches assertion".to_owned(),
            format!(
                "Test `{}` uses `assert!(matches!(...))` with `_` wildcards in payload positions. Match on specific payload values instead of wildcards.",
                input.function.name
            ),
            Some(input.file.rel_path.clone()),
            Some(*line),
        ));
    }
    if input.function.harness.weak_matches_lines.is_empty() {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "weak matches assertion absent".to_owned(),
                format!(
                    "Test `{}` uses specific payload checks rather than weak wildcard matches.",
                    input.function.name
                ),
                Some(input.file.rel_path.clone()),
                Some(input.function.line),
            )
            .into_inventory(),
        );
    }
}
