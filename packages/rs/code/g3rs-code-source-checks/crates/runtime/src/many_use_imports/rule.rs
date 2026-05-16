#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::count_top_level_use_imports;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/many-use-imports";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_imports(input.source);
    if !(16..=20).contains(&use_count) {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "many use imports".to_owned(),
        format!("{use_count} top-level use imports (warn at 16, max 20)."),
        Some(input.rel_path.to_owned()),
        None,
    ));
}
