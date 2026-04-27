use g3rs_release_config_checks_assertions::license_present::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

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
