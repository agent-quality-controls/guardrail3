use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_expected_deny_is_weakened() {
    let results = run_check(
        include_str!("../../rs_cargo_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml")
            .replace("unwrap_used = \"deny\"", "unwrap_used = \"warn\"")
            .as_str(),
    );

    let result = results.iter().find(|result| result.id() == "RS-CARGO-02").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.title().contains("weakens policy"));
}
