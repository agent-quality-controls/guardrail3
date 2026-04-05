use super::helpers::run_for_tests;
use guardrail3_app_rs_family_clippy_assertions::rs_clippy_25_config_parseable as assertions;
use test_support::root_workspace_tree;

#[test]
fn errors_when_clippy_config_fails_typed_schema_parsing() {
    let tree = root_workspace_tree("max-struct-bools = \"three\"\n");
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_parse_error(&results, "clippy.toml", "Failed to parse `clippy.toml`:");
}
