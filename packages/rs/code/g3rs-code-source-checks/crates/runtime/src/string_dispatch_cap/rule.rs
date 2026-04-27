use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::visitors::find_string_dispatch_sites;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/string-dispatch-cap";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_string_dispatch_sites(input.source, input.is_test) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "string dispatch is too large".to_owned(),
            format!(
                "{} site has {} string-literal branches (cap 10). Replace string dispatch with typed models.",
                info.site_kind, info.string_literal_branch_count
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
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
