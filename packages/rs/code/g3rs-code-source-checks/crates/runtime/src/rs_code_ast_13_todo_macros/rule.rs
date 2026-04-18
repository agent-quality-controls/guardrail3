use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::line_text;
use crate::parse::visitors::find_forbidden_macros;
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
