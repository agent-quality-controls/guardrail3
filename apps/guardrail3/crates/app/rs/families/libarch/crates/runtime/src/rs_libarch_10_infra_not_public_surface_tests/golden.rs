use guardrail3_app_rs_family_libarch_assertions::rs_libarch_10_infra_not_public_surface as assertions;
use test_support::{copy_fixture, promote_golden_shared_types_to_layered_library};

const GOLDEN_FIXTURE_REL: &str = "../../../../../../tests/fixtures/r_arch_01/golden";
const GOLDEN_SHARED_TYPES_LIB: &str = "packages/shared-types/src/lib.rs";

#[test]
fn golden_fixture_ignores_promoted_shared_types_infra_reexport() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_LIB,
        "pub use shared_types_infra::InfraType;\n",
    );

    assertions::assert_rule_quiet(&super::super::run_family_check(tmp.path()));
}
