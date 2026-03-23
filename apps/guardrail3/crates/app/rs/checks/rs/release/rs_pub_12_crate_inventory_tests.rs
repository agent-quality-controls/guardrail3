use super::super::test_support::{repo_facts, repo_input};
use super::check;

#[test]
fn inventories_publishable_and_non_publishable_counts() {
    let mut facts = repo_facts();
    facts.publishable_count = 2;
    facts.non_publishable_count = 1;
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
