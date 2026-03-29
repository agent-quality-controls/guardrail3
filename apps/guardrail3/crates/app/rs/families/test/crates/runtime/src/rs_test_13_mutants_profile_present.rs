use crate::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-13";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.has_mutants_profile {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "profile.mutants configured".to_owned(),
                message: format!(
                    "`{}` defines `[profile.mutants]`.",
                    input.root.cargo_rel_path
                ),
                file: Some(input.root.cargo_rel_path.clone()),
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "profile.mutants missing".to_owned(),
            message: format!(
                "`{}` does not define `[profile.mutants]` for an active mutation-testing setup.",
                input.root.cargo_rel_path
            ),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    super::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_13_mutants_profile_present_tests/mod.rs"]
mod rs_test_13_mutants_profile_present_tests;
