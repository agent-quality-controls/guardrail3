use crate::domain::report::Severity;
use crate::ports::outbound::CommandRunResult;

use super::super::test_support::{crate_facts, crate_input};
use super::check;

#[test]
fn errors_when_publish_dry_run_fails() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(CommandRunResult {
        success: false,
        stderr: "boom".to_owned(),
    });
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Error);
}

#[test]
fn inventories_when_publish_dry_run_succeeds() {
    let mut facts = crate_facts("x");
    facts.dry_run = Some(CommandRunResult {
        success: true,
        stderr: String::new(),
    });
    let input = crate_input(&facts);
    let mut results = Vec::new();
    check(&input, &mut results);
    assert!(results[0].inventory);
}
