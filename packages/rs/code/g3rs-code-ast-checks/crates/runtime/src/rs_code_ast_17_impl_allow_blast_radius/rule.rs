use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_impl_block_allows;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-17";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_impl_block_allows(input.ast) {
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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
