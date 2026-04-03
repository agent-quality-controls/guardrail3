use guardrail3_app_rs_family_clippy_assertions::rs_clippy_20_macro_bans as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn counts_plain_string_macro_entries_for_completeness() {
    let tree = root_workspace_tree(
        r#"
disallowed-macros = [
    "std::println",
    "std::eprintln",
    "std::dbg",
        "std::todo",
        "std::unimplemented",
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_golden(&results, &assertions::macro_bans(), "clippy.toml");
}
