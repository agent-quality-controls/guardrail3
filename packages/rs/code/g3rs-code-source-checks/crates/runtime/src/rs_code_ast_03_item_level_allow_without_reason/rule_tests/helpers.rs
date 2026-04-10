use crate::rs_code_ast_03_item_level_allow_without_reason::check;
use crate::support::{CodeSourceRuleInput, G3RsCodeSourceFileAst};
use g3rs_code_source_checks_types::G3RsSourceFile;
use guardrail3_check_types::G3CheckResult;

pub(super) fn check_source(rel_path: &str, content: &str, is_test: bool) -> Vec<G3CheckResult> {
    let source = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = G3RsCodeSourceFileAst {
        source_file: G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test,
            profile_name: None,
            is_library_root: false,
        },
        source,
    };
    let input = CodeSourceRuleInput::from(&parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
