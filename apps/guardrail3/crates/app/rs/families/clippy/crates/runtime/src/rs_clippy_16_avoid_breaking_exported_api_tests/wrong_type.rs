use guardrail3_domain_report::Severity;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_when_avoid_breaking_exported_api_is_not_a_bool() {
    let tree = root_workspace_tree("avoid-breaking-exported-api = \"no\"\n");
    let results = run_for_tests(&tree, "clippy.toml");

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-CLIPPY-16");
    assert_eq!(result.severity, Severity::Warn);
    assert_eq!(result.title, "avoid-breaking-exported-api wrong type");
    assert_eq!(
        result.message,
        "`avoid-breaking-exported-api` must be a bool, found string."
    );
    assert_eq!(result.file.as_deref(), Some("clippy.toml"));
    assert!(!result.inventory);
}
