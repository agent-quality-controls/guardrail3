use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_06_publish_dry_run_workflow as assertions;

use super::super::check;
use super::super::{repo_facts, repo_input, workflow_from_yaml};

#[test]
fn inventories_real_publish_dry_run_command_from_workflow_yaml() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/publish.yml",
        r#"
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - run: cargo publish --dry-run --package example
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some(".github/workflows/publish.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_publish_dry_run_with_toolchain_override() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/publish-toolchain.yml",
        r#"
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - run: cargo +nightly publish --dry-run
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some(".github/workflows/publish-toolchain.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_publish_dry_run_with_manifest_path_flag() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/publish-manifest.yml",
        r#"
jobs:
  verify:
    runs-on: ubuntu-latest
    steps:
      - run: cargo --manifest-path crates/example/Cargo.toml publish --dry-run
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Info),
            file: Some(".github/workflows/publish-manifest.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
