use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::effective_non_comment_line_count;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-09";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let effective_lines = effective_non_comment_line_count(input.content);
    if effective_lines <= 500 {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "too many effective code lines".to_owned(),
        format!(
            "{effective_lines} effective code-bearing lines (max 500). Split this file into smaller modules."
        ),
        Some(input.rel_path.to_owned()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
