use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::line_text;
use crate::parse::visitors::find_test_expect_calls;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/test-expect-message-quality";

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

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let source = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = crate::support::G3RsCodeSourceFileAst {
        source_file: g3rs_code_types::G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        source,
    };
    let input = crate::support::CodeSourceRuleInput::from(&parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
