use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

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
    assertions::assert_weak_reason_messages(
        &results,
        &[
            "`std::println` in `disallowed-macros` has a weak `reason`: reason must be at least 12 characters; found 5.",
            "`std::collections::HashMap` in `disallowed-types` has a weak `reason`: reason must not be a placeholder.",
            "`std::env::var` in `disallowed-methods` has a weak `reason`: reason must not be a placeholder.",
        ],
        "clippy.toml",
    );
    assertions::assert_count_summary(
        &results,
        "`clippy.toml` has 3 clippy ban entries (0 documented, 0 missing reasons, 3 weak reasons).",
    );
}
