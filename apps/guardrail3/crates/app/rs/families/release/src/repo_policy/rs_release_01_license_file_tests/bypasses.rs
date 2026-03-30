use guardrail3_app_rs_family_release_assertions::rs_release_01_license_file as assertions;

use super::super::check;
use super::super::run_tree as run_family;
use super::super::{StubToolChecker, dir_entry, project_tree, temp_root};
use super::super::{repo_facts, repo_input};

#[test]
fn errors_when_no_license_material_exists() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Error),
            file: Some("Cargo.toml"),
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn errors_when_license_path_is_nested_or_not_whitelisted() {
    for rel_path in ["docs/LICENSE", "LICENSE.txt"] {
        let mut facts = repo_facts();
        facts.license_rel_path = Some(rel_path.to_owned());
        let input = repo_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert!(!assertions::findings(&results).is_empty());
        assertions::assert_rule_results(
            &results,
            &[assertions::ExpectedRuleResult {
                severity: Some(assertions::Severity::Error),
                file: Some("Cargo.toml"),
                inventory: Some(false),
                ..Default::default()
            }],
        );
    }
}

#[test]
fn does_not_error_when_allowed_root_license_exists_beside_distracting_near_misses() {
    let root = temp_root("release-license-root-plus-near-miss");
    let tree = project_tree(
        vec![(
            "",
            dir_entry(&[], &["Cargo.toml", "LICENSE", "LICENSE.txt", "README.md"]),
        )],
        vec![
            (
                "Cargo.toml",
                r#"
[package]
name = "example"
version = "0.1.0"
description = "example"
license = "MIT"
repository = "https://example.com/repo"
"#,
            ),
            ("LICENSE", "canonical license text\n"),
            ("LICENSE.txt", "distracting near miss\n"),
            ("README.md", "# Example\n\nREADME\n"),
        ],
        root,
    );
    let results = run_family(&tree, &StubToolChecker::new(true), false);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some("LICENSE"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
