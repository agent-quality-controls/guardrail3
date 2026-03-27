use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_for_placeholder_reasons_across_methods_types_and_macros() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "std::env::var", reason = "todo" },
]
disallowed-types = [
    { path = "std::collections::HashMap", reason = "reason" },
]
disallowed-macros = [
    { path = "std::println", reason = "short" },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_placeholder_messages(
        &results,
        &[
            "`std::println` in `disallowed-macros` has a trivial or placeholder `reason`.",
            "`std::collections::HashMap` in `disallowed-types` has a trivial or placeholder `reason`.",
            "`std::env::var` in `disallowed-methods` has a trivial or placeholder `reason`.",
        ],
        "clippy.toml",
    );
}
