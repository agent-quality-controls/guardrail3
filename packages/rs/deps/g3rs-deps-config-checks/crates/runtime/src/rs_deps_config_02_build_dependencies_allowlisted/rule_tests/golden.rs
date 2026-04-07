use super::helpers::{build_dependency, run_check};

#[test]
fn workspace_true_external_build_dependency_is_checked() {
    let results = run_check(true, &["serde"], &[build_dependency("bindgen")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-02"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Build dependency `bindgen`")
    }));
}

#[test]
fn build_rule_stays_silent_without_allowlist() {
    let results = run_check(false, &[], &[build_dependency("bindgen")]);

    assert!(results.is_empty());
}
