#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_impl_block_allows;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/impl-allow-blast-radius";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_impl_block_allows(input.source) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            if info.kind.attr_name() == "allow" {
                "blanket impl-level allow".to_owned()
            } else {
                "blanket impl-level expect".to_owned()
            },
            format!(
                "`#[{}({})]` covers an impl block with {} methods. Apply lint suppressions to individual methods instead.",
                info.kind.attr_name(),
                info.lint,
                info.method_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}
