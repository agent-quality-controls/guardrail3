use g3_clippy_content_checks_assertions::rs_clippy_02_max_struct_bools as assertions;

use super::helpers::run_check;

#[test]
fn inventories_when_max_struct_bools_matches_baseline() {
    let results = run_check("max-struct-bools = 3\n");

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "max-struct-bools correct",
            "max-struct-bools = 3",
            "clippy.toml",
            true,
        )],
    );
}
