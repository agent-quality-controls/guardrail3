use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::CodeInputFailureRuleInput;

const ID: &str = "RS-CODE-SOURCE-30";

pub(crate) fn check(input: &CodeInputFailureRuleInput, results: &mut Vec<G3CheckResult>) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "code-family input failure".to_owned(),
        input.message.clone(),
        Some(input.rel_path.clone()),
        None,
    ));
}

#[cfg(test)]
pub(super) fn check_broken_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
    is_shared_crate: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let input = g3rs_code_types::G3RsCodeSourceChecksInput {
        source_file: g3rs_code_types::G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        parsed_source: g3rs_code_types::G3RsCodeParsedSourceState::Invalid {
            message: "Failed to parse Rust source file: fixture parse failure".to_owned(),
        },
        is_shared_crate,
        waivers: Vec::new(),
    };
    let parse_failure = crate::support::rule_input(&input)
        .err()
        .expect("source fixture should route the prebound parse failure");
    let mut results = Vec::new();
    check(&parse_failure, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
