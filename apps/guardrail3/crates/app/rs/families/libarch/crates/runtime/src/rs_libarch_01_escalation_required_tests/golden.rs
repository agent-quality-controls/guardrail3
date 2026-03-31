use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required::{
    ExpectedRuleResult, Severity,
};

use test_support::copy_fixture;

const GOLDEN_FIXTURE_REL: &str = "../../../../../../tests/fixtures/r_arch_01/golden";
const GOLDEN_SHARED_TYPES_CARGO: &str = "packages/shared-types/Cargo.toml";
const GOLDEN_SHARED_TYPES_LIB: &str = "packages/shared-types/src/lib.rs";

#[test]
fn golden_fixture_keeps_shared_types_below_flat_library_thresholds() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_LIB,
        "pub struct SharedType;\n",
    );

    assertions::assert_rule_quiet(&super::super::run_family_check(tmp.path()));
}

#[test]
fn golden_fixture_errors_when_shared_types_exceeds_dependency_threshold() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    let dependencies = (0..13)
        .map(|index| format!("dep{index} = \"1\"\n"))
        .collect::<String>();
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_LIB,
        "pub struct SharedType;\n",
    );
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_CARGO,
        &format!(
            "[package]\nname = \"shared-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n{dependencies}"
        ),
    );

    assertions::assert_rule_results(
        &super::super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(GOLDEN_SHARED_TYPES_CARGO),
            message_contains: Some("exceeds the flat-library thresholds"),
            ..Default::default()
        }],
    );
}
