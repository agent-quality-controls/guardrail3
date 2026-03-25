use guardrail3_domain_report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn inventories_release_plz_file_when_present() {
    let mut facts = repo_facts();
    facts.release_plz_exists = true;
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-02");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("release-plz.toml"));
}
