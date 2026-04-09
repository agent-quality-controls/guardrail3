use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{LargeTypeItem, find_large_type_items};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-19";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for item in find_large_type_items(input.ast) {
        match item {
            LargeTypeItem::Struct {
                line,
                name,
                field_count,
            } => {
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
