use guardrail3_domain_report::Severity;

use super::super::{crate_facts, crate_input};
use super::super::check;

#[test]
fn warns_on_publishable_crate_with_no_release_metadata() {
    let mut facts = crate_facts("internal");
    facts.description_present = false;
    facts.license_present = false;
    facts.repository_present = false;
    let input = crate_input(&facts);
    let mut results = Vec::new();

    check(&input, &mut results);

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-RELEASE-11");
    assert_eq!(results[0].severity, Severity::Warn);
    assert!(!results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("internal"));
    assert!(results[0].message.contains("internal"));
    assert!(results[0].message.contains("description"));
    assert!(results[0].message.contains("license"));
    assert!(results[0].message.contains("repository"));
}
