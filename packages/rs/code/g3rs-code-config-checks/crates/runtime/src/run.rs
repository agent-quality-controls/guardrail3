use g3rs_code_config_checks_types::G3RsCodeConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCodeConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for comment in &input.exception_comments {
        crate::rs_code_config_07_exception_comment_inventory::check(
            &comment.rel_path,
            comment.line,
            &comment.line_text,
            &mut results,
        );
    }

    for lint in &input.unsafe_code_lints {
        crate::rs_code_config_12_unsafe_code_lint::check(
            &lint.cargo_rel_path,
            lint.lint_level.as_deref(),
            &mut results,
        );
    }

    results
}
