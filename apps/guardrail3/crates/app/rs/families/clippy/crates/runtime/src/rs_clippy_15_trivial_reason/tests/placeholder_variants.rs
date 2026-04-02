use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn warns_for_all_placeholder_reason_variants() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "method::empty", reason = "" },
    { path = "method::space", reason = "   " },
    { path = "method::fixme", reason = "fixme" },
]
disallowed-types = [
    { path = "type::later", reason = "fix later" },
    { path = "type::tbd", reason = "tbd" },
]
disallowed-macros = [
    { path = "macro::dots", reason = "..." },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_weak_reason_messages(
        &results,
        &[
            "`macro::dots` in `disallowed-macros` has a weak `reason`: reason must not be a placeholder.",
            "`method::empty` in `disallowed-methods` has a weak `reason`: reason must not be empty.",
            "`method::fixme` in `disallowed-methods` has a weak `reason`: reason must not be a placeholder.",
            "`method::space` in `disallowed-methods` has a weak `reason`: reason must not be empty.",
            "`type::later` in `disallowed-types` has a weak `reason`: reason must not be a placeholder.",
            "`type::tbd` in `disallowed-types` has a weak `reason`: reason must not be a placeholder.",
        ],
        "clippy.toml",
    );
    assertions::assert_count_summary(
        &results,
        "`clippy.toml` has 6 clippy ban entries (0 documented, 0 missing reasons, 6 weak reasons).",
    );
}
