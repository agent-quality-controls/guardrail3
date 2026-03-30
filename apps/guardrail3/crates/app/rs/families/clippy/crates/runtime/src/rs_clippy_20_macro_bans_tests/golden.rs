use guardrail3_app_rs_family_clippy_assertions::rs_clippy_20_macro_bans as assertions;
use guardrail3_domain_modules::clippy::EXPECTED_MACRO_BANS;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn inventories_every_required_macro_ban_from_generated_baseline() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, EXPECTED_MACRO_BANS, "clippy.toml");
}
