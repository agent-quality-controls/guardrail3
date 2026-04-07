use super::helpers::{dependency, run_check};

#[test]
fn normalized_external_workspace_dependency_is_checked() {
    let results = run_check(true, &["serde"], &[dependency("reqwest")]);

    assert!(results.iter().any(|result| {
        result.id() == "RS-DEPS-CONFIG-01"
            && matches!(result.severity(), guardrail3_check_types::G3Severity::Error)
            && result.message().contains("Dependency `reqwest`")
    }));
}
