use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_07_registry_token as assertions;

use super::helpers::check;
use super::helpers::{repo_facts, repo_input, workflow_from_yaml};

#[test]
fn warns_when_token_exists_but_no_real_release_step_uses_it() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/docs.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - run: cargo fmt --check
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
fn warns_when_token_only_appears_in_values_or_display_text() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-token.yml",
        r#"
name: CARGO_REGISTRY_TOKEN overview
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - env:
          NOTE: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: release-plz release-pr
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
fn should_not_count_token_wiring_on_release_pr_only_step() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-pr-only.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: release-plz release-pr
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
fn should_not_count_token_wiring_on_flagged_release_pr_only_step() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-pr-flagged.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
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
            severity: Some(assertions::Severity::Warn),
            file: None,
            inventory: Some(false),
            ..Default::default()
        }],
    );
}

#[test]
fn should_count_multiline_real_publish_commands_with_inherited_token() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/multiline.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: |
          cargo publish \
            --dry-run
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
            file: Some(".github/workflows/multiline.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn should_not_count_unrelated_with_values_or_empty_token_override() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-token.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0
        env:
          CARGO_REGISTRY_TOKEN: ""
        with:
          command: release-pr
          profile: release
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
