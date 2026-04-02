use guardrail3_app_rs_family_libarch_assertions::rs_libarch_04_exact_layered_crate_set as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_04_exact_layered_crate_set::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library, write_util_member};

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";

#[test]
fn stays_quiet_for_exact_layered_set() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}

#[test]
fn errors_when_extra_layer_dir_exists() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    write_util_member(tmp.path());

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("optional `infra` only"),
            ..Default::default()
        }],
    );
}
