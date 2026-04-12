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
#[path = "rule_tests/mod.rs"]
mod rule_tests;
