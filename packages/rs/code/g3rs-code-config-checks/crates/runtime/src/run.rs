use g3rs_code_types::G3RsCodeConfigChecksInput;
use guardrail3_check_types::G3CheckResult;

pub fn check(input: &G3RsCodeConfigChecksInput) -> Vec<G3CheckResult> {
    let mut results = Vec::new();

    for comment in &input.exception_comments {
        crate::rs_code_config_07_exception_comment_inventory::check(comment, &mut results);
    }

    for file in &input.files {
        crate::rs_code_config_12_unsafe_code_lint::check(file, &mut results);
    }

    results
}
