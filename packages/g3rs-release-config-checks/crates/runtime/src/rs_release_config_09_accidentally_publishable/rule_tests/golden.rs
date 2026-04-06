use g3rs_release_config_checks_assertions::rs_release_config_09_accidentally_publishable as assertions;

use super::helpers::run_check;

const GOLDEN: &str = include_str!("../../fixtures/golden_cargo.toml");

#[test]
fn no_error_when_metadata_present() {
    let results = run_check(GOLDEN);

    assertions::assert_no_findings(&results);
}
