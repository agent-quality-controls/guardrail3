use g3_deny_content_checks_assertions::rs_deny_11_highlight_inventory as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_bans_section_missing() {
    let results = run_check("");

    assertions::assert_no_findings(&results);
}
