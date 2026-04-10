use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{find_forbidden_macros, line_text};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-16";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    for info in find_forbidden_macros(input.source, input.is_test) {
        if info.in_test_context {
            continue;
        }
        let line = info.line;
        let macro_name = info.macro_name;
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        if base_name != "panic" {
            continue;
        }

        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            "panic! macro".to_owned(),
            format!(
                "`panic!()` macro found: {}.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
