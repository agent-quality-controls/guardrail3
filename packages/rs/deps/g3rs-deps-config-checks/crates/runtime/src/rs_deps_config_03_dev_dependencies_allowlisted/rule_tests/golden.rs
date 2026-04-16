use g3rs_deps_config_checks_assertions::rs_deps_config_03_dev_dependencies_allowlisted::rule as assertions;

use super::helpers::{dev_dependency, run_check};

#[test]
fn workspace_true_external_dev_dependency_keeps_warn_severity() {
    let results = run_check(true, &["serde"], &[dev_dependency("tempfile")]);

    assertions::assert_has_warn(&results, "unauthorized dev dependency", false);
    assertions::assert_message_contains(
        &results,
        "unauthorized dev dependency",
        "Dev dependency `tempfile`",
    );
}

#[test]
fn dev_rule_stays_silent_without_allowlist() {
    let results = run_check(false, &[], &[dev_dependency("tempfile")]);

    assertions::assert_no_findings(&results);
}
