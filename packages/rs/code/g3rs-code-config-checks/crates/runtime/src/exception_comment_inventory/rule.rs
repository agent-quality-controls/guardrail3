use g3rs_code_types::G3RsCodeExceptionComment;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "g3rs-code/exception-comment-inventory";

pub(crate) fn check(comment: &G3RsCodeExceptionComment, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "EXCEPTION comment inventory".to_owned(),
        format!("Config exception comment: {}", comment.text),
        Some(comment.rel_path.clone()),
        Some(comment.line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
