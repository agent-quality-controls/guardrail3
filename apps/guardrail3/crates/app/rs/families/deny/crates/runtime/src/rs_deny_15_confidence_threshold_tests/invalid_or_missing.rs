use guardrail3_app_rs_family_deny_assertions::rs_deny_15_confidence_threshold as assertions;

use super::super::ConfigDenyInput;
use super::super::check;
use super::super::{
    build_fixture_deny_toml, config_facts, remove_section_key, set_license_confidence_threshold,
};

#[test]
fn warns_when_confidence_threshold_is_missing_or_invalid() {
    let missing = config_facts(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "licenses",
        "confidence-threshold",
    ));
    let invalid = config_facts(&set_license_confidence_threshold(
        &build_fixture_deny_toml("service"),
        toml::Value::String("high".to_owned()),
    ));

    for config in [&missing, &invalid] {
        let input = ConfigDenyInput { config };
        let mut results = Vec::new();

        check(&input, &mut results);

        assertions::assert_findings(
            &results,
            &[assertions::warn(
                "confidence-threshold missing or invalid",
                "`deny.toml` must set `confidence-threshold >= 0.8`.",
                "deny.toml",
                false,
            )],
        );
    }
}
