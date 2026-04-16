use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_02_max_fn_params_bools::rule as assertions;

#[test]
fn inventories_when_max_fn_params_bools_matches_baseline() {
    let results = run_check("max-fn-params-bools = 3\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "max-fn-params-bools correct",
            "max-fn-params-bools = 3",
            "clippy.toml",
            true,
        )],
    );
}
