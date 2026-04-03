use guardrail3_app_rs_family_clippy_assertions::rs_clippy_19_unknown_keys as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn inventories_when_no_top_level_keys_look_like_managed_typos() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
