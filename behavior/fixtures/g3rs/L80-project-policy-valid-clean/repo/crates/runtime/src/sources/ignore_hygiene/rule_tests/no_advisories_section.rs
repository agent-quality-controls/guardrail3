use g3rs_deny_config_checks_assertions::sources::ignore_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_advisories_section_produces_no_findings() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
