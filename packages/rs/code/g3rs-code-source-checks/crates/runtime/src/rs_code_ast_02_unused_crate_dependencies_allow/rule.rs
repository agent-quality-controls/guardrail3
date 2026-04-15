use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-02";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.source) {
        if lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line);
    }

    for info in find_inline_mod_allows(input.source) {
        if info.lint != "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, info.line);
    }
}

fn push_result(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>, line: usize) {
    results.push(G3CheckResult::new(
        ID.to_owned(),
        G3Severity::Info,
        "unused_crate_dependencies exemption".to_owned(),
        "unused_crate_dependencies is an approved universal exemption.".to_owned(),
        Some(input.rel_path.to_owned()),
        Some(line),
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
