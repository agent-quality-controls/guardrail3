use g3rs_deny_config_checks_assertions::wrappers as assertions;

use test_support::run;

use super::helpers;

#[test]
fn stays_quiet_for_canonical_wrapper_policy() {
    let results = run(
        &helpers::service_canonical_bans_toml(),
        Some(guardrail3_rs_toml_parser::types::RustProfile::Service),
        true,
        crate::wrappers::check,
    );

    assertions::assert_no_findings(&results);
}
