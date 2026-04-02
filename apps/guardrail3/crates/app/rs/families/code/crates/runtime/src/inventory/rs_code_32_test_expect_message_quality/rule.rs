use guardrail3_domain_report::{CheckResult, Severity};

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_test_expect_calls, line_text};

const ID: &str = "RS-CODE-32";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for issue in find_test_expect_calls(input.ast, input.is_test_root) {
        match issue.message {
            Some(message) if test_expect_message_is_useful(&message) => {}
            Some(message) => results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "test expect message too weak".to_owned(),
    format!(
                    "Test `expect(...)` message must explain the failure clearly. Weak message `{message}` found in `{}`.",
                    line_text(input.content, issue.line)
                ),
    Some(input.rel_path.to_owned()),
    Some(issue.line),
    false,
            )),
            None => results.push(CheckResult::from_parts(
    ID.to_owned(),
    Severity::Error,
    "test expect message must be literal".to_owned(),
    format!(
                    "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `{}`.",
                    line_text(input.content, issue.line)
                ),
    Some(input.rel_path.to_owned()),
    Some(issue.line),
    false,
            )),
        }
    }
}

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





// reason: test-only sidecar module wiring
