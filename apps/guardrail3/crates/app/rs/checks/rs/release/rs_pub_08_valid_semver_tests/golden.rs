use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_valid_semver_and_workspace_inheritance() {
    let valid = crate_facts("x");
    let valid_input = crate_input(&valid);
    let mut valid_results = Vec::new();
    check(&valid_input, &mut valid_results);
    assert_eq!(valid_results[0].severity, Severity::Info);
    assert!(valid_results[0].inventory);

    let mut workspace = crate_facts("x");
    workspace.workspace_version = true;
    let workspace_input = crate_input(&workspace);
    let mut workspace_results = Vec::new();
    check(&workspace_input, &mut workspace_results);
    assert_eq!(workspace_results[0].severity, Severity::Info);
    assert!(workspace_results[0].inventory);
}
