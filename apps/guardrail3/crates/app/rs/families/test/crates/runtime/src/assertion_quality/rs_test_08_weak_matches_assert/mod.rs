use crate::{CheckResult, Severity};

use crate::inputs::TestFunctionInput;

const ID: &str = "RS-TEST-08";

pub fn check(input: &TestFunctionInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.function.weak_matches_lines {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "weak matches assertion".to_owned(),
            format!(
                "Test `{}` uses `assert!(matches!(...))` with `_` wildcards in payload positions.",
                input.function.name
            ),
            Some(input.file.rel_path.clone()),
            Some(*line),
            false,
        ));
    }
    if input.function.weak_matches_lines.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "weak matches assertion absent".to_owned(),
                format!(
                    "Test `{}` uses specific payload checks rather than weak wildcard matches.",
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
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod tests;
