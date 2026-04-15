use g3rs_clippy_config_checks_assertions::rs_clippy_config_02_max_fn_params_bools::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_max_fn_params_bools_has_the_wrong_value() {
    let results = run_check("max-fn-params-bools = 4\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "max-fn-params-bools wrong value",
            "Expected 3, got 4. Set `max-fn-params-bools = 3` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}
