use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::find_std_fs_glob_import_lines;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/fs-glob-import";

fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    for line in find_std_fs_glob_import_lines(input.source) {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "std::fs glob import".to_owned(),
            "Direct `use std::fs::*` glob import bypasses clippy method bans.".to_owned(),
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
