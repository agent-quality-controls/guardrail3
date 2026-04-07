use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn inventories_when_disallowed_macros_is_forbidden() {
    let results = run_check(
        include_str!("../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml")
            .replace("disallowed_macros = \"deny\"", "disallowed_macros = \"forbid\"")
            .as_str(),
    );
    let result = results
        .iter()
        .find(|result| result.id() == "RS-CARGO-CONFIG-06")
        .expect("should produce a result for RS-CARGO-CONFIG-06");
    assert_eq!(result.severity(), G3Severity::Info, "forbid is at least as strong as deny and should be accepted");
    assert!(result.inventory(), "successful check should be an inventory result");
}
