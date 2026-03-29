use guardrail3_app_rs_family_deny_assertions::rs_deny_14_license_allow_baseline as assertions;

use super::super::{build_fixture_deny_toml, set_private_ignore};

#[test]
fn errors_when_licenses_private_ignore_is_not_true() {
    let results = super::super::run_check(&set_private_ignore(
        &build_fixture_deny_toml("service"),
        false,
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "licenses.private.ignore must be true",
            "`deny.toml` must set `[licenses.private].ignore = true`.",
            "deny.toml",
            false,
        )],
    );
}
