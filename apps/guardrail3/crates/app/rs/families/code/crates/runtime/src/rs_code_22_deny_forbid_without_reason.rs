use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{CfgPredicateTruth, find_deny_forbid_attrs, same_line_reason};

const ID: &str = "RS-CODE-22";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_deny_forbid_attrs(input.ast) {
        if info.cfg_truth == CfgPredicateTruth::KnownFalse {
            continue;
        }
        if info.level == "forbid" && info.lint == "unsafe_code" && info.crate_level_inner {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "forbid(unsafe_code)".to_owned(),
                    message: "`forbid(unsafe_code)` strengthens the local safety boundary."
                        .to_owned(),
                    file: Some(input.rel_path.to_owned()),
                    line: Some(info.line),
                    inventory: false,
                }
                .as_inventory(),
            );
            continue;
        }
        if same_line_reason(input.content, info.line).is_some() {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: "#[deny]/#[forbid] without reason".to_owned(),
            message: format!(
                "`#[{}({})]` changes local lint policy without documenting why. Add `// reason:` on the same line.",
                info.level, info.lint
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(info.line),
            inventory: false,
        });
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
    let ast = super::parse::parse_rust_file(content)
        .unwrap_or_else(|error| std::panic::panic_any(format!("valid rust: {error}")));
    let input = super::inputs::RustCodeFileInput {
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
#[path = "rs_code_22_deny_forbid_without_reason_tests/mod.rs"] // reason: test-only sidecar module wiring
mod rs_code_22_deny_forbid_without_reason_tests;
