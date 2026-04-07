use g3rs_release_config_checks_assertions::rs_release_config_11_cliff_baseline as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_cliff.toml");

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
