use guardrail3_domain_report::{CheckResult, Severity};

use super::inputs::TestCoverageInput;

const ID: &str = "RS-TEST-05";

pub fn check(input: &TestCoverageInput<'_>, results: &mut Vec<CheckResult>) {
    let ratio = if input.coverage.public_fn_count == 0 {
        0
    } else {
        input.coverage.test_fn_count.saturating_mul(100) / input.coverage.public_fn_count
    };

    results.push(
        CheckResult {
            id: ID.to_owned(),
            severity: Severity::Info,
            title: "test coverage inventory".to_owned(),
            message: format!(
                "{} has {} public functions and {} test functions ({}%).",
                display_root(&input.coverage.root_rel_dir),
                input.coverage.public_fn_count,
                input.coverage.test_fn_count,
                ratio
            ),
            file: Some(anchor_path(&input.coverage.root_rel_dir)),
            line: None,
            inventory: false,
        }
        .as_inventory(),
    );
}

fn display_root(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "project root".to_owned()
    } else {
        format!("`{rel_dir}`")
    }
}

fn anchor_path(rel_dir: &str) -> String {
    if rel_dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        rel_dir.to_owned()
    }
}

#[cfg(test)]
#[path = "rs_test_05_test_coverage_inventory_tests.rs"]
mod tests;
