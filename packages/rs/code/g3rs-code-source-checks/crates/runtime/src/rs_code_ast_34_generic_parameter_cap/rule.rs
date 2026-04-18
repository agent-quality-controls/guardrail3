use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::visitors::find_generic_parameter_caps;
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-34";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_generic_parameter_caps(input.source) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "too many generic parameters".to_owned(),
            format!(
                "{} `{}` has {} type/const generic parameters (cap 6; lifetimes do not count). Reduce the number of generic parameters or introduce a trait to abstract them.",
                info.item_kind, info.item_name, info.type_const_param_count
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
