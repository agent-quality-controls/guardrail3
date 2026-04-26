use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::attrs::find_item_lint_policies;
use crate::parse::comments::same_line_reason;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-04";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_item_lint_policies(input.source) {
        let line = info.line;
        let Some(reason) = same_line_reason(input.content, line) else {
            continue;
        };
        if !reason_text_is_useful(&reason) {
            continue;
        }
        if crate::support::has_matching_waiver(input, ID, &format!("lint:{}", info.lint)) {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "item-level allow with reason".to_owned()
            } else {
                "item-level expect with reason".to_owned()
            },
            format!(
                "#[{}({})] reason: {reason}",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
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
