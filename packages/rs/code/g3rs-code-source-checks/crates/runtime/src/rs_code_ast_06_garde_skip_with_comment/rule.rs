use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::comments::{same_line_has_comment, same_line_reason};
use crate::parse::find_garde_skips_with_types;
use crate::parse::types::GardeSkipInfo;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-06";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_garde_skips_with_types(input.source) {
        if info.is_exempt {
            continue;
        }
        if !same_line_has_comment(input.content, info.line) {
            continue;
        }
        if let Some(reason) = same_line_reason(input.content, info.line) {
            if !reason_text_is_useful(&reason) {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "garde(skip) reason too weak".to_owned(),
                    format!(
                        "`#[garde(skip)]` on non-exempt {} reason must be specific and at least two words. Weak reason `{reason}` found.",
                        target_label(&info)
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
                continue;
            }
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Warn,
                "garde(skip) with reason".to_owned(),
                format!(
                    "`#[garde(skip)]` on non-exempt {} reason: {reason}",
                    target_label(&info)
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
            ));
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "garde(skip) comment missing reason".to_owned(),
            format!(
                "`#[garde(skip)]` on non-exempt {} needs `// reason:`.",
                target_label(&info)
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
        ));
    }
}

fn target_label(info: &GardeSkipInfo) -> String {
    if info.is_type_level {
        format!("type `{}`", info.field_name)
    } else {
        format!("field `{}: {}`", info.field_name, info.field_type)
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
