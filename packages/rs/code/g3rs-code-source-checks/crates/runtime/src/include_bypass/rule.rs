use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_include_macros;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/include-bypass";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_include_macros(input.source) {
        match info.macro_name.as_str() {
            "include" if info.build_script_pattern && info.path_traversal => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "include path traversal".to_owned(),
                    "`include!()` build-script pattern appends a path containing `..`.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            "include" if info.build_script_pattern && !info.path_traversal => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "build-script include! inventory".to_owned(),
                    "`include!(concat!(env!(\"OUT_DIR\"), ...))` detected. Review generated-code boundary.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            "include" => results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "include! bypass".to_owned(),
                "`include!()` pulls in Rust code outside the scanned file boundary.".to_owned(),
                Some(input.rel_path.to_owned()),
                Some(info.line),
            )),
            "include_str" | "include_bytes" if info.path_traversal => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "include path traversal".to_owned(),
                    format!("`{}!()` uses a path containing `..`.", info.macro_name),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
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
