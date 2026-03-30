use guardrail3_app_rs_family_libarch_assertions::rs_libarch_03_crates_dir_exists as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_03_crates_dir_exists::{
    ExpectedRuleResult, Severity,
};

use test_support::{temp_repo, write_layered_library};

const ROOT_CARGO: &str = "packages/shared/Cargo.toml";

#[test]
fn stays_quiet_when_crates_dir_exists() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());

    assertions::assert_rule_quiet(&super::run_family_check(tmp.path()));
}

#[test]
fn errors_when_layered_root_has_no_crates_dir() {
    let tmp = temp_repo();
    write_layered_library(tmp.path());
    test_support::remove_dir(tmp.path(), "packages/shared/crates");

    assertions::assert_rule_results(
        &super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(ROOT_CARGO),
            message_contains: Some("must define `crates/api` and `crates/core`"),
            ..Default::default()
        }],
    );
}
