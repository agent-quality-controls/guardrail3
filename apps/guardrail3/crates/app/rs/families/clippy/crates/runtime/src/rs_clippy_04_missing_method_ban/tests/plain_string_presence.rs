use guardrail3_app_rs_family_clippy_assertions::rs_clippy_04_missing_method_ban as missing_assertions;
use test_support::{build_fixture_clippy_toml, replace_ban_entry_with_string, root_workspace_tree};

use super::helpers::run_for_tests;

#[test]
fn plain_string_entries_still_count_for_completeness_but_fail_reason_quality() {
    let clippy = replace_ban_entry_with_string(
        &build_fixture_clippy_toml("service", false, true, "", ""),
        "disallowed-methods",
        "std::env::var",
    );
    let tree = root_workspace_tree(clippy);

    let completeness_results = run_for_tests(&tree, "clippy.toml");
    missing_assertions::assert_service_method_bans(&completeness_results, "clippy.toml");
}
