use super::helpers::{dependency, run_check};

#[test]
fn workspace_true_external_path_dependency_is_checked() {
    let results = run_check(true, &["serde"], &[dependency("reqwest")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Dependency `reqwest`")
    }));
}
