use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::HookTestInput;

const ID: &str = "RS-TEST-08";

pub fn check(input: &HookTestInput<'_>, results: &mut Vec<CheckResult>) {
    if input.hook.matching_files.is_empty() {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Warn,
            title: "mutation hook missing".to_owned(),
            message: "No cached pre-commit hook contains a `cargo mutants` step.".to_owned(),
            file: None,
            line: None,
            inventory: false,
        });
    } else {
        for rel_path in &input.hook.matching_files {
            results.push(
                CheckResult {
                    id: ID.to_owned(),
                    severity: Severity::Info,
                    title: "mutation hook present".to_owned(),
                    message: format!("`{rel_path}` contains a mutation-testing command."),
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
#[path = "rs_test_08_mutation_hook_present_tests.rs"]
mod tests;
