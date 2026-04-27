use guardrail3_check_types::{G3CheckResult, G3Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::parse::attrs::find_path_attrs;
use crate::parse::comments::same_line_reason;
use crate::parse::types::CfgPredicateTruth;
use crate::support::CodeSourceRuleInput;

const ID: &str = "g3rs-code/path-attr-with-reason";

pub(crate) fn check(input: &CodeSourceRuleInput<'_>, results: &mut Vec<G3CheckResult>) {
    for info in find_path_attrs(input.source) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse
            || is_exact_owned_test_sidecar(input.rel_path, &info.module_name, &info.path_value)
        {
            continue;
        }

        if info.escapes_parent {
            results.push(G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Error,
                "#[path] escapes parent directory".to_owned(),
                format!(
                    "`#[path = \"{}\"]` on `mod {}` uses a parent-directory segment. Keep module resolution inside the normal Rust module tree.",
                    info.path_value, info.module_name
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
            ));
            continue;
        }

        match same_line_reason(input.content, info.line) {
            Some(reason) if reason_text_is_useful(&reason) => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Warn,
                    "#[path] with reason".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` reason: {reason}",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            Some(reason) => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "#[path] reason too weak".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` needs a specific same-line `// reason:` comment. Weak reason `{reason}` found.",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
            None => {
                results.push(G3CheckResult::new(
                    ID.to_owned(),
                    G3Severity::Error,
                    "#[path] without reason".to_owned(),
                    format!(
                        "`#[path = \"{}\"]` on `mod {}` redirects module resolution. Add a specific same-line `// reason:` comment.",
                        info.path_value, info.module_name
                    ),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                ));
            }
        }
    }
}

fn is_exact_owned_test_sidecar(rel_path: &str, module_name: &str, path_value: &str) -> bool {
    let file_name = rel_path.rsplit('/').next();
    let Some(file_name) = file_name else {
        return false;
    };
    let Some(stem) = file_name.strip_suffix(".rs") else {
        return false;
    };
    if stem == "mod" || stem.is_empty() {
        return false;
    }
    let expected_module_name = format!("{stem}_tests");
    module_name == expected_module_name && path_value == format!("{expected_module_name}/mod.rs")
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
