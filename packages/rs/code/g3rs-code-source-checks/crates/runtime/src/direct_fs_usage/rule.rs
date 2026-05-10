#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use std::collections::BTreeSet;

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::comments::line_text;
use crate::parse::{find_inline_std_fs_call_lines, find_std_fs_import_lines};
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/direct-fs-usage";

/// Implements `is filesystem boundary module`.
fn is_filesystem_boundary_module(rel_path: &str) -> bool {
    rel_path.ends_with("src/fs.rs")
        || rel_path.ends_with("src/fs/mod.rs")
        || rel_path.ends_with("fs/src/lib.rs")
}

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    if input.is_test || is_filesystem_boundary_module(input.rel_path) {
        return;
    }

    let mut import_lines: BTreeSet<usize> =
        find_std_fs_import_lines(input.source).into_iter().collect();
    let call_lines: BTreeSet<usize> = find_inline_std_fs_call_lines(input.source)
        .into_iter()
        .collect();

    for line in import_lines.iter().copied() {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "direct std::fs import".to_owned(),
            format!(
                "Direct `use std::fs` import found: `{}`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
                line_text(input.content, line)
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
        ));
    }

    for line in call_lines {
        if import_lines.remove(&line) {
            continue;
        }
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "direct std::fs call".to_owned(),
            format!(
                "Direct `std::fs::*` call found: `{}`. Route filesystem access through a dedicated `fs` module or crate instead of using `std::fs` directly.",
                line_text(input.content, line)
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
