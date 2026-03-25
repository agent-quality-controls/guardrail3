use crate::test_support::{lockfile_facts, lockfile_input};
use guardrail3_domain_report::Severity;

#[test]
fn inventories_clean_gitignore() {
    let facts = lockfile_facts(true, false, Some("service"));
    let input = lockfile_input(&facts);
    let mut results = Vec::new();

    super::super::check(&input, &mut results);

    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.id, "RS-DEPS-10");
    assert_eq!(result.severity, Severity::Info);
    assert!(result.inventory);
    assert_eq!(
        result.message,
        "No relevant `.gitignore` masks `Cargo.lock` for Rust root `.`."
    );
}
