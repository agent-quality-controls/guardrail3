use g3rs_deny_config_checks_assertions::rs_deny_config_06_multiple_versions_floor as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_multiple_versions_is_deny() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "deny"
"#,
    );

    assertions::assert_no_findings(&results);
}
