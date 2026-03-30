use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_01_escalation_required::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_flat_library};

mod golden;

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";
const ROOT_LIB: &str = "packages/shared/src/lib.rs";

#[test]
fn stays_quiet_below_escalation_thresholds() {
    let tmp = temp_repo();
    write_flat_library(tmp.path(), 12);

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}

#[test]
fn errors_when_flat_library_exceeds_dependency_threshold() {
    let tmp = temp_repo();
    write_flat_library(tmp.path(), 13);

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("exceeds the flat-library thresholds"),
            ..Default::default()
        }],
    );
}

#[test]
fn fails_closed_when_declared_lib_source_is_missing() {
    let tmp = temp_repo();
    test_support::write_file(
        tmp.path(),
        ROOT_CARGO,
        "[package]\nname = \"shared\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[lib]\npath = \"src/lib.rs\"\n",
    );
    let _ = ROOT_LIB;

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("Cannot verify whether"),
            ..Default::default()
        }],
    );
}
