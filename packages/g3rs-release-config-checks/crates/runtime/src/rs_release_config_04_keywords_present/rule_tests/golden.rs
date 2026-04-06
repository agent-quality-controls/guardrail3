use g3rs_release_config_checks_assertions::rs_release_config_04_keywords_present as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_cargo.toml");

#[test]
fn info_when_keywords_present() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: keywords present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
