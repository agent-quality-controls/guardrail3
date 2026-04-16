use g3rs_deps_config_checks_assertions::rs_deps_config_01_dependencies_allowlisted::rule as assertions;

use super::helpers::{dependency, run_check};

#[test]
fn inventories_allowlisted_runtime_dependency() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assertions::assert_has_info(&results, "dependency allowlisted", true);
    assertions::assert_message_contains(&results, "dependency allowlisted", "Dependency `serde`");
}

#[test]
fn allowlist_check_uses_normalized_package_name() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assertions::assert_has_info(&results, "dependency allowlisted", true);
    assertions::assert_message_contains(&results, "dependency allowlisted", "Dependency `serde`");
}
