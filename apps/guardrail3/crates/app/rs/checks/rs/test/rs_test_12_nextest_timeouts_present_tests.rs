use crate::domain::report::Severity;

use super::super::facts::TestRootKind;
use super::super::test_support::root_input;
use super::check;

#[test]
fn warns_when_tokio_root_lacks_nextest() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        true,
        false,
        None,
        Some("timeout_multiplier = 2.0"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn warns_when_timeouts_missing() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        true,
        true,
        Some("[profile.default]\nslow-timeout = { period = \"60s\" }\n"),
        Some("timeout_multiplier = 2.0"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-12");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn inventories_complete_nextest_timeouts() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        true,
        true,
        Some("[profile.default]\nslow-timeout = { period = \"60s\" }\nleak-timeout = \"100ms\"\n"),
        Some("timeout_multiplier = 2.0"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Info);
    assert!(results[0].inventory);
}
