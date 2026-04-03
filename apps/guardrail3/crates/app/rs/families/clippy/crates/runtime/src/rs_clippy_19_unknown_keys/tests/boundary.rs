use guardrail3_app_rs_family_clippy_assertions::rs_clippy_19_unknown_keys as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn does_not_warn_for_semantically_related_but_non_typo_unknown_keys() {
    let tree = root_workspace_tree(
        "allow-print-output-in-tests = false\navoid-public-api-breakage = false\n",
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
