use guardrail3_app_rs_family_clippy_assertions::rs_clippy_08_reason_quality as assertions;
use test_support::{build_fixture_clippy_toml, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn emits_no_result_when_all_ban_entries_use_reasoned_table_format() {
    let tree = root_workspace_tree(build_fixture_clippy_toml("service", false, true, "", ""));
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_no_results(&results);
}
