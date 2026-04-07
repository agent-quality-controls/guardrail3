use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_lint_levels_match_policy() {
    let results = run_check(include_str!("../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml"));
    let result = results.iter().find(|result| result.id() == "RS-CARGO-CONFIG-02").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
