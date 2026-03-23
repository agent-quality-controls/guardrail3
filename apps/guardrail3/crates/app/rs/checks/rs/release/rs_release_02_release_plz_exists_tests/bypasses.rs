use crate::domain::report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn warns_when_release_plz_file_is_missing() {
    let facts = repo_facts();
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-02");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
}
