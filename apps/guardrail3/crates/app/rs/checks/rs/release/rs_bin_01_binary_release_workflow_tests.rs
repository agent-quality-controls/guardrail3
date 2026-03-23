use super::super::test_support::{crate_facts, crate_input, repo_facts, workflow};
use super::check;

#[test]
fn inventories_binary_release_workflow_when_present() {
    let mut crate_facts = crate_facts("bin");
    crate_facts.is_binary = true;
    let input = crate_input(&crate_facts);
    let mut repo = repo_facts();
    let mut wf = workflow(".github/workflows/binary.yml");
    wf.has_binary_release = true;
    repo.workflows.push(wf);
    let mut results = Vec::new();
    check(&input, &[repo], &mut results);
    assert!(results[0].inventory);
}
