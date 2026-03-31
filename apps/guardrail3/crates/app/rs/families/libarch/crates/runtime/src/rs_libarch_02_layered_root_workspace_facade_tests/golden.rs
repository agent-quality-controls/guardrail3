use guardrail3_app_rs_family_libarch_assertions::rs_libarch_02_layered_root_workspace_facade as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_02_layered_root_workspace_facade::{
    ExpectedRuleResult, Severity,
};
use test_support::{copy_fixture, promote_golden_shared_types_to_layered_library};

const GOLDEN_FIXTURE_REL: &str = "../../../../../../tests/fixtures/full_golden";
const GOLDEN_SHARED_TYPES_CARGO: &str = "packages/shared-types/Cargo.toml";

#[test]
fn golden_fixture_ignores_promoted_shared_types_nested_workspace() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());

    assertions::assert_rule_quiet(&super::super::run_family_check(tmp.path()));
}

#[test]
fn golden_fixture_ignores_promoted_shared_types_after_workspace_table_is_removed() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_CARGO,
        "[package]\nname = \"shared-types\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshared-types-api = { path = \"crates/api\" }\nshared-types-core = { path = \"crates/core\" }\nshared-types-infra = { path = \"crates/infra\" }\n",
    );

    assertions::assert_rule_results(
        &super::super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(GOLDEN_SHARED_TYPES_CARGO),
            message_contains: Some("must keep both `[workspace]` and `[package]`"),
            ..Default::default()
        }],
    );
}
