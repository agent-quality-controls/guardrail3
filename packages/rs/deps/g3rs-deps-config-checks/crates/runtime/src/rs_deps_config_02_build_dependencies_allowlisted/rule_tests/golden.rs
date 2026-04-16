use g3rs_deps_config_checks_assertions::rs_deps_config_02_build_dependencies_allowlisted::rule as assertions;

use super::helpers::{build_dependency, run_check};

#[test]
fn workspace_true_external_build_dependency_is_checked() {
    let results = run_check(true, &["serde"], &[build_dependency("bindgen")]);

    assertions::assert_has_error(&results, "unauthorized build dependency", false);
    assertions::assert_message_contains(
        &results,
        "unauthorized build dependency",
        "Build dependency `bindgen`",
    );
}

#[test]
fn build_rule_stays_silent_without_allowlist() {
    let results = run_check(false, &[], &[build_dependency("bindgen")]);

    assertions::assert_no_findings(&results);
}
