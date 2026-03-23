use crate::domain::report::Severity;

use super::super::facts::TestRootKind;
use super::super::test_support::root_input;
use super::check;

#[test]
fn warns_on_exclude_all_pattern() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        false,
        false,
        None,
        Some("exclude_re = [\".*\"]"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].id, "RS-TEST-18");
    assert_eq!(results[0].severity, Severity::Warn);
}

#[test]
fn warns_on_low_timeout_multiplier() {
    let input = root_input(
        TestRootKind::WorkspaceRoot,
        true,
        true,
        false,
        false,
        None,
        Some("timeout_multiplier = 0.5"),
    );
    let mut results = Vec::new();
    check(&input, &mut results);
    assert_eq!(results[0].severity, Severity::Warn);
}
