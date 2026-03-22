use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_too_many_arguments_threshold_when_correct() {
    let results = check(&root_workspace_tree("too-many-arguments-threshold = 7"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-10" && r.inventory));
}
