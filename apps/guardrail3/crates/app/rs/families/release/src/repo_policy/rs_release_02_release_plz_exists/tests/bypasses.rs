use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_02_release_plz_exists as assertions;

use super::helpers::check;
use super::helpers::run_tree as run_family;
use super::helpers::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::helpers::{repo_facts, repo_input};

#[test]
fn warns_when_release_plz_file_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn nested_non_root_release_plz_file_does_not_satisfy_repo_root_rule() {
    let root = temp_root("release-plz-nested-non-root");
    let tree = project_tree(
        vec![
            ("", dir_entry(&["examples"], &["Cargo.toml"])),
            ("examples", dir_entry(&["demo"], &[])),
            ("examples/demo", dir_entry(&[], &["release-plz.toml"])),
        ],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
resolver = "2"
"#,
            ),
            ("examples/demo/release-plz.toml", "[workspace]\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: Some("release-plz.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn malformed_root_release_plz_still_satisfies_existence_rule() {
    let root = temp_root("release-plz-root-malformed-exists");
    let tree = project_tree(
        vec![("", dir_entry(&[], &["Cargo.toml", "release-plz.toml"]))],
        vec![
            (
                "Cargo.toml",
                r#"
[workspace]
resolver = "2"
"#,
            ),
            ("release-plz.toml", "[workspace"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("release-plz.toml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
    assertions::assert_related_rule_results(
        &results,
        assertions::INPUT_FAILURE_RULE_ID,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("release-plz.toml"),
            ..Default::default()
        }],
    );
}
