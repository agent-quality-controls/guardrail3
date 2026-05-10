use super::helpers::run_check;
use g3rs_clippy_config_checks_assertions::test_relaxations::rule as assertions;

#[test]
fn inventories_when_test_relaxations_match_the_baseline() {
    let results = run_check(
        r"
allow-dbg-in-tests = false
allow-expect-in-tests = true
allow-panic-in-tests = false
allow-print-in-tests = false
allow-unwrap-in-tests = false
",
    );

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "clippy test relaxation policy exact",
            "Managed test relaxation keys match the expected clippy policy.",
            "clippy.toml",
            true,
        )],
    );
}
