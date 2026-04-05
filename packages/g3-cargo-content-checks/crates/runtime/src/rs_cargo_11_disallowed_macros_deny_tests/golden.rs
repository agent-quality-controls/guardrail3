use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_disallowed_macros_is_denied() {
    let results = run_check(include_str!("../rs_cargo_01_workspace_lints_tests/fixtures/golden_workspace.toml"));
    let result = results.iter().find(|result| result.id() == "RS-CARGO-11").unwrap();
    assert_eq!(result.severity(), G3Severity::Info);
    assert!(result.inventory());
}
