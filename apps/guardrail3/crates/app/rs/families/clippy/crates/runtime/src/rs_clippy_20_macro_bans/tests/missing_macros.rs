use guardrail3_app_rs_family_clippy_assertions::rs_clippy_20_macro_bans as assertions;
use test_support::{build_fixture_clippy_toml, remove_ban_path, root_workspace_tree};

use super::super::run_for_tests;

#[test]
fn errors_for_each_missing_required_macro_ban() {
    let clippy = remove_ban_path(
        &remove_ban_path(
            &build_fixture_clippy_toml("service", false, true, "", ""),
            "disallowed-macros",
            "std::eprintln",
        ),
        "disallowed-macros",
        "std::todo",
    );
    let tree = root_workspace_tree(clippy);
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_messages(
        &results,
        &[
            "`eprintln!` is not present in `disallowed-macros`.",
            "`todo!` is not present in `disallowed-macros`.",
        ],
    );
}
