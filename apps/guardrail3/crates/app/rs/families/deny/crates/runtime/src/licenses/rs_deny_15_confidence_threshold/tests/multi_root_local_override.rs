use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_15_confidence_threshold as assertions;

use super::super::{build_fixture_deny_toml, set_license_confidence_threshold};

#[test]
fn local_weaker_confidence_threshold_only_warns_for_the_owned_local_root() {
    let results = super::super::run_check(&set_license_confidence_threshold(
        &build_fixture_deny_toml("service"),
        toml::Value::Float(0.7),
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "confidence-threshold weaker than baseline",
            "`deny.toml` sets `confidence-threshold = 0.7`.",
            "deny.toml",
            false,
        )],
    );
}
