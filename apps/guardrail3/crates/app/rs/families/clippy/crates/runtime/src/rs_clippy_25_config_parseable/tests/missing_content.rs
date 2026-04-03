use guardrail3_app_rs_family_clippy_assertions::rs_clippy_25_config_parseable as assertions;
use test_support::{dir_entry, project_tree};

use super::helpers::run_for_tests;

#[test]
fn errors_when_allowed_clippy_config_content_is_missing() {
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "clippy.toml"]))],
        vec![("Cargo.toml", "[workspace]\nmembers = []".to_owned())],
    );
    let results = run_for_tests(&tree, "clippy.toml");
    assertions::assert_missing_content(&results, "clippy.toml");
}
