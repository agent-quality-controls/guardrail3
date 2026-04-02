use guardrail3_domain_report::{CheckResult, Severity};
use guardrail3_reason_policy::reason_text_is_useful;

use crate::inputs::RustCodeFileInput;
use crate::parse::{find_item_lint_policies, same_line_reason};

const ID: &str = "RS-CODE-03";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_item_lint_policies(input.ast) {
        let line = info.line;
        if let Some(reason) = same_line_reason(input.content, line) {
            if reason_text_is_useful(&reason) {
                continue;
            }
            results.push(CheckResult::from_parts(
                ID.to_owned(),
                Severity::Error,
                if info.kind.attr_name() == "allow" {
                    "item-level allow reason too weak".to_owned()
                } else {
                    "item-level expect reason too weak".to_owned()
                },
                format!(
                    "`#[{}({})]` reason must be specific and at least two words. Weak reason `{reason}` found.",
                    info.kind.attr_name(),
                    info.lint
                ),
                Some(input.rel_path.to_owned()),
                Some(line),
                false,
            ));
            continue;
        }
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            if info.kind.attr_name() == "allow" {
                "item-level allow without reason".to_owned()
            } else {
                "item-level expect without reason".to_owned()
            },
            format!(
                "`#[{}({})]` requires `// reason:` on the same line.",
                info.kind.attr_name(),
                info.lint
            ),
            Some(input.rel_path.to_owned()),
            Some(line),
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

// reason: test-only sidecar module wiring
mod rs_code_03_item_level_allow_without_reason_tests;
