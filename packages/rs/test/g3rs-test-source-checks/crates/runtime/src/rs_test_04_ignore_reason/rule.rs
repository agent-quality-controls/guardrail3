use guardrail3_reason_policy::validate_reason_text;
use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::TestFileInput;

const ID: &str = "RS-TEST-04";

pub(crate) fn check(input: &TestFileInput<'_>, results: &mut Vec<G3CheckResult>) {
    let mut documented = 0usize;
    let mut missing = 0usize;
    let mut weak = 0usize;

    for finding in &input.parsed.ignore_reasons {
        match finding.reason.as_deref() {
            None => {
                missing += 1;
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "ignored test lacks reason".to_owned(),
                    "`#[ignore]` requires an inline or previous-line `reason:` comment.".to_owned(),
                    Some(input.file.rel_path.clone()),
                    Some(finding.line),
                ));
            }
            Some(reason) => match validate_reason_text(reason) {
                Ok(()) => {
                    documented += 1;
                    results.push(G3CheckResult::new(
                        ID.to_owned(),
                        G3Severity::Warn,
                        "ignored test has documented reason".to_owned(),
                        format!("`#[ignore]` uses documented reason: {reason}"),
                        Some(input.file.rel_path.clone()),
                        Some(finding.line),
                    ));
                }
                Err(issue) => {
                    weak += 1;
                    results.push(G3CheckResult::new(
                        ID.to_owned(),
                        G3Severity::Error,
                        "ignored test reason too weak".to_owned(),
                        format!(
                            "`#[ignore]` reason must be specific and meaningful. Weak reason `{reason}` found: {}.",
                            issue.message()
                        ),
                        Some(input.file.rel_path.clone()),
                        Some(finding.line),
                    ));
                }
            },
        }
    }

    let total = documented + missing + weak;
    if total > 0 {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "ignored test count".to_owned(),
            format!(
                "`{}` has {total} ignored tests ({documented} documented, {missing} missing reasons, {weak} weak reasons).",
                input.file.rel_path
            ),
            None,
            None,
        ));
    } else {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "ignored tests have reasons".to_owned(),
                "This file has no ignored tests, or every `#[ignore]` is paired with a strong reason."
                    .to_owned(),
                Some(input.file.rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}
