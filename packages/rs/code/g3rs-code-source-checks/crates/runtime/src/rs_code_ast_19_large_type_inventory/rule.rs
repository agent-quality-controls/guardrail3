use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::types::LargeTypeItem;
use crate::parse::visitors::find_large_type_items;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-19";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for item in find_large_type_items(input.source) {
        match item {
            LargeTypeItem::Struct {
                line,
                name,
                field_count,
            } => {
                if crate::support::has_matching_waiver(
                    input,
                    ID,
                    &format!("struct:{name}"),
                ) {
                    continue;
                }
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "large type inventory".to_owned(),
                    format!("struct `{name}` has {field_count} fields (inventory threshold 15)."),
                    Some(input.rel_path.to_owned()),
                    Some(line),
                ));
            }
            LargeTypeItem::Enum {
                line,
                name,
                variant_count,
            } => {
                if crate::support::has_matching_waiver(
                    input,
                    ID,
                    &format!("enum:{name}"),
                ) {
                    continue;
                }
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "large type inventory".to_owned(),
                    format!("enum `{name}` has {variant_count} items (inventory threshold 20)."),
                    Some(input.rel_path.to_owned()),
                    Some(line),
                ));
            }
        }
    }
}

#[cfg(test)]
pub(super) fn check_source(
    rel_path: &str,
    content: &str,
    is_test: bool,
) -> Vec<guardrail3_check_types::G3CheckResult> {
    check_source_with_waivers(rel_path, content, is_test, &[])
}

#[cfg(test)]
pub(super) fn check_source_with_waivers(
    rel_path: &str,
    content: &str,
    is_test: bool,
    waivers: &[(&str, &str, &str, &str)],
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
    let input = crate::support::CodeSourceRuleInput {
        waivers: &waivers
            .iter()
            .map(|(rule, file, selector, reason)| g3rs_code_types::G3RsCodeWaiver {
                rule: (*rule).to_owned(),
                file: (*file).to_owned(),
                selector: (*selector).to_owned(),
                reason: (*reason).to_owned(),
            })
            .collect::<Vec<_>>(),
        ..input
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
