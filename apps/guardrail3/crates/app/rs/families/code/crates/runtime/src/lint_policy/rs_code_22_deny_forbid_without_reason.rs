use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::inputs::RustCodeFileInput;
use crate::parse::{CfgPredicateTruth, find_deny_forbid_attrs, same_line_reason};

const ID: &str = "RS-CODE-22";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_deny_forbid_attrs(input.ast) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse {
            continue;
        }
        if info.level == "forbid" && info.lint == "unsafe_code" && info.crate_level_inner {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "forbid(unsafe_code)".to_owned(),
                    "`forbid(unsafe_code)` strengthens the local safety boundary.".to_owned(),
                    Some(input.rel_path.to_owned()),
                    Some(info.line),
                    false,
                )
                .as_inventory(),
            );
            continue;
        }
        if let Some(reason) = same_line_reason(input.content, info.line) {
            if reason_text_is_useful(&reason) {
                continue;
            }
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                "#[deny]/#[forbid] reason too weak".to_owned(),
                format!(
                    "`#[{}({})]` reason must be specific and at least two words. Weak reason `{reason}` found.",
                    info.level, info.lint
                ),
                Some(input.rel_path.to_owned()),
                Some(info.line),
                false,
            ));
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "#[deny]/#[forbid] without reason".to_owned(),
            format!(
                "`#[{}({})]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                info.level, info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(info.line),
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    crate::check_test_root(root)
}

#[cfg(test)]
pub(crate) fn copy_fixture() -> test_support::TempDir {
    crate::copy_test_fixture()
}

#[cfg(test)]
pub(crate) fn check_source(rel_path: &str, content: &str, is_test_root: bool) -> Vec<CheckResult> {
    let ast = crate::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = crate::inputs::RustCodeFileInput {
        rel_path,
        content,
        ast: &ast,
        is_test_root,
        profile_name: None,
    };
    let mut results = Vec::new();
    check(&input, &mut results);
    results
}

#[cfg(test)]
#[path = "rs_code_22_deny_forbid_without_reason_tests/mod.rs"]
// reason: test-only sidecar module wiring
mod rs_code_22_deny_forbid_without_reason_tests;
