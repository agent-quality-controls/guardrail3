use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_disallowed_macros_is_not_denied() {
    let results = run_check(
        include_str!("../../rs_cargo_01_workspace_lints/tests/fixtures/golden_workspace.toml")
            .replace("disallowed_macros = \"deny\"", "disallowed_macros = \"warn\"")
            .as_str(),
    );
    let result = results.iter().find(|result| result.id() == "RS-CARGO-11").unwrap();
    assert_eq!(result.severity(), G3Severity::Error);
    assert_eq!(result.title(), "disallowed macros lint weakened");
}
