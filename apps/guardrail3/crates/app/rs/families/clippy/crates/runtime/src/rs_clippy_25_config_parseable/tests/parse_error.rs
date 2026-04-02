use super::super::run_for_tests;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_25_config_parseable as assertions;

#[test]
fn errors_when_clippy_config_cannot_be_parsed() {
    let tree = test_support::root_workspace_tree("not = [valid");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_parse_error(&results, "clippy.toml", "Failed to parse `clippy.toml`: ");
}
