use g3rs_deny_config_checks_assertions::bans::rs_deny_config_08_allow_wildcard_paths::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_allow_wildcard_paths_is_true() {
    let results = run_check(
        r#"
[bans]
allow-wildcard-paths = true
"#,
    );

    assertions::assert_no_findings(&results);
}
