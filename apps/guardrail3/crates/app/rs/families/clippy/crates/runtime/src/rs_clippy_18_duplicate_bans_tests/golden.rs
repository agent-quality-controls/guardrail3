use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn emits_no_result_for_generated_non_duplicate_ban_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(
        results.is_empty(),
        "expected generated ban baseline to avoid duplicate warnings: {results:#?}"
    );
}
