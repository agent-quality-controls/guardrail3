use super::helpers::{dev_dependency, run_check};

#[test]
fn workspace_true_external_dev_dependency_keeps_warn_severity() {
    let results = run_check(true, &["serde"], &[dev_dependency("tempfile")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-03"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Warn)
            && result.message().contains("Dev dependency `tempfile`")
    }));
}

#[test]
fn dev_rule_stays_silent_without_allowlist() {
    let results = run_check(false, &[], &[dev_dependency("tempfile")]);

    assert!(results.is_empty());
}
