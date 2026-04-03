use guardrail3_app_rs_family_clippy_assertions::rs_clippy_23_policy_context_parseable as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree_with_guardrail};

use super::helpers::run_for_tests;

#[test]
fn inventories_when_guardrail_policy_context_is_parseable() {
    let tree = root_workspace_tree_with_guardrail(
        build_fixture_clippy_toml("service", false, true, "", ""),
        "[profile]\nname = \"service\"\n",
    );

    let results = run_for_tests(&tree);
    assertions::assert_inventory(&results);
}
