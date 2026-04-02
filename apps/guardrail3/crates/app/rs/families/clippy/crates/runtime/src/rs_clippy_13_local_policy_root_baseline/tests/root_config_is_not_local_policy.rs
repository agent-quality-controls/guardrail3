use guardrail3_app_rs_family_clippy_assertions::rs_clippy_13_local_policy_root_baseline as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn ignores_validation_root_config_because_it_does_not_replace_inherited_policy() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_results(&results);
}
