use super::super::super::test_support::{
    build_fixture_clippy_toml, collected_facts, config_input, root_workspace_tree,
};
use super::super::check;

#[test]
fn ignores_validation_root_config_because_it_does_not_replace_inherited_policy() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(
        results.is_empty(),
        "validation-root config should not be treated as RS-CLIPPY-13 local replacement: {results:#?}"
    );
}
