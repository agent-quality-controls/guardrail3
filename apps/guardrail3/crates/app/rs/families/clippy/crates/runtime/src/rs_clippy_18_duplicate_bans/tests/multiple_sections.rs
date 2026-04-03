use guardrail3_app_rs_family_clippy_assertions::rs_clippy_18_duplicate_bans as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn warns_once_per_duplicate_path_per_section() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "good enough reason text" },
    { path = "std::env::var", reason = "another good enough reason text" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "good enough reason text" },
    { path = "std::collections::HashMap", reason = "another good enough reason text" },
]
disallowed-macros = [
    { path = "std::println", reason = "good enough reason text" },
    { path = "std::println", reason = "another good enough reason text" },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_messages(
        &results,
        &[
            "`std::println` appears 2 times in `disallowed-macros`. Remove the duplicate entries.",
            "`std::collections::HashMap` appears 2 times in `disallowed-types`. Remove the duplicate entries.",
            "`std::env::var` appears 2 times in `disallowed-methods`. Remove the duplicate entries.",
        ],
        "clippy.toml",
    );
}
