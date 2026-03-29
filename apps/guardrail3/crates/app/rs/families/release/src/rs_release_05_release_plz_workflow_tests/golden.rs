use guardrail3_app_rs_family_release_assertions::rs_release_05_release_plz_workflow as assertions;

use super::super::{repo_facts, repo_input, workflow_from_yaml};
use super::super::check;

#[test]
fn inventories_real_release_plz_execution_step_from_workflow_yaml() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release.yml",
        r#"
name: release
on:
  push:
    tags:
      - "v*"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: release-plz release-pr
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
            file: Some(".github/workflows/release.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_release_plz_action_when_configured_for_release_command() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-action.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0.5
        with:
          command: release
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
            file: Some(".github/workflows/release-action.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_release_plz_cli_release_command_with_leading_global_flags() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-cli.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz --verbose release
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
            file: Some(".github/workflows/release-cli.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_release_plz_cli_release_pr_with_config_flag() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-config.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz --config .config/release-plz.toml release-pr
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
            file: Some(".github/workflows/release-config.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}
