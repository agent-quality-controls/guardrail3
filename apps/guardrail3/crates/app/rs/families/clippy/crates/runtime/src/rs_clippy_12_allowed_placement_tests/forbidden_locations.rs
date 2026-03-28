use guardrail3_app_rs_family_clippy_assertions::rs_clippy_12_allowed_placement as assertions;
use test_support::nested_workspace_member_shadow_tree;

use super::super::run_for_tests;

#[test]
fn reports_every_forbidden_nested_clippy_config_variant() {
    let mut results = Vec::new();

    for file_name in ["clippy.toml", ".clippy.toml"] {
        results.extend(run_for_tests(&nested_workspace_member_shadow_tree(
            file_name,
        )));
    }

    assertions::assert_forbidden_files(
        &results,
        &[
            "workspace/crates/core/clippy.toml",
            "workspace/crates/core/.clippy.toml",
        ],
    );
}
