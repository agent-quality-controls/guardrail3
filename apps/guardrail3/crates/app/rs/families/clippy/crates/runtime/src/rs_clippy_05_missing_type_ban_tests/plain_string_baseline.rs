use guardrail3_app_rs_family_clippy_assertions::rs_clippy_05_missing_type_ban as assertions;
use test_support::{build_fixture_clippy_toml, replace_ban_entry_with_string, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn counts_plain_string_type_entries_for_completeness() {
    let clippy = replace_ban_entry_with_string(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        "disallowed-types",
        "std::collections::HashMap",
    );
    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_service_type_bans(&results, "clippy.toml");
}
