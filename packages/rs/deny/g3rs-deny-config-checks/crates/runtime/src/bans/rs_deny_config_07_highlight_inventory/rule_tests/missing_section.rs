use g3rs_deny_config_checks_assertions::bans::rs_deny_config_07_highlight_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_bans_section_missing() {
    let results = run_check("");

    assertions::assert_no_findings(&results);
}
