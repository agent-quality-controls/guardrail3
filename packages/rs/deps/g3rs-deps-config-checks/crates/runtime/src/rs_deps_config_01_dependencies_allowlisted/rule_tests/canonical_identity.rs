use g3rs_deps_config_checks_assertions::rs_deps_config_01_dependencies_allowlisted::rule as assertions;

use super::helpers::{dependency, run_check, target_dependency};

#[test]
fn canonical_dependency_identity_uses_package_name_for_allowlist() {
    let results = run_check(true, &["serde"], &[dependency("serde")]);

    assertions::assert_has_info(&results, "dependency allowlisted", true);
    assertions::assert_message_contains(&results, "dependency allowlisted", "Dependency `serde`");
}

#[test]
fn target_dependency_uses_same_allowlist_contract() {
    let results = run_check(true, &["serde"], &[target_dependency("serde", "cfg(unix)")]);

    assertions::assert_has_info(&results, "dependency allowlisted", true);
}
