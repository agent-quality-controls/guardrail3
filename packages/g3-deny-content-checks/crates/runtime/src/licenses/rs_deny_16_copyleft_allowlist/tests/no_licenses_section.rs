use g3_deny_content_checks_assertions::rs_deny_16_copyleft_allowlist as assertions;

use super::helpers::run_check;

#[test]
fn no_licenses_section_produces_no_findings() {
    let results = run_check("");
    assertions::assert_no_findings(&results);
}
