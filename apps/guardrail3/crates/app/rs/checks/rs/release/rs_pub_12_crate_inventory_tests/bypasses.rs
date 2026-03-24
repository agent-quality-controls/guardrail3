use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;
use crate::domain::report::Severity;

#[test]
fn inventories_zero_publishable_and_zero_non_publishable_counts() {
    let mut facts = repo_facts();
    facts.publishable_count = 0;
    facts.non_publishable_count = 0;
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
        "Repo has 0 publishable crate(s) and 0 non-publishable crate(s)."
    );
}

#[test]
fn inventories_when_only_non_publishable_crates_exist() {
    let mut facts = repo_facts();
    facts.publishable_count = 0;
    facts.non_publishable_count = 3;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-12");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert_eq!(
        results[0].message,
        "Repo has 0 publishable crate(s) and 3 non-publishable crate(s)."
    );
}

#[test]
fn inventories_when_only_publishable_crates_exist() {
    let mut facts = repo_facts();
    facts.publishable_count = 4;
    facts.non_publishable_count = 0;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-12");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("Cargo.toml"));
    assert_eq!(
        results[0].message,
        "Repo has 4 publishable crate(s) and 0 non-publishable crate(s)."
    );
}
