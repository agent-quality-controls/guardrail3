use guardrail3_app_rs_family_clippy_assertions::rs_clippy_13_local_policy_root_baseline as assertions;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use test_support::library_workspace_root_tree;

use super::super::run_for_tests;

#[test]
fn ignores_keys_owned_by_specific_rules() {
    let clippy = build_clippy_toml("library", false, true, "", "")
        .replace(
            "avoid-breaking-exported-api = false",
            "avoid-breaking-exported-api = true",
        )
        .replace("allow-dbg-in-tests = false", "allow-dbg-in-tests = true")
        .replace(
            "allow-print-in-tests = false",
            "allow-print-in-tests = true",
        )
        .replace(
            "allow-expect-in-tests = true",
            "allow-expect-in-tests = false",
        )
        .replace(
            "allow-panic-in-tests = false",
            "allow-panic-in-tests = true",
        )
        .replace(
            "allow-unwrap-in-tests = false",
            "allow-unwrap-in-tests = true",
        );
    let tree = library_workspace_root_tree(clippy);
    let results = run_for_tests(&tree, "apps/libsite/clippy.toml");
    assertions::assert_self_contained_inventory(&results, "apps/libsite/clippy.toml");
}
