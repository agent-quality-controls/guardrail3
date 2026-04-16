use g3rs_deps_config_checks_assertions::rs_deps_config_01_dependencies_allowlisted::rule as assertions;

use super::helpers::{dependency, run_check};

#[test]
fn external_dependency_is_checked_against_allowlist() {
    let results = run_check(true, &["serde"], &[dependency("reqwest")]);

    assertions::assert_has_error(&results, "unauthorized dependency", false);
    assertions::assert_message_contains(
        &results,
        "unauthorized dependency",
        "Dependency `reqwest`",
    );
}
