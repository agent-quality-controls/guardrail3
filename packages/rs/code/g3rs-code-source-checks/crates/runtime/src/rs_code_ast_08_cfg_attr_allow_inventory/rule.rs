use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_cfg_attr_lint_policies;
use crate::parse::types::CfgPredicateTruth;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-08";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.source) {
        if info.truth != CfgPredicateTruth::Unknown {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Warn,
            if info.kind.attr_name() == "allow" {
                "conditional cfg_attr allow".to_owned()
            } else {
                "conditional cfg_attr expect".to_owned()
            },
            format!(
                "Conditional cfg_attr {} for `{}`.",
                info.kind.attr_name(),
                info.lint
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
