use super::super::check;
use super::super::{repo_facts, repo_input};

#[test]
fn stays_quiet_when_publish_status_is_absent() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results.is_empty());
}
