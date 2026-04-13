use g3rs_code_config_checks_types::{G3RsCodeConfigChecksInput, G3RsCodeExceptionComment};
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(exception_comments: Vec<G3RsCodeExceptionComment>) -> Vec<G3CheckResult> {
    crate::run::check(&G3RsCodeConfigChecksInput {
        files: Vec::new(),
        exception_comments,
    })
}

pub(super) fn exception_comment(
    rel_path: &str,
    line: usize,
    text: &str,
) -> G3RsCodeExceptionComment {
    G3RsCodeExceptionComment {
        rel_path: rel_path.to_owned(),
        line,
        text: text.to_owned(),
    }
}
