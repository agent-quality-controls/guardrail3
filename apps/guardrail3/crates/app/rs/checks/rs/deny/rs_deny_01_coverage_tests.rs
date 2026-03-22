use crate::domain::report::Severity;

use super::super::test_support::{
    collected_facts, covered_input, root_coverage_tree, uncovered_input, uncovered_workspace_tree,
};
use super::{check_covered, check_uncovered};

#[test]
fn inventories_covered_roots() {
    let facts = collected_facts(&root_coverage_tree());
    let input = covered_input(&facts, "workspace");
    let mut results = Vec::new();

    check_covered(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-01");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(result.message, "workspace root `workspace` is covered by `deny.toml`.");
    assert_eq!(result.file.as_deref(), Some("deny.toml"));
}

#[test]
fn errors_on_uncovered_root() {
    let facts = collected_facts(&uncovered_workspace_tree());
    let input = uncovered_input(&facts, "workspace");
    let mut results = Vec::new();

    check_uncovered(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DENY-01");
    assert_eq!(result.severity, Severity::Error);
    assert!(!result.inventory);
    assert_eq!(
        result.message,
        "workspace root `workspace` is not covered by any allowed deny config."
    );
    assert_eq!(result.file, None);
}
