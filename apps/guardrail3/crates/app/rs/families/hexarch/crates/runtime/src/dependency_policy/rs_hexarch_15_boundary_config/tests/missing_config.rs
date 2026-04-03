use super::helpers::check_boundary_config_for_test as check_boundary_config;
use guardrail3_app_rs_family_hexarch_assertions::dependency_policy::rs_hexarch_15_boundary_config as assertions;

#[test]
fn non_app_boundaries_do_not_warn() {
    let results = check_boundary_config("packages/shared", false, false, None);

    assertions::assert_no_warning(&results, "");
}
