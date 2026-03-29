use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RustCodeFileInput;
use super::parse::{CfgPredicateTruth, find_cfg_attr_lint_policies};

const ID: &str = "RS-CODE-08";

pub fn check(input: &RustCodeFileInput<'_>, results: &mut Vec<CheckResult>) {
    for info in find_cfg_attr_lint_policies(input.ast) {
        if info.truth != CfgPredicateTruth::Unknown {
            continue;
        }
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: if info.kind.attr_name() == "allow" {
                    "conditional cfg_attr allow".to_owned()
                } else {
                    "conditional cfg_attr expect".to_owned()
                },
                message: format!(
                    "Conditional cfg_attr {} for `{}`.",
                    info.kind.attr_name(),
                    info.lint
                ),
                file: Some(input.rel_path.to_owned()),
                line: Some(info.line),
                inventory: false,
            }
            .as_inventory(),
        );
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
#[path = "rs_code_08_cfg_attr_allow_inventory_tests/mod.rs"] // reason: test-only sidecar module wiring
// reason: test-only sidecar module wiring
mod rs_code_08_cfg_attr_allow_inventory_tests;
