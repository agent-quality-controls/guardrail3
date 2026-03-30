use crate::{CheckResult, Severity};

use super::inputs::TestFileInput;

const ID: &str = "RS-TEST-04";

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    for line in &input.parsed.ignore_without_reason_lines {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "ignored test lacks reason".to_owned(),
            "`#[ignore]` requires an inline or previous-line `reason:` comment.".to_owned(),
            Some(input.file.rel_path.clone()),
            Some(*line),
            false,
        ));
    }
    if input.parsed.ignore_without_reason_lines.is_empty() {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "ignored tests have reasons".to_owned(),
                "Every `#[ignore]` in this file is paired with a reason comment or attribute."
                    .to_owned(),
                Some(input.file.rel_path.clone()),
                None,
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
#[path = "rs_test_04_ignore_reason_tests/mod.rs"]
mod rs_test_04_ignore_reason_tests;
