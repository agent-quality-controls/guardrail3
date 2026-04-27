use g3rs_release_config_checks_assertions::accidentally_publishable::rule as assertions;

use super::helpers::run_check;

use super::super::GOLDEN;

#[test]
fn no_error_when_metadata_present() {
    let results = run_check(GOLDEN);

    assertions::assert_no_findings(&results);
}
