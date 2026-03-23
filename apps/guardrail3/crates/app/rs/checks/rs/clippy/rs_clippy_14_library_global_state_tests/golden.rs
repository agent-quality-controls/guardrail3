use crate::domain::modules::clippy::build_clippy_toml;

use super::super::super::test_support::{
    canonical_clippy_toml, collected_facts, config_input, published_library_package_root_tree,
    root_workspace_tree,
};
use super::super::check;

#[test]
fn emits_no_result_for_non_library_profile() {
    let tree = root_workspace_tree(canonical_clippy_toml());
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.is_empty());
}

#[test]
fn emits_no_result_when_library_profile_keeps_global_state_bans() {
    let tree =
        published_library_package_root_tree(build_clippy_toml("library", false, true, "", ""));
    let facts = collected_facts(&tree);
    let mut results = Vec::new();

    check(&config_input(&facts, "clippy.toml"), &mut results);

    assert!(results.is_empty());
}
