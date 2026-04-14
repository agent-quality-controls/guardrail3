use std::collections::BTreeMap;

use guardrail3_check_types::G3CheckResult;
use guardrail3_reason_policy::validate_reason_text;

use crate::support::{QueryAsMacroSite, error, warn};

const ID: &str = "RS-GARDE-SOURCE-04";

pub(crate) fn check(macro_use: &QueryAsMacroSite, results: &mut Vec<G3CheckResult>) {
    if !macro_use.policy_resolved {
        return;
    }
    match macro_use.waiver_reason.as_deref() {
        None => results.push(error(
            ID,
            "sqlx query_as missing reason",
            format!(
                "`{}` bypasses derive-based garde boundary checks without a matching waiver reason. Add a waiver entry in guardrail3-rs.toml for this usage with a reason.",
                macro_use.macro_name
            ),
            &macro_use.rel_path,
            Some(macro_use.line),
        )),
        Some(reason) => match validate_reason_text(reason) {
            Ok(()) => results.push(warn(
                ID,
                "sqlx query_as requires validation review",
                format!(
                    "`{}` bypasses derive-based garde boundary checks with documented reason `{reason}`. Review the target type and ensure validated input handling is explicit.",
                    macro_use.macro_name
                ),
                Some(&macro_use.rel_path),
                Some(macro_use.line),
            )),
            Err(issue) => results.push(error(
                ID,
                "sqlx query_as reason too weak",
                format!(
                    "`{}` bypasses derive-based garde boundary checks with a weak reason: {}. Provide a more specific reason.",
                    macro_use.macro_name,
                    issue.message()
                ),
                &macro_use.rel_path,
                Some(macro_use.line),
            )),
        },
    }
}

pub(crate) fn check_count(macro_uses: &[QueryAsMacroSite], results: &mut Vec<G3CheckResult>) {
    let mut counts = BTreeMap::<String, usize>::new();
    for macro_use in macro_uses {
        if !macro_use.policy_resolved {
            continue;
        }
        *counts.entry(macro_use.rel_path.clone()).or_default() += 1;
    }

    for (rel_path, count) in counts {
        results.push(warn(
            ID,
            "sqlx query_as count",
            format!("`{rel_path}` has {count} sqlx query_as escape hatches."),
            None,
            None,
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod tests;
