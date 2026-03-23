use crate::domain::report::{CheckResult, Severity};

use super::inputs::TestCoverageInput;

const ID: &str = "RS-TEST-04";

pub fn check(input: &TestCoverageInput<'_>, results: &mut Vec<CheckResult>) {
    if input.coverage.has_any_tests {
        results.push(
            CheckResult {
                id: ID.to_owned(),
                severity: Severity::Info,
                title: "tests exist".to_owned(),
                message: format!(
                    "{} contains at least one Rust test.",
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
            severity: Severity::Error,
            title: "no Rust tests found".to_owned(),
            message: format!(
                "{} does not contain any `#[test]` or `#[tokio::test]` functions.",
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
#[path = "rs_test_04_tests_exist_tests.rs"]
mod tests;
