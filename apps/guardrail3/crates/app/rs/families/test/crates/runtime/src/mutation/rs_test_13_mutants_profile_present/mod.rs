use crate::{CheckResult, Severity};

use crate::inputs::RootTestInput;

const ID: &str = "RS-TEST-13";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.root.has_mutants_profile {
        results.push(
            CheckResult::from_parts(
                ID.to_owned(),
                Severity::Info,
                "profile.mutants configured".to_owned(),
                format!(
                    "`{}` defines `[profile.mutants]`.",
                    input.root.cargo_rel_path
                ),
                Some(input.root.cargo_rel_path.clone()),
                None,
                false,
            )
            .as_inventory(),
        );
    } else {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "profile.mutants missing".to_owned(),
            format!(
                "`{}` does not define `[profile.mutants]` for an active mutation-testing setup.",
                input.root.cargo_rel_path
            ),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]

mod rs_test_13_mutants_profile_present_tests;
