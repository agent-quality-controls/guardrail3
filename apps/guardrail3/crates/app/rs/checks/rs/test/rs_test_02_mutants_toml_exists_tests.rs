use crate::domain::report::Severity;

use super::check;
use super::super::facts::TestRootKind;
use super::super::test_support::root_input;

#[test]
fn warns_when_mutants_config_missing() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        false,
        true,
        false,
        false,
        None,
        None,
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-02");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_when_mutants_config_exists() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        false,
        false,
        None,
        Some("timeout_multiplier = 2.0"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
