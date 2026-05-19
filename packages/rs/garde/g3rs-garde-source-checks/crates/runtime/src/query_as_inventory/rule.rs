use std::collections::BTreeMap;

use guardrail3_check_types::G3CheckResult;

use crate::support::{QueryAsMacroSite, error, warn};

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-garde/query-as-inventory";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(macro_use: &QueryAsMacroSite, results: &mut Vec<G3CheckResult>) {
    if !macro_use.policy_resolved {
        return;
    }
    let selector = format!("{}@L{}", macro_use.macro_name, macro_use.line);
    results.push(
        error(
            ID,
            "sqlx query_as missing reason",
            format!(
                "`{}` bypasses derive-based garde boundary checks. Add a waiver entry in guardrail3-rs.toml with rule = \"{ID}\", subject = \"{}\", selector = \"{selector}\", and a reason for this usage.",
                macro_use.rel_path,
                macro_use.macro_name
            ),
            &macro_use.rel_path,
            Some(macro_use.line),
        )
        .with_selector(selector),
    );
}

/// Implements `check count`.
pub(crate) fn check_count(macro_uses: &[QueryAsMacroSite], results: &mut Vec<G3CheckResult>) {
    let mut counts = BTreeMap::<String, usize>::new();
    for macro_use in macro_uses {
        if !macro_use.policy_resolved {
            continue;
        }
        let entry = counts.entry(macro_use.rel_path.clone()).or_default();
        *entry = entry.saturating_add(1);
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
