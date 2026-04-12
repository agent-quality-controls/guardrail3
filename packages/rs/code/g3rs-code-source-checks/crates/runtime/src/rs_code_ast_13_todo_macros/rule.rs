use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::{find_forbidden_macros, line_text};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-13";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_forbidden_macros(input.source, input.is_test) {
        let line = info.line;
        let macro_name = info.macro_name;
        let base_name = macro_name.rsplit("::").next().unwrap_or(&macro_name);
        match base_name {
            "todo" | "unimplemented" => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                format!("{macro_name}! macro"),
                format!(
                    "`{macro_name}!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
            )),
            "unreachable" if !info.in_test_context => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "unreachable! macro".to_owned(),
                format!(
                    "`unreachable!()` macro found: {}.",
                    line_text(input.content, line)
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
            )),
            _ => {}
        }
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
