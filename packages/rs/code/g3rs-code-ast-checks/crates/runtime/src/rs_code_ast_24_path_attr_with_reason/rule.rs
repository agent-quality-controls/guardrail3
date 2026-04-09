use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::{CfgPredicateTruth, find_path_attrs, same_line_reason};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-24";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_path_attrs(input.ast) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse || info.is_test_sidecar_exempt {
            continue;
        }

        if info.escapes_parent {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "#[path] escapes parent directory".to_owned(),
                format!(
                    "`#[path = \"{}\"]` on `mod {}` uses a parent-directory segment. Keep module resolution inside the normal Rust module tree.",
                    info.path_value, info.module_name
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
            ));
            continue;
        }

        match same_line_reason(input.content, info.line) {
            Some(reason) if reason_text_is_useful(&reason) => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "#[path] with reason".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` reason: {reason}",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            Some(reason) => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "#[path] reason too weak".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` needs a specific same-line `// reason:` comment. Weak reason `{reason}` found.",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            None => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "#[path] without reason".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` redirects module resolution. Add a specific same-line `// reason:` comment.",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
