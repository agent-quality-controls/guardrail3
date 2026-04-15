use guardrail3_check_types::{G3CheckResult, G3Severity};

use crate::parse::attrs::{find_crate_level_allows, find_inline_mod_allows};
use crate::support::CodeSourceRuleInput;

const ID: &str = "RS-CODE-SOURCE-01";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for (line, lint) in find_crate_level_allows(input.source) {
        if lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(input, results, line, &lint, None);
    }

    for info in find_inline_mod_allows(input.source) {
        if info.lint == "unused_crate_dependencies" {
            continue;
        }
        push_result(
            input,
            results,
            info.line,
            &info.lint,
            Some(info.module_path.as_str()),
        );
    }
}

fn push_result(
    input: &CodeSourceRuleInput<'_>,
    results: &mut Vec<G3CheckResult>,
    line: usize,
    lint: &str,
    module_path: Option<&str>,
) {
    let severity = if input.is_test {
        G3Severity::Info
    } else {
        G3Severity::Error
    };
    let title = module_path.map_or_else(
        || "crate-level allow".to_owned(),
        |module_path| format!("module-level allow in {module_path}"),
    );
    let message = if input.is_test {
        format!("Crate/module-wide allow for `{lint}` is test-file exempt.")
    } else {
        format!(
            "Crate/module-wide `allow({lint})` suppresses the lint too broadly. Use item-level `#[allow({lint})]` with a `// reason:` comment instead."
        )
    };
    results.push(G3CheckResult::new(
        ID.to_owned(),
        severity,
        title,
        message,
        Some(input.rel_path.to_owned()),
        Some(line),
    ));
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
