use super::super::super::test_support::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_valid_semver() {
    let valid = crate_facts("x");
    let valid_input = crate_input(&valid);
    let mut valid_results = Vec::new();
    check(&valid_input, &mut valid_results);

    assert_eq!(valid_results.len(), 1);
    assert_eq!(valid_results[0].id, "RS-PUB-08");
    assert_eq!(valid_results[0].severity, Severity::Info);
    assert!(valid_results[0].inventory);
    assert_eq!(
        valid_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(valid_results[0].title.contains("valid semver"));
    assert!(valid_results[0].message.contains("1.2.3"));
}

#[test]
fn inventories_valid_workspace_version_inheritance() {
    let mut workspace = crate_facts("x");
    workspace.workspace_version = true;
    let workspace_input = crate_input(&workspace);
    let mut workspace_results = Vec::new();
    check(&workspace_input, &mut workspace_results);

    assert_eq!(workspace_results.len(), 1);
    assert_eq!(workspace_results[0].id, "RS-PUB-08");
    assert_eq!(workspace_results[0].severity, Severity::Info);
    assert!(workspace_results[0].inventory);
    assert_eq!(
        workspace_results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(
        workspace_results[0]
            .title
            .contains("version inherited from workspace")
    );
    assert!(
        workspace_results[0]
            .message
            .contains("`version.workspace = true`")
    );
}
