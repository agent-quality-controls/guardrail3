use guardrail3_domain_report::Severity;

use super::super::test_support::hook_input;
use super::check;

#[test]
fn warns_when_no_hook_matches() {
    let input = hook_input(&[]);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-08");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_matching_hook_files() {
    let input = hook_input(&[".githooks/pre-commit"]);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
    assert_eq!(results[0].file.as_deref(), Some(".githooks/pre-commit"));
}
