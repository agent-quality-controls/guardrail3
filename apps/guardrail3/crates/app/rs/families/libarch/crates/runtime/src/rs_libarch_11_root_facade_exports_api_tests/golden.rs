use guardrail3_app_rs_family_libarch_assertions::rs_libarch_11_root_facade_exports_api as assertions;
use guardrail3_app_rs_family_libarch_assertions::rs_libarch_11_root_facade_exports_api::{
    ExpectedRuleResult, Severity,
};

use test_support::{copy_fixture, promote_golden_shared_types_to_layered_library};

const GOLDEN_FIXTURE_REL: &str = "../../../../../../tests/fixtures/r_arch_01/golden";
const GOLDEN_SHARED_TYPES_LIB: &str = "packages/shared-types/src/lib.rs";

#[test]
fn golden_fixture_inventories_promoted_shared_types_root_facade_exporting_api() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());

    assertions::assert_rule_results(
        &super::super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Info),
            file: Some(GOLDEN_SHARED_TYPES_LIB),
            inventory: Some(true),
            ..Default::default()
        }],
    );
}

#[test]
fn golden_fixture_errors_when_promoted_shared_types_exports_core_directly() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_LIB,
        "pub use shared_types_core::{AuditStamp, ServiceMode, TenantSlug};\n",
    );

    assertions::assert_rule_results(
        &super::super::run_family_check(tmp.path()),
        &[ExpectedRuleResult {
            severity: Some(Severity::Error),
            file: Some(GOLDEN_SHARED_TYPES_LIB),
            message_contains: Some("violates root facade export policy"),
            ..Default::default()
        }],
    );
}
