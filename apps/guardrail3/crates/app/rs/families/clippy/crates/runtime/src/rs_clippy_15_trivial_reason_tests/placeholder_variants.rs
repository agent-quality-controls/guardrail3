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
    assertions::assert_placeholder_messages(
        &results,
        &[
            "`macro::dots` in `disallowed-macros` has a trivial or placeholder `reason`.",
            "`method::empty` in `disallowed-methods` has a trivial or placeholder `reason`.",
            "`method::fixme` in `disallowed-methods` has a trivial or placeholder `reason`.",
            "`method::space` in `disallowed-methods` has a trivial or placeholder `reason`.",
            "`type::later` in `disallowed-types` has a trivial or placeholder `reason`.",
            "`type::tbd` in `disallowed-types` has a trivial or placeholder `reason`.",
        ],
        "clippy.toml",
    );
}
