#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::visitors::find_large_traits;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/large-trait-surface";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_large_traits(input.source) {
        let severity = if info.method_count > 12 {
            G3Severity::Error
        } else {
            G3Severity::Warn
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            "large trait surface".to_owned(),
            format!(
                "Trait `{}` has {} methods (warn above 8, error above 12).",
                info.trait_name, info.method_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}
