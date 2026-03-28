use guardrail3_app_rs_family_clippy_assertions::rs_clippy_23_policy_context_parseable as assertions;
use test_support::{dir_entry, project_tree};

use super::super::run_for_tests;

#[test]
fn errors_when_guardrail_policy_file_content_is_missing_from_project_tree() {
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "guardrail3.toml", "clippy.toml"]),
        )],
        vec![
            ("Cargo.toml", "[workspace]\nmembers = []".to_owned()),
            (
                "clippy.toml",
                guardrail3_domain_modules::clippy::build_clippy_toml(
                    "service", false, true, "", "",
                ),
            ),
        ],
    );

    let results = run_for_tests(&tree);
    assertions::assert_guardrail_content_missing(&results);
}
