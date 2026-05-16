#![allow(
    clippy::panic,
    clippy::type_complexity,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::attrs::find_item_lint_policies;
use crate::parse::comments::same_line_reason;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/item-level-allow-with-reason";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_item_lint_policies(input.source) {
        let line = info.line;
        let Some(reason) = same_line_reason(input.content, line) else {
            continue;
        };
        if !reason_text_is_useful(&reason) {
            continue;
        }
        if crate::support::has_matching_waiver(input, ID, &format!("lint:{}", info.lint)) {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "item-level allow with reason".to_owned()
            } else {
                "item-level expect with reason".to_owned()
            },
            format!(
                "#[{}({})] reason: {reason}",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }
}
