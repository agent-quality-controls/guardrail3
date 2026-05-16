use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::support::CodeInputFailureRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/input-failures";

/// Runs the rule and appends any findings to `results`.
///
/// # Panics
/// Panics on assertion failure or unexpected input.
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
