use super::super::{crate_facts, crate_input};
use super::super::check;
use guardrail3_domain_report::Severity;

#[test]
fn inventories_description_for_publishable_crate() {
    let facts = crate_facts("x");
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "RS-PUB-01");
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(
        results[0].file.as_deref(),
        Some("crates/example/Cargo.toml")
    );
    assert!(results[0].title.contains("description"));
    assert!(results[0].message.contains("[package].description"));
}
