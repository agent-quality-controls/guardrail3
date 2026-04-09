use g3rs_code_config_checks_types::{G3RsCodeConfigChecksInput, G3RsCodeUnsafeCodeLintFact};
use guardrail3_check_types::G3CheckResult;

pub(super) fn run_check(lints: Vec<G3RsCodeUnsafeCodeLintFact>) -> Vec<G3CheckResult> {
    crate::run::check(&G3RsCodeConfigChecksInput {
        exception_comments: Vec::new(),
        unsafe_code_lints: lints,
    })
}
