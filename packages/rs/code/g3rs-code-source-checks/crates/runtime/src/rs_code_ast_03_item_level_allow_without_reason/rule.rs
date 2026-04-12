use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::{find_item_lint_policies, same_line_reason};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-03";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_item_lint_policies(input.source) {
        let line = info.line;
        if let Some(reason) = same_line_reason(input.content, line) {
            if reason_text_is_useful(&reason) {
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                if info.kind.attr_name() == "allow" {
                    "item-level allow reason too weak".to_owned()
                } else {
                    "item-level expect reason too weak".to_owned()
                },
                format!(
                    "`#[{}({})]` reason must be specific and at least two words. Weak reason `{reason}` found.",
                    info.kind.attr_name(),
                    info.lint
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
            ));
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            if info.kind.attr_name() == "allow" {
                "item-level allow without reason".to_owned()
            } else {
                "item-level expect without reason".to_owned()
            },
            format!(
                "`#[{}({})]` requires `// reason:` on the same line.",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
