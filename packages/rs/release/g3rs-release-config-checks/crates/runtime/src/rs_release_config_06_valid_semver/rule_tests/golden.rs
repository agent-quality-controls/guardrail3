use g3rs_release_config_checks_assertions::rs_release_config_06_valid_semver as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_cargo.toml");

#[test]
fn info_when_valid_semver() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: valid semver",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
