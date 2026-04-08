use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-18";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.ast) {
        if info.truth != CfgPredicateTruth::KnownTrue {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "always-true cfg_attr bypass".to_owned(),
            format!(
                "`#[cfg_attr(..., {}({}))]` is effectively unconditional. Use a direct `#[{}]` with an explicit reason instead.",
                info.kind.attr_name(),
                info.lint,
                info.kind.attr_name()
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
