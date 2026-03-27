use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::ExceptionCommentInput;

const ID: &str = "RS-CODE-07";

pub fn check(input: &ExceptionCommentInput<'_>, results: &mut Vec<CheckResult>) {
    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "EXCEPTION comment inventory".to_owned(),
            message: format!("Config exception comment: {}", input.line_text),
            file: Some(input.rel_path.to_owned()),
            line: Some(input.line),
            inventory: false,
        }
        .as_inventory(),
    );
}

#[cfg(test)]
#[path = "rs_code_07_exception_comment_inventory_tests/mod.rs"]
mod tests;
