use g3rs_release_config_checks_assertions::cliff_baseline::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

#[test]
fn info_when_baseline_correct() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "cliff: baseline configuration correct",
            "",
            "cliff.toml",
            true,
        )],
    );
}
