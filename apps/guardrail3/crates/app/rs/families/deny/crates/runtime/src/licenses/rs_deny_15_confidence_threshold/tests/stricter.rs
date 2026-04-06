use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_config_11_confidence_threshold as assertions;

use super::helpers::{build_fixture_deny_toml, set_license_confidence_threshold};

#[test]
fn inventories_when_confidence_threshold_is_stricter() {
    let results = super::helpers::run_check(&set_license_confidence_threshold(
        &build_fixture_deny_toml("service"),
        toml::Value::Float(0.9),
    ));

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "confidence-threshold stricter than baseline",
            "`deny.toml` sets `confidence-threshold = 0.9`.",
            "deny.toml",
            true,
        )],
    );
}
