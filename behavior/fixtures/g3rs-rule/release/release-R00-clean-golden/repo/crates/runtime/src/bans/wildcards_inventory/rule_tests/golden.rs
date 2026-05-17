use g3rs_deny_config_checks_assertions::bans::wildcards_inventory::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_wildcards_matches_baseline() {
    let results = run_check(
        r#"
[bans]
wildcards = "allow"
"#,
    );

    assertions::assert_no_findings(&results);
}
