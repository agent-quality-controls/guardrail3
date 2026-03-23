use super::super::test_support::{repo_facts, repo_input};
use super::check;

#[test]
fn inventories_release_profile_when_present() {
    let mut facts = repo_facts();
    facts.release_profile_settings = vec!["lto = true".to_owned()];
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert!(results[0].inventory);
}
