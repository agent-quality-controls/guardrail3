use g3rs_release_config_checks_assertions::rs_release_config_02_license_present as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_cargo.toml");

#[test]
fn info_when_license_present() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: license present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
