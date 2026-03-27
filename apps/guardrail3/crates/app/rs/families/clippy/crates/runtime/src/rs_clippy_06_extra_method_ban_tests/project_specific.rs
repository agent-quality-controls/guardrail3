use guardrail3_app_rs_family_clippy_assertions::rs_clippy_06_extra_method_ban as assertions;
use test_support::{build_fixture_clippy_toml, prepend_ban_path, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn inventories_project_specific_extra_method_bans() {
    let clippy = prepend_ban_path(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        "disallowed-methods",
        "std::io::stdin",
        "good enough reason text",
    );
    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_project_specific(&results, "std::io::stdin", "clippy.toml");
}
