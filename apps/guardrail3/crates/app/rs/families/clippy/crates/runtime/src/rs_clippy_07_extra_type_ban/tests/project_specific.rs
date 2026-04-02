use guardrail3_app_rs_family_clippy_assertions::rs_clippy_07_extra_type_ban as assertions;
use test_support::{build_fixture_clippy_toml, prepend_ban_path, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn inventories_project_specific_extra_type_bans() {
    let clippy = prepend_ban_path(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        "disallowed-types",
        "std::sync::Arc",
        "good enough reason text",
    );
    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_project_specific(&results, "std::sync::Arc");
}
