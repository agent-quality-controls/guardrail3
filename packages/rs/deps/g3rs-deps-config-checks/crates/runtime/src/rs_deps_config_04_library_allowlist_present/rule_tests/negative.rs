use super::helpers::run_check;
use guardrail3_rs_toml_parser::RustProfile;

#[test]
fn stays_quiet_for_service_profile() {
    let results = run_check(Some(RustProfile::Service), false);
    assert!(results.is_empty(), "{results:#?}");
}

#[test]
fn stays_quiet_when_profile_is_unknown() {
    let results = run_check(None, false);
    assert!(results.is_empty(), "{results:#?}");
}
