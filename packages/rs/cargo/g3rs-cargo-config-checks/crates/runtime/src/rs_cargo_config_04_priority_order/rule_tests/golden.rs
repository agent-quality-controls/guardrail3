use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_specific_deny_lints_do_not_use_negative_priority() {
    let results = run_check(include_str!("../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml"));
    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-04").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
