use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_too_many_lines_threshold_when_correct() {
    let results = check(&root_workspace_tree("too-many-lines-threshold = 75"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-09" && r.inventory));
}
