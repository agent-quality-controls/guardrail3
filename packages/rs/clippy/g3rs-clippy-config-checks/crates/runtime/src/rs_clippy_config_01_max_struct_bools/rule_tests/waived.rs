use super::helpers::run_check_with_waiver;
use g3rs_clippy_config_checks_assertions::rs_clippy_config_01_max_struct_bools::rule as assertions;

#[test]
fn skips_max_struct_bools_when_exact_waiver_matches() {
    let results = run_check_with_waiver("max-struct-bools = 32\n");

    assertions::assert_no_findings(&results);
}
