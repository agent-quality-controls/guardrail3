use guardrail3_app_rs_family_release_assertions::rs_release_06_publish_dry_run_workflow as assertions;

use super::super::{repo_facts, repo_input, workflow_from_yaml};
use super::super::check;

#[test]
fn warns_when_publish_dry_run_only_appears_in_comments_names_or_echo_lines() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-dry-run.yml",
        r#"
name: cargo publish --dry-run notes
jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - name: publish dry run explained
        run: |
          # cargo publish --dry-run
          echo cargo publish --dry-run
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: None,
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn ignores_fake_workflow_and_owns_the_real_publish_dry_run_path() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake.yml",
        r#"
jobs:
  fake:
    runs-on: ubuntu-latest
    steps:
      - run: echo cargo publish --dry-run
"#,
    ));
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/real.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: CARGO_TERM_COLOR=always cargo publish --dry-run
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
            file: Some(".github/workflows/real.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_count_real_publish_dry_run_when_wrapped_by_bash() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/bash-wrapper.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: bash -lc 'cargo publish --dry-run'
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
            file: Some(".github/workflows/bash-wrapper.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_count_absolute_bash_wrappers_and_ignore_non_publish_subcommands() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/real.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: /bin/bash -lc 'cargo publish --dry-run'
"#,
    ));
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake.yml",
        r#"
jobs:
  fake:
    runs-on: ubuntu-latest
    steps:
      - run: cargo metadata publish --dry-run
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
            file: Some(".github/workflows/real.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_not_count_publish_without_dry_run() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/publish-no-dry-run.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: cargo publish --package example
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert!(!assertions::findings(&results).is_empty());
    assertions::assert_rule_results(
        &results,
        &[assertions::ExpectedRuleResult {
            severity: Some(assertions::Severity::Warn),
            file: None,
            inventory: Some(false),
            ..Default::default()
        }],
    );
}
