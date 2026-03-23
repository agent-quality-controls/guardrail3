use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input, workflow_from_yaml};
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
