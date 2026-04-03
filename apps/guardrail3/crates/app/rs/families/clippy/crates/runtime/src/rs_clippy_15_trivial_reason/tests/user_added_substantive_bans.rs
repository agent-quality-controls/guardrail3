use guardrail3_app_rs_family_clippy_assertions::rs_clippy_15_trivial_reason as assertions;
use test_support::root_workspace_tree;

use super::helpers::run_for_tests;

#[test]
fn warns_cleanly_when_user_added_bans_have_substantive_reasons() {
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
    assertions::assert_documented_messages(
        &results,
        &[
            "`custom::Type` in `disallowed-types` uses a documented ban entry with reason `Avoid leaking crate-local type erasure into downstream boundaries.`.",
            "`custom::macro` in `disallowed-macros` uses a documented ban entry with reason `Macro expansion here would hide policy-sensitive control flow.`.",
            "`custom::method` in `disallowed-methods` uses a documented ban entry with reason `Project-specific boundary must stay on the approved adapter surface.`.",
        ],
        "clippy.toml",
    );
    assertions::assert_count_summary(
        &results,
        "`clippy.toml` has 3 clippy ban entries (3 documented, 0 missing reasons, 0 weak reasons).",
    );
}
