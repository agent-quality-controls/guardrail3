use g3rs_release_config_checks_assertions::rs_release_config_08_binstall_metadata::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

#[test]
fn info_when_binstall_metadata_present() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: binstall metadata present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
