use g3rs_clippy_config_checks_assertions::rs_clippy_config_01_max_struct_bools::rule as assertions;
use super::helpers::run_check;

#[test]
fn errors_when_max_struct_bools_is_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "max-struct-bools missing",
            "Add `max-struct-bools = 3` to clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}
