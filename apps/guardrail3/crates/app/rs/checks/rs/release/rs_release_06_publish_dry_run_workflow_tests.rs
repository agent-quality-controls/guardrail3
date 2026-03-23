use crate::domain::report::Severity;

use super::super::test_support::{repo_facts, repo_input, workflow};
use super::check;

#[test]
fn warns_without_publish_dry_run_step() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_publish_dry_run_step() {
    let mut facts = repo_facts();
    let mut wf = workflow(".github/workflows/ci.yml");
    wf.has_publish_dry_run_step = true;
    facts.workflows.push(wf);
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
