use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn warns_when_specific_lint_uses_negative_priority() {
    let results = run_check(
        include_str!("../../rs_cargo_01_workspace_lints/tests/fixtures/golden_workspace.toml")
            .replace("unwrap_used = \"deny\"", "unwrap_used = { level = \"deny\", priority = -2 }")
            .as_str(),
    );

    let result = results.iter().find(|result| result.id() == "RS-CARGO-07").unwrap();
    assert_eq!(result.severity(), G3Severity::Warn);
}
