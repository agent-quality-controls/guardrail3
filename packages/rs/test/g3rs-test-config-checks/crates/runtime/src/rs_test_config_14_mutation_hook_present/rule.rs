use g3rs_test_types::G3RsTestConfigChecksInput;
use guardrail3_check_types::{G3CheckResult, G3Severity};

const ID: &str = "RS-TEST-14";

pub(crate) fn check(input: &G3RsTestConfigChecksInput, results: &mut Vec<G3CheckResult>) {
    if !input.mutation_hook_active {
        results.push(G3CheckResult::new(
            ID.to_owned(),
            G3Severity::Error,
            "mutation hook step missing".to_owned(),
            "Active hook surfaces do not contain an executable `cargo mutants` step. Add a `cargo mutants` step to the project's hook configuration.".to_owned(),
            Some(input.cargo_rel_path.clone()),
            None,
        ));
        return;
    }

    for rel_path in &input.mutation_hook_files {
        results.push(
            G3CheckResult::new(
                ID.to_owned(),
                G3Severity::Info,
                "mutation hook step present".to_owned(),
                format!("`{rel_path}` contains an executable mutation-testing command."),
                Some(rel_path.clone()),
                None,
            )
            .into_inventory(),
        );
    }
}

#[cfg(test)]
#[path = "rule_tests/mod.rs"]
mod rule_tests;
