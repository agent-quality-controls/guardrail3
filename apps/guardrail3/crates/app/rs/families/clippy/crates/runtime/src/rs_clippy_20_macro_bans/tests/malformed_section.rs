use super::helpers::run_for_tests;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_20_macro_bans as assertions;

#[test]
fn reports_malformed_macro_sections_instead_of_clean_inventory() {
    let tree = test_support::root_workspace_tree("disallowed-macros = {}\n");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_malformed_section(
        &results,
        "disallowed-macros section malformed",
        "clippy.toml",
    );
}
