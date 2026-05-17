use g3rs_deny_config_checks_assertions::bans::highlight_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_highlight_matches_baseline() {
    let results = run_check(
        r#"
[bans]
highlight = "all"
"#,
    );

    assertions::assert_no_findings(&results);
}
