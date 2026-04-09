use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::count_top_level_use_imports;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-11";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_imports(input.ast);
    if !(16..=20).contains(&use_count) {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Warn,
        "many use imports".to_owned(),
        format!("{use_count} top-level use imports (warn at 16, max 20)."),
        Some(input.rel_path.to_owned()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
