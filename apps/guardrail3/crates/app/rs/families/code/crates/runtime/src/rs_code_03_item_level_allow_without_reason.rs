use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{find_item_lint_policies, same_line_reason};

const ID: &str = "RS-CODE-03";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_item_lint_policies(input.ast) {
        let line = info.line;
        if same_line_reason(input.content, line).is_some() {
            continue;
        }
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Error,
            title: if info.kind.attr_name() == "allow" {
                "item-level allow without reason".to_owned()
            } else {
                "item-level expect without reason".to_owned()
            },
            message: format!(
                "`#[{}({})]` requires `// reason:` on the same line.",
                info.kind.attr_name(),
                info.lint
            ),
            file: Some(input.rel_path.to_owned()),
            line: Some(line),
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
#[path = "rs_code_03_item_level_allow_without_reason_tests/mod.rs"] // reason: test-only sidecar module wiring
// reason: test-only sidecar module wiring
mod rs_code_03_item_level_allow_without_reason_tests;
