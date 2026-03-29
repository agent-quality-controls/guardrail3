use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_test_expect_calls, line_text};

const ID: &str = "RS-CODE-32";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for issue in find_test_expect_calls(input.ast, input.is_test) {
        match issue.message {
            Some(message) if test_expect_message_is_useful(&message) => {}
            Some(message) => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "test expect message too weak".to_owned(),
                message: format!(
                    "Test `expect(...)` message must explain the failure clearly. Weak message `{message}` found in `{}`.",
                    line_text(input.content, issue.line)
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(issue.line),
                inventory: false,
            }),
            None => results.push(CheckResult {
                id: ID.to_owned(),
                severity: Severity::Error,
                title: "test expect message must be literal".to_owned(),
                message: format!(
                    "Test `expect(...)` message must be a useful string literal or `concat!` of string literals, not an indirect expression: `{}`.",
                    line_text(input.content, issue.line)
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(issue.line),
                inventory: false,
            }),
        }
    }
}

fn test_expect_message_is_useful(message: &str) -> bool {
    let trimmed = message.trim();
    if trimmed.len() < 7 {
        return false;
    }
    let normalized = trimmed.to_ascii_lowercase();
    if normalized.split_whitespace().count() < 2 {
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
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<CheckResult> {
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_32_test_expect_message_quality_tests/mod.rs"] // reason: test-only sidecar module wiring
// reason: test-only sidecar module wiring
mod rs_code_32_test_expect_message_quality_tests;
