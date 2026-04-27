use g3rs_deps_config_checks_assertions::library_allowlist_present::rule as assertions;

use super::helpers::run_check;
use guardrail3_rs_toml_parser::types::RustProfile;

#[test]
fn stays_quiet_for_service_profile() {
    let results = run_check(Some(RustProfile::Service), false);
    assertions::assert_no_findings(&results);
}

#[test]
fn stays_quiet_when_profile_is_unknown() {
    let results = run_check(None, false);
    assertions::assert_no_findings(&results);
}
