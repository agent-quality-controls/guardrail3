use g3rs_release_config_checks_assertions::rs_release_config_10_release_plz_baseline as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_release_plz.toml");

#[test]
fn info_when_baseline_correct() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "release-plz: baseline configuration correct",
            "",
            "release-plz.toml",
            true,
        )],
    );
}
