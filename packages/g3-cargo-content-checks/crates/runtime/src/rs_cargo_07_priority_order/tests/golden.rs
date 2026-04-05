use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_specific_deny_lints_do_not_use_negative_priority() {
    let results = run_check(include_str!("../../rs_cargo_01_workspace_lints/tests/fixtures/golden_workspace.toml"));
    let result = results.iter().find(|result| result.id() == "RS-CARGO-07").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
