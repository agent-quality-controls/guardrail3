use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::same_line_has_comment;
use crate::parse::find_garde_skips_with_types;
use crate::parse::types::GardeSkipInfo;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/garde-skip-without-comment";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_garde_skips_with_types(input.source) {
        if info.is_exempt {
            continue;
        }
        if same_line_has_comment(input.content, info.line) {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "garde(skip) without comment".to_owned(),
            format!(
                "`#[garde(skip)]` on non-exempt {} requires documentation. Add a `// reason:` comment explaining why validation is skipped.",
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
