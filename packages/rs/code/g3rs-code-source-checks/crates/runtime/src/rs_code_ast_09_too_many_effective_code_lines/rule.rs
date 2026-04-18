use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::effective_non_comment_line_count;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-09";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test {
        return;
    }

    let effective_lines = effective_non_comment_line_count(input.content);
    if effective_lines <= 500 {
        return;
    }

    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Error,
        "too many effective code lines".to_owned(),
        format!(
            "{effective_lines} effective code-bearing lines (max 500). Split this file into smaller modules."
        ),
        Some(input.rel_path.to_owned()),
        None,
    ));
}

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    let source = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = crate::support::G3RsCodeSourceFileAst {
        source_file: g3rs_code_types::G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        source,
    };
    let input = crate::support::CodeSourceRuleInput::from(&parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
