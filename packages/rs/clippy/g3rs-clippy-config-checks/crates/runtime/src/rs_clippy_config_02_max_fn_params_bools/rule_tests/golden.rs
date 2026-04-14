use super::assertions;

use super::helpers::run_check;

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
