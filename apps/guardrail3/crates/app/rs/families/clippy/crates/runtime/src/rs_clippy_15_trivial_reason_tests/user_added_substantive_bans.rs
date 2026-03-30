use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::super::run_for_tests;

#[test]
fn inventories_cleanly_when_user_added_bans_have_substantive_reasons() {
    let tree = root_workspace_tree(
        r#"
disallowed-methods = [
    { path = "custom::method", reason = "Project-specific boundary must stay on the approved adapter surface." },
]
disallowed-types = [
    { path = "custom::Type", reason = "Avoid leaking crate-local type erasure into downstream boundaries." },
]
disallowed-macros = [
    { path = "custom::macro", reason = "Macro expansion here would hide policy-sensitive control flow." },
]
"#,
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_inventory(&results, "clippy.toml");
}
