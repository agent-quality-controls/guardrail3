use guardrail3_app_rs_family_clippy_assertions::rs_clippy_18_duplicate_bans as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_for_malformed_managed_ban_sections() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = {}
disallowed-types = [
    123,
    { reason = "missing path" },
    { path = 9, reason = "wrong type" },
]
disallowed-macros = [
    { path = "std::println", reason = ["bad"] },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_malformed_messages(
        &results,
        &[
            "`disallowed-macros[0].reason` must be a string when present, found array.",
            "`disallowed-methods` must be an array, found table.",
            "`disallowed-types[0]` must be a string or table, found integer.",
            "`disallowed-types[1]` must contain a string `path` field.",
            "`disallowed-types[2].path` must be a string, found integer.",
        ],
        "clippy.toml",
    );
}
