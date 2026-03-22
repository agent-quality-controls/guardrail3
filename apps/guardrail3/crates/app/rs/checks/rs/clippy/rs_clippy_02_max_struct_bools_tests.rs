use super::super::check;
use super::super::test_support::root_workspace_tree;

#[test]
fn inventories_max_struct_bools_when_correct() {
    let results = check(&root_workspace_tree("max-struct-bools = 3"));
    assert!(results.iter().any(|r| r.id == "RS-CLIPPY-02" && r.inventory));
}
