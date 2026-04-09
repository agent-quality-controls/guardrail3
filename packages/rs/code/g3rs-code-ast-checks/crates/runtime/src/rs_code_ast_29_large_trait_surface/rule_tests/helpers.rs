use crate::rs_code_ast_29_large_trait_surface::check;
use crate::support::{CodeSourceRuleInput, G3RsCodeSourceFileAst};
use g3rs_code_ast_checks_types::G3RsSourceFile;
use guardrail3_check_types::G3CheckResult;

pub(super) fn check_source(rel_path: &str, content: &str) -> Vec<G3CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let parsed = G3RsCodeSourceFileAst {
        source_file: G3RsSourceFile {
            rel_path: rel_path.to_owned(),
            content: content.to_owned(),
            is_test: false,
            profile_name: None,
            is_library_root: false,
        },
        ast,
    };
    let input = CodeSourceRuleInput::from(&parsed);
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}
