use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::{CfgPredicateTruth, find_deny_forbid_attrs, same_line_reason};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-22";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_deny_forbid_attrs(input.ast) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse {
            continue;
        }
        if info.level == "forbid" && info.lint == "unsafe_code" && info.crate_level_inner {
            results.push(
                G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Info,
                    "forbid(unsafe_code)".to_owned(),
                    "`forbid(unsafe_code)` strengthens the local safety boundary.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                )
                .into_inventory(),
            );
            continue;
        }
        if let Some(reason) = same_line_reason(input.content, info.line) {
            if reason_text_is_useful(&reason) {
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "#[deny]/#[forbid] reason too weak".to_owned(),
                format!(
                    "`#[{}({})]` reason must be specific and at least two words. Weak reason `{reason}` found.",
                    info.level, info.lint
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
            ));
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "#[deny]/#[forbid] without reason".to_owned(),
            format!(
                "`#[{}({})]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                info.level, info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
