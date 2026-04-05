use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_group_priority_is_wrong() {
    let results = run_check(
        include_str!("../rs_cargo_01_workspace_lints_tests/fixtures/golden_workspace.toml")
            .replace("all = { level = \"deny\", priority = -1 }", "all = { level = \"deny\", priority = 0 }")
            .as_str(),
    );

    let result = results.iter().find(|result| result.id() == "RS-CARGO-02").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert!(result.title().contains("wrong priority"));
}
