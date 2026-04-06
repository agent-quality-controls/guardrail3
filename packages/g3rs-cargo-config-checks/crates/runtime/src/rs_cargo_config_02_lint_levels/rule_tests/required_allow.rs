use guardrail3_check_types::G3Severity;

use super::helpers::run_check;

#[test]
fn errors_when_required_allow_lint_is_set_to_deny() {
    let results = run_check(
        include_str!("../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml")
            .replace("redundant_pub_crate = \"allow\"", "redundant_pub_crate = \"deny\"")
            .as_str(),
    );

    let violations: Vec<_> = results
        .iter()
        .filter(|result| {
            result.id() == "RS-CARGO-CONFIG-02"
                && result.severity() == G3Severity::Error
                && result.title().contains("redundant_pub_crate")
        })
        .collect();

    assert_eq!(
        violations.len(),
        1,
        "check 02 should catch required-allow lint set to deny — found {} violations: {violations:?}",
        violations.len()
    );
    assert!(
        violations.first().unwrap().title().contains("deviates from policy"),
        "error should say 'deviates from policy' for a required-allow lint set to a stricter level"
    );
}

#[test]
fn passes_when_required_allow_lint_is_correctly_set() {
    let results = run_check(
        include_str!(
            "../../rs_cargo_config_01_workspace_lints/rule_tests/fixtures/golden_workspace.toml"
        ),
    );

    let violations: Vec<_> = results
        .iter()
        .filter(|result| {
            result.id() == "RS-CARGO-CONFIG-02"
                && result.severity() == G3Severity::Error
                && result.title().contains("redundant_pub_crate")
        })
        .collect();

    assert_eq!(
        violations.len(),
        0,
        "check 02 should not flag redundant_pub_crate when it is correctly set to allow"
    );
}
