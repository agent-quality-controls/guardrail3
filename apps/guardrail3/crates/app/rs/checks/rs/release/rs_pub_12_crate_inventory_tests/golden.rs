use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_publishable_and_non_publishable_counts() {
    let mut facts = repo_facts();
    facts.publishable_count = 2;
    facts.non_publishable_count = 1;
    let input = repo_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-12");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert_eq!(results[0].title, "Crate inventory");
    assert_eq!(
        results[0].message,
        "Repo has 2 publishable crate(s) and 1 non-publishable crate(s)."
    );
}
