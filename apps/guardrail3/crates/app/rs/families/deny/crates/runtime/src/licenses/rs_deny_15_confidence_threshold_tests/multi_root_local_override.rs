use guardrail3_app_rs_family_deny_assertions::rs_deny_15_confidence_threshold as assertions;

use super::super::{
    build_fixture_deny_toml, copy_fixture, set_license_confidence_threshold, write_file,
};

#[test]
fn local_weaker_confidence_threshold_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_license_confidence_threshold(
            &build_fixture_deny_toml("service"),
            toml::Value::Float(0.7),
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "confidence-threshold weaker than baseline",
            "`apps/devctl/deny.toml` sets `confidence-threshold = 0.7`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
