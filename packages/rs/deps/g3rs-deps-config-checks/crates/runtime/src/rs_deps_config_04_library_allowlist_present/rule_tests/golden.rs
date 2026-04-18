use g3rs_deps_config_checks_assertions::rs_deps_config_04_library_allowlist_present::rule as assertions;

use super::helpers::run_check;
use guardrail3_rs_toml_parser::types::RustProfile;

#[test]
fn inventories_allowlist_when_present() {
    let results = run_check(Some(RustProfile::Library), true);

    assertions::assert_has_info(&results, "dependency allowlist present", true);
    assertions::assert_message_contains(
        &results,
        "dependency allowlist present",
        "has an `allowed_deps` policy",
    );
}

#[test]
fn warns_when_allowlist_missing() {
    let results = run_check(Some(RustProfile::Library), false);

    assertions::assert_has_warn(&results, "dependency allowlist missing", false);
    assertions::assert_message_contains(
        &results,
        "dependency allowlist missing",
        "has no `allowed_deps` policy",
    );
}
