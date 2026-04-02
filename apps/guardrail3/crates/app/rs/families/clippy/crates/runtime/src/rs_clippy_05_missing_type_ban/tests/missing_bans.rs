use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{build_fixture_clippy_toml, remove_ban_path, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn errors_for_each_missing_required_service_type_ban() {
    let mut clippy = build_fixture_clippy_toml("service", false, true, "", "");
    for path in ["std::collections::HashMap", "std::any::Any"] {
        clippy = remove_ban_path(&clippy, "disallowed-types", path);
    }

    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "`std::collections::HashMap` is not present in `disallowed-types`.",
            "`std::any::Any` is not present in `disallowed-types`.",
        ],
    );
}
