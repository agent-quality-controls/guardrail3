#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/unused-crate-dependencies-allow";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.source) {
        if lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line);
    }

    for info in find_inline_mod_allows(input.source) {
        if info.lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, info.line);
    }
}

/// Implements `push result`.
fn push_result(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>, line: usize) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Info,
        "unused_crate_dependencies exemption".to_owned(),
        "unused_crate_dependencies is an approved universal exemption.".to_owned(),
        Some(input.rel_path.to_owned()),
        Some(line),
    ));
}
