#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::line_text;
use crate::parse::visitors::find_test_expect_calls;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/test-expect-message-quality";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for issue in find_test_expect_calls(input.source, input.is_test) {
        match issue.message {
            Some(message) if test_expect_message_is_useful(&message) => {}
            Some(message) => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test expect message too weak".to_owned(),
                format!(
                    "Test `expect(...)` message must explain the failure clearly. Weak message `{message}` found in `{}`.",
                    line_text(input.content, issue.line)
                ),
                Some(input.rel_path.to_owned()),
                Some(issue.line),
            )),
            None => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "test expect message must be literal".to_owned(),
                format!(
                    "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `{}`.",
                    line_text(input.content, issue.line)
                ),
                Some(input.rel_path.to_owned()),
                Some(issue.line),
            )),
        }
    }
}

/// Implements `test expect message is useful`.
fn test_expect_message_is_useful(message: &str) -> bool {
    let trimmed = message.trim();
    if trimmed.len() < 12 {
        return false;
    }
    let normalized = trimmed.to_ascii_lowercase();
    if normalized.split_whitespace().count() < 3 {
        return false;
    }
    if matches!(
        normalized.as_str(),
        "ok" | "okay"
            | "present"
            | "works"
            | "valid"
            | "value"
            | "error"
            | "failed"
            | "failure"
            | "test"
            | "reason"
            | "tbd"
            | "todo"
            | "fixme"
    ) {
        return false;
    }
    true
}
