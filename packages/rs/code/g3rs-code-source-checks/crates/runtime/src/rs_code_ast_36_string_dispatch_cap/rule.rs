use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_string_dispatch_sites;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-36";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_string_dispatch_sites(input.source, input.is_test) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "string dispatch is too large".to_owned(),
            format!(
                "{} site has {} string-literal branches (cap 10). Replace string dispatch with typed models.",
                info.site_kind, info.string_literal_branch_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
