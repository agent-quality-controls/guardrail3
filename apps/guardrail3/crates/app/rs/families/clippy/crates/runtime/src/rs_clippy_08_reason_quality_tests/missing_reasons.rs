use guardrail3_app_rs_family_clippy_assertions::rs_clippy_08_reason_quality as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_for_plain_string_and_missing_reason_entries_across_sections() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = ["std::env::var"]
disallowed-types = [{ path = "std::collections::HashMap" }]
disallowed-macros = [{ path = "std::println", reason = "good enough reason text" }, "std::dbg"]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_reasons(
        &results,
        &[
            "`std::dbg` in `disallowed-macros` must use table format with a `reason` field.",
            "`std::collections::HashMap` in `disallowed-types` must use table format with a `reason` field.",
            "`std::env::var` in `disallowed-methods` must use table format with a `reason` field.",
        ],
        "clippy.toml",
    );
}
