#![allow(
    clippy::panic,
    clippy::type_complexity,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::effective_non_comment_line_count;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/too-many-effective-code-lines";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let effective_lines = effective_non_comment_line_count(input.content);
    if effective_lines <= 500 {
        return;
    }
    if crate::support::has_matching_waiver(input, ID, "effective-code-lines") {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "too many effective code lines".to_owned(),
        format!(
            "{effective_lines} effective code-bearing lines (max 500). Split this file into smaller modules."
        ),
        Some(input.rel_path.to_owned()),
        None,
    ));
}
