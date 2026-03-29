use guardrail3_domain_report::Severity;

use super::super::{repo_facts, repo_input, workflow_from_yaml};
use super::super::check;

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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-06");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/publish.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-06");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/publish-toolchain.yml")
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

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-06");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some(".github/workflows/publish-manifest.yml")
    );
}
