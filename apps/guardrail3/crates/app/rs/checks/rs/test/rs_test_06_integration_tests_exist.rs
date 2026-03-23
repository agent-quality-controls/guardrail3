use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestCoverageInput;

const ID: &str = "RS-TEST-06";

pub fn check(input: &TestCoverageInput<'_>, results: &mut Vec<CheckResult>) {
    if input.coverage.integration_test_exists {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "integration tests exist".to_owned(),
                message: format!(
                    "{} has at least one `tests/*.rs` file.",
                    display_root(&input.coverage.root_rel_dir)
                ),
                file: None,
                line: None,
                inventory: false,
            }
            .as_inventory(),
        );
    } else {
        results.push(CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "no integration tests".to_owned(),
            message: format!(
                "{} does not have a `tests/` Rust file.",
                display_root(&input.coverage.root_rel_dir)
            ),
            file: None,
            line: None,
            inventory: false,
        });
    }
}

fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

#[cfg(test)]
#[path = "rs_test_06_integration_tests_exist_tests.rs"]
mod tests;
