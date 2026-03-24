use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input, workflow_from_yaml};
use super::super::check;

#[test]
fn warns_when_release_plz_only_appears_in_comments_names_or_echo_lines() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake.yml",
        r#"
name: release-plz overview
jobs:
  docs:
    runs-on: ubuntu-latest
    steps:
      - name: mention release-plz in display text
        run: |
          # release-plz release-pr
          echo release-plz release-pr
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}

#[test]
fn ignores_fake_workflow_and_owns_the_real_release_workflow_path() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake.yml",
        r#"
jobs:
  fake:
    runs-on: ubuntu-latest
    steps:
      - run: echo release-plz release-pr
"#,
    ));
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/real.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0.5
        with:
          command: release-pr
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/real.yml")
    );
}

#[test]
fn should_not_count_cargo_release_plz_as_real_release_plz_execution() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/fake-cargo-plugin.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: cargo release-plz
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}

#[test]
fn should_not_count_non_release_plz_subcommands_as_real_release_flow() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/non-release-subcommand.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: release-plz init
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}

#[test]
fn should_count_real_release_plz_when_wrapped_by_bash() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/bash-wrapper.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: bash -lc 'release-plz release-pr'
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(
        results[0].inventory,
        "bash wrapper should still count as real execution"
    );
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/bash-wrapper.yml")
    );
}

#[test]
fn should_count_real_release_plz_in_shell_control_flow_and_env_wrappers() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/control-flow.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - run: if true; then foo=1 /usr/bin/env /bin/bash -lc 'release-plz release-pr'; fi
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/control-flow.yml")
    );
}

#[test]
fn should_not_count_local_release_plz_named_action() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/local-action.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: ./release-plz/action
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}

#[test]
fn should_not_count_release_plz_action_without_release_flow_command() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/non-release-action.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0.5
        with:
          command: set-version
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}

#[test]
fn should_not_count_release_plz_action_without_any_command_binding() {
    let mut facts = repo_facts();
    facts.workflows.push(workflow_from_yaml(
        ".github/workflows/missing-action-command.yml",
        r#"
jobs:
  release:
    runs-on: ubuntu-latest
    steps:
      - uses: release-plz/action@v0.5
"#,
    ));
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-05");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file, None);
}
