#![allow(
    clippy::panic,
    clippy::type_complexity,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::count_top_level_use_imports;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/too-many-use-imports";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_imports(input.source);
    if use_count <= 20 {
        return;
    }
    if crate::support::has_matching_waiver(input, ID, "top-level-use-imports") {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "too many use imports".to_owned(),
        format!(
            "{use_count} top-level use imports (max 20). Reduce imports by consolidating or splitting the file."
        ),
        Some(input.rel_path.to_owned()),
        None,
    ));
}
