use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_large_traits;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-29";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_large_traits(input.source) {
        let severity = if info.method_count > 12 {
            G3Severity::Error
        } else {
            G3Severity::Warn
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            severity,
            "large trait surface".to_owned(),
            format!(
                "Trait `{}` has {} methods (warn above 8, error above 12).",
                info.trait_name, info.method_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
