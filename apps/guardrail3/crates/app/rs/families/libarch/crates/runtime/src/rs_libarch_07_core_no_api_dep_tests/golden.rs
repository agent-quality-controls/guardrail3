use guardrail3_app_rs_family_libarch_assertions::rs_libarch_07_core_no_api_dep as assertions;
use test_support::{copy_fixture, promote_golden_shared_types_to_layered_library};

const GOLDEN_FIXTURE_REL: &str = "../../../../../../tests/fixtures/r_arch_01/golden";
const GOLDEN_SHARED_TYPES_CORE_CARGO: &str = "packages/shared-types/crates/core/Cargo.toml";

#[test]
fn golden_fixture_ignores_promoted_shared_types_core_depends_on_api() {
    let tmp = copy_fixture(GOLDEN_FIXTURE_REL);
    promote_golden_shared_types_to_layered_library(tmp.path());
    test_support::write_file(
        tmp.path(),
        GOLDEN_SHARED_TYPES_CORE_CARGO,
        "[package]\nname = \"shared-types-core\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\nshared-types-api = { workspace = true }\n",
    );

    assertions::assert_rule_quiet(&super::super::run_family_check(tmp.path()));
}
