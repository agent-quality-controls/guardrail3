use guardrail3_app_rs_family_clippy_assertions::rs_clippy_06_extra_method_ban as extra_assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn plain_string_user_added_method_bans_still_count_as_extra_inventory() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    "custom::Boundary::call",
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    extra_assertions::assert_messages(
        &results,
        &["Additional method ban `custom::Boundary::call` beyond baseline."],
        "clippy.toml",
    );
}
