use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-01";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.source) {
        if lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line, &lint, None);
    }

    for info in find_inline_mod_allows(input.source) {
        if info.lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(
            input,
            results,
            info.line,
            &info.lint,
            Some(info.module_path.as_str()),
        );
    }
}

fn push_result(
    input: &CodeSourceRuleInput<'_>,
    results: &mut Vec<G3CheckResult>,
    line: usize,
    lint: &str,
    module_path: Option<&str>,
) {
    let severity = if input.is_test {
        G3Severity::Info
    } else {
        G3Severity::Error
    };
    let title = module_path.map_or_else(
        || "crate-level allow".to_owned(),
        |module_path| format!("module-level allow in {module_path}"),
    );
    let message = if input.is_test {
        format!("Crate/module-wide allow for `{lint}` is test-file exempt.")
    } else {
        format!(
            "Crate/module-wide `allow({lint})` suppresses the lint too broadly. Use item-level `#[allow({lint})]` with a `// reason:` comment instead."
        )
    };
    results.push(G3CheckResult::new(
        ID.to_owned(),
        severity,
        title,
        message,
        Some(input.rel_path.to_owned()),
        Some(line),
    ));
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
