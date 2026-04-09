use g3rs_code_config_checks_types::{G3RsCodeConfigChecksInput, G3RsCodeExceptionCommentFact};
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(comments: Vec<G3RsCodeExceptionCommentFact>) -> Vec<G3CheckResult> {
    crate::run::check(&G3RsCodeConfigChecksInput {
        exception_comments: comments,
        unsafe_code_lints: Vec::new(),
    })
}
