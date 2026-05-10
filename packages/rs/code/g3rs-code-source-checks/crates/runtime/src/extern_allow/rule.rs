#![allow(
    clippy::panic,
    reason = "rule check fns intentionally call std::panic::panic_any to surface unparseable input bubbled up by the upstream parser; this is the documented fail-fast contract for the source-checks family"
)]

use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::find_foreign_mod_allows;
use crate::support::CodeSourceRuleInput;

/// Rule identifier emitted by this check.
const ID: &str = "g3rs-code/extern-allow";

/// Runs the rule and appends any findings to `results`.
pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_foreign_mod_allows(input.source) {
        let lint = info.lint;
        let message = if info.via_cfg_attr {
            format!(
                "`#[cfg_attr(..., {}({lint}))]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        } else {
            format!(
                "`#[{}({lint})]` on an `extern` block hides FFI risk behind a broad suppression.",
                info.kind.attr_name()
            )
        };
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            if info.kind.attr_name() == "allow" {
                "allow on extern block".to_owned()
            } else {
                "expect on extern block".to_owned()
            },
            message,
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
