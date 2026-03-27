use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn emits_no_result_when_all_ban_entries_use_reasoned_table_format() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(
        results.is_empty(),
        "expected canonical reasoned ban entries to avoid RS-CLIPPY-08 warnings: {results:#?}"
    );
}
