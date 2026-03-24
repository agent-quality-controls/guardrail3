use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input, workflow_from_yaml};
use super::super::check;

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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/release.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/step-local.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/release-action.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/release-flags.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-07");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/publish-toolchain.yml")
    );
}
