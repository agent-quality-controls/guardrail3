use guardrail3_app_rs_family_hexarch_assertions::rs_hexarch_05_container_not_empty as assertions;
use super::{copy_fixture, empty_dir, remove_dir, write_file};

#[test]
fn files_only_container_is_owned_by_rule_05() {
    let tmp = copy_fixture();
    empty_dir(tmp.path(), "apps/devctl/crates/domain");
    write_file(tmp.path(), "apps/devctl/crates/domain/README.md", "# stray");

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain"),
            file_contains: None,
            title_contains: None,
            message_contains: Some(&["README.md"]),
        }],
    );
}

#[test]
fn missing_container_dir_is_not_owned_by_rule_05() {
    let tmp = copy_fixture();
    remove_dir(tmp.path(), "apps/devctl/crates/domain");

    let results = super::run_family(tmp.path());
    assertions::assert_expected_rule_results(
        &results,
        "",
        &[assertions::ExpectedRuleResult {
            file: Some("apps/devctl/crates/domain"),
            file_contains: None,
            title_contains: None,
            message_contains: None,
        }],
    );
}
