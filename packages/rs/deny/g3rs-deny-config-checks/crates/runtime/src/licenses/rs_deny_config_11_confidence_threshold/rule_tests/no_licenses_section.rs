use g3rs_deny_config_checks_assertions::licenses::rs_deny_config_11_confidence_threshold::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_licenses_section_produces_no_findings() {
    let results = run_check("");
    assertions::assert_no_findings(&results);
}
