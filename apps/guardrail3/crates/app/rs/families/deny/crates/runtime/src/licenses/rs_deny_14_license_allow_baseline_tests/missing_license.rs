use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_14_license_allow_baseline as assertions;

use super::super::{build_fixture_deny_toml, remove_allowed_license};

#[test]
fn errors_when_a_baseline_allowed_license_is_missing() {
    let results = super::super::run_check(&remove_allowed_license(
        &build_fixture_deny_toml("service"),
        "MIT",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "baseline license missing",
            "`deny.toml` is missing allowed license `MIT`.",
            "deny.toml",
            false,
        )],
    );
}
