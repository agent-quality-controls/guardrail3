use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn emits_no_result_for_generated_service_type_ban_baseline() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(
        results.is_empty(),
        "expected canonical type bans to avoid extra inventory: {results:#?}"
    );
}
