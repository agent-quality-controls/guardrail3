use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_foreign_mod_allows;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-20";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_foreign_mod_allows(input.ast) {
        let lint = info.lint;
        let message = if info.via_cfg_attr {
            format!(
                "`#[cfg_attr(..., {}({lint}))]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        } else {
            format!(
                "`#[{}({lint})]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            if info.kind.attr_name() == "allow" {
                "allow on extern block".to_owned()
            } else {
                "expect on extern block".to_owned()
            },
            message,
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
