use crate::{CheckResult, Severity};
use guardrail3_reason_policy::validate_reason_text;

use crate::inputs::TestFileInput;

const ID: &str = "RS-TEST-04";

pub fn check(input: &TestFileInput<'_>, results: &mut Vec<CheckResult>) {
    let mut documented = 0usize;
    let mut missing = 0usize;
    let mut weak = 0usize;

    for finding in &input.parsed.ignore_reasons {
        match finding.reason.as_deref() {
            None => {
                missing += 1;
                results.push(CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Error,
                    "ignored test lacks reason".to_owned(),
                    "`#[ignore]` requires an inline or previous-line `reason:` comment.".to_owned(),
                    Some(input.file.rel_path.clone()),
                    Some(finding.line),
                    false,
                ));
            }
            Some(reason) => match validate_reason_text(reason) {
                Ok(()) => {
                    documented += 1;
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Warn,
                        "ignored test has documented reason".to_owned(),
                        format!("`#[ignore]` uses documented reason: {reason}"),
                        Some(input.file.rel_path.clone()),
                        Some(finding.line),
                        false,
                    ));
                }
                Err(issue) => {
                    weak += 1;
                    results.push(CheckResult::from_parts(
                        ID.to_owned(),
                        Severity::Error,
                        "ignored test reason too weak".to_owned(),
                        format!(
                            "`#[ignore]` reason must be specific and meaningful. Weak reason `{reason}` found: {}.",
                            issue.message()
                        ),
                        Some(input.file.rel_path.clone()),
                        Some(finding.line),
                        false,
                    ));
                }
            },
        }
    }

    let total = documented + missing + weak;
    if total > 0 {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Warn,
            "ignored test count".to_owned(),
            format!(
                "`{}` has {total} ignored tests ({documented} documented, {missing} missing reasons, {weak} weak reasons).",
                input.file.rel_path
            ),
            None,
            None,
            false,
        ));
    } else {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "ignored tests have reasons".to_owned(),
                "This file has no ignored tests, or every `#[ignore]` is paired with a strong reason."
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
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod tests;
