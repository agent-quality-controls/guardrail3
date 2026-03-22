use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_excessive_nesting_threshold_when_correct() {
    let results = check(&root_workspace_tree("excessive-nesting-threshold = 4"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-11" && r.inventory));
}
