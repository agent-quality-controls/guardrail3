use g3rs_clippy_config_checks_assertions::rs_clippy_config_01_max_struct_bools as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_max_struct_bools_has_the_wrong_value() {
    let results = run_check("max-struct-bools = 4\n");

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "max-struct-bools wrong value",
            "Expected 3, got 4. Set `max-struct-bools = 3` in clippy.toml.",
            "clippy.toml",
            false,
        )],
    );
}
