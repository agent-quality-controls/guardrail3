use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::count_top_level_use_imports;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-10";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let use_count = count_top_level_use_imports(input.source);
    if use_count <= 20 {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "too many use imports".to_owned(),
        format!(
            "{use_count} top-level use imports (max 20). Reduce imports by consolidating or splitting the file."
        ),
        Some(input.rel_path.to_owned()),
        None,
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
