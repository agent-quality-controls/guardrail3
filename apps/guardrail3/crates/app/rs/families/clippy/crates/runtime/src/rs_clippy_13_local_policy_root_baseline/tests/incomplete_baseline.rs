use guardrail3_app_rs_family_clippy_assertions::rs_clippy_13_local_policy_root_baseline as assertions;
use test_support::incomplete_workspace_policy_root_tree;

use super::helpers::run_for_tests;

#[test]
fn errors_when_local_policy_root_drops_managed_sections() {
    let tree = incomplete_workspace_policy_root_tree();
    let results = run_for_tests(&tree, "apps/backend/clippy.toml");
    assertions::assert_incomplete_baseline(
        &results,
        "apps/backend/clippy.toml",
        "`apps/backend/clippy.toml` replaces inherited clippy policy but is incomplete. Missing or wrong managed sections: disallowed-macros, disallowed-methods, disallowed-types, thresholds. Copy the missing sections from the root clippy.toml to ensure the full baseline is present.",
    );
}
