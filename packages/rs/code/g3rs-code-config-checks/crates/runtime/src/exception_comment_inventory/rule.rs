use g3rs_code_types::G3RsCodeExceptionComment;
use guardrail3_check_types::{G3CheckResult, G3Severity};

/// I D const.
const ID: &str = "g3rs-code/exception-comment-inventory";

/// check fn.
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
