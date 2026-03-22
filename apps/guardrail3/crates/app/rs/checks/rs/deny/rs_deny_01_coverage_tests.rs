use super::super::check;
use super::super::test_support::{root_coverage_tree, uncovered_workspace_tree};

#[test]
fn inventories_covered_roots() {
    let results = check(&root_coverage_tree());
    assert!(results.iter().any(|r| r.id == "RS-DENY-01" && r.inventory && r.message.contains("workspace root `workspace`")));
}

#[test]
fn errors_on_uncovered_root() {
    let results = check(&uncovered_workspace_tree());
    assert!(results.iter().any(|r| r.id == "RS-DENY-01" && !r.inventory && r.message.contains("workspace root `workspace`")));
}
