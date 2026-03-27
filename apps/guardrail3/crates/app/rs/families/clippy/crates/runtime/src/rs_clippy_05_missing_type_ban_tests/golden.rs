use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn inventories_every_expected_service_type_ban_from_generated_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, "clippy.toml");
}
