use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_cognitive_complexity_threshold_when_correct() {
    let results = check(&root_workspace_tree("cognitive-complexity-threshold = 15"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-21" && r.inventory));
}
