use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::RootTestInput;

const ID: &str = "RS-TEST-14";

pub fn check(input: &RootTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.mutation_hook_files.is_empty() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "mutation hook step missing".to_owned(),
            message: "Active hook surfaces do not contain an executable `cargo mutants` step."
                .to_owned(),
            file: Some(input.root.cargo_rel_path.clone()),
            line: None,
            inventory: false,
        });
    } else {
        for rel_path in input.mutation_hook_files {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "mutation hook step present".to_owned(),
                    message: format!(
                        "`{rel_path}` contains an executable mutation-testing command."
                    ),
                    file: Some(rel_path.clone()),
                    line: None,
                    inventory: false,
                }
                .as_inventory(),
            );
        }
    }
}

#[cfg(test)]
#[path = "rs_test_14_mutation_hook_present_tests/mod.rs"]
mod rs_test_14_mutation_hook_present_tests;
