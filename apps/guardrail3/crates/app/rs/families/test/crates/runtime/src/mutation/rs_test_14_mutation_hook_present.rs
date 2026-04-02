use crate::{CheckResult, Severity};

use crate::inputs::RootTestInput;

const ID: &str = "RS-TEST-14";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if !input.mutation_hook_active {
        results.push(CheckResult::from_parts(
            ID.to_owned(),
            Severity::Error,
            "mutation hook step missing".to_owned(),
            "Active hook surfaces do not contain an executable `cargo mutants` step.".to_owned(),
            Some(input.root.cargo_rel_path.clone()),
            None,
            false,
        ));
    } else {
        for rel_path in input.mutation_hook_files {
            results.push(
                CheckResult::from_parts(
                    ID.to_owned(),
                    Severity::Info,
                    "mutation hook step present".to_owned(),
                    format!("`{rel_path}` contains an executable mutation-testing command."),
                    Some(rel_path.clone()),
                    None,
                    false,
                )
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
pub(crate) fn run_family(root: &std::path::Path) -> Vec<CheckResult> {
    let tree = test_support::walk(root);
    crate::check_test_tree(&tree, &test_support::StubToolChecker::default())
}
#[cfg(test)]
#[path = "rs_test_14_mutation_hook_present_tests/mod.rs"]
mod rs_test_14_mutation_hook_present_tests;
