use guardrail3_app_rs_family_release_assertions::repo_policy::rs_release_07_registry_token as assertions;

use super::helpers::check;
use super::helpers::{repo_facts, repo_input, workflow_from_yaml};

#[test]
fn inventories_registry_token_when_inherited_into_real_release_step() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz release
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
fn inventories_registry_token_when_wired_on_step_local_publish_dry_run() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/step-local.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: cargo publish --dry-run
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
            file: Some(".github/workflows/step-local.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_registry_token_for_release_plz_action_publish_command() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-action.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0.5
        env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
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
fn inventories_registry_token_for_flagged_release_plz_release_command() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/release-flags.yml",
        r#"
env:
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz --config .config/release-plz.toml release
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
            file: Some(".github/workflows/release-flags.yml"),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn inventories_registry_token_for_toolchain_override_publish_dry_run() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/publish-toolchain.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - env:
          CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
        run: cargo +nightly publish --dry-run
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
