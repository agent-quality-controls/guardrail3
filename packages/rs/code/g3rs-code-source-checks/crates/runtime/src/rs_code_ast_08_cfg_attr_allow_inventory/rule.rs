use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-08";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.source) {
        if info.truth != CfgPredicateTruth::Unknown {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "conditional cfg_attr allow".to_owned()
            } else {
                "conditional cfg_attr expect".to_owned()
            },
            format!(
                "Conditional cfg_attr {} for `{}`.",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
