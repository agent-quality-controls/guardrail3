#![allow(
    clippy::panic,
    clippy::type_complexity,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::types::LargeTypeItem;
use crate::parse::visitors::find_large_type_items;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/large-type-inventory";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for item in find_large_type_items(input.source) {
        match item {
            LargeTypeItem::Struct {
                line,
                name,
                field_count,
            } => {
                if crate::support::has_matching_waiver(input, ID, &format!("struct:{name}")) {
                    continue;
                }
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "large type inventory".to_owned(),
                    format!("struct `{name}` has {field_count} fields (inventory threshold 15)."),
                    Some(input.rel_path.to_owned()),
                    Some(line),
                ));
            }
            LargeTypeItem::Enum {
                line,
                name,
                variant_count,
            } => {
                if crate::support::has_matching_waiver(input, ID, &format!("enum:{name}")) {
                    continue;
                }
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "large type inventory".to_owned(),
                    format!("enum `{name}` has {variant_count} items (inventory threshold 20)."),
                    Some(input.rel_path.to_owned()),
                    Some(line),
                ));
            }
        }
    }
}
