use g3rs_release_config_checks_assertions::rs_release_config_05_categories_present::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

#[test]
fn info_when_categories_present() {
    let results = run_check(GOLDEN);

    assertions::assert_findings(
        &results,
        &[assertions::info(
            "golden-crate: categories present",
            "",
            "Cargo.toml",
            true,
        )],
    );
}
