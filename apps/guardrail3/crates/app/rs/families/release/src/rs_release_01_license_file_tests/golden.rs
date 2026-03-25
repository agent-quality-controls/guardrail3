use guardrail3_domain_report::Severity;

use super::super::super::test_support::{repo_facts, repo_input};
use super::super::check;

#[test]
fn inventories_real_license_file_path() {
    let mut facts = repo_facts();
    facts.license_rel_path = Some("LICENSE-APACHE".to_owned());
    let input = repo_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some("LICENSE-APACHE"));
}

#[test]
fn inventories_each_allowed_root_license_name() {
    for rel_path in ["LICENSE", "LICENSE-MIT", "LICENSE.md"] {
        let mut facts = repo_facts();
        facts.license_rel_path = Some(rel_path.to_owned());
        let input = repo_input(&facts);
        let mut results = Vec::new();

        check(&input, &mut results);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, "RS-RELEASE-01");
        assert!(results[0].inventory);
        assert_eq!(results[0].file.as_deref(), Some(rel_path));
    }
}
