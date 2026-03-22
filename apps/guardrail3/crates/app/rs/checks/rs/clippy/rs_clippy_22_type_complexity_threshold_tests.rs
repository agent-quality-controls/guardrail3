use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_type_complexity_threshold_when_correct() {
    let results = check(&root_workspace_tree("type-complexity-threshold = 75"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-22" && r.inventory));
}
