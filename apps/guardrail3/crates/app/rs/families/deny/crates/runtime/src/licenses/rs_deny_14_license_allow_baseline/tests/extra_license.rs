use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_config_10_license_allow_baseline as assertions;

use super::helpers::{add_allowed_license, build_fixture_deny_toml};

#[test]
fn errors_when_an_extra_allowed_license_is_added() {
    let results = super::helpers::run_check(&add_allowed_license(
        &build_fixture_deny_toml("service"),
        "0BSD",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "unexpected allowed license",
            "`deny.toml` allows unexpected license `0BSD`.",
            "deny.toml",
            false,
        )],
    );
}
