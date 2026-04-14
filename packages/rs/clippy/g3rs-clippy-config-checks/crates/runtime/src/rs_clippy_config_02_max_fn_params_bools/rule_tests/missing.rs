use super::assertions;

use super::helpers::run_check;

#[test]
fn errors_when_max_fn_params_bools_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "max-fn-params-bools missing",
            "Add `max-fn-params-bools = 3` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}
