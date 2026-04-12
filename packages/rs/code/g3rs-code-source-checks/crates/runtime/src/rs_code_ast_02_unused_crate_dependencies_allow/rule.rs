use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-02";

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

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
