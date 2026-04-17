use g3rs_release_config_checks_assertions::rs_release_config_03_repository_present::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

#[test]
fn info_when_repository_present() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: repository present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
