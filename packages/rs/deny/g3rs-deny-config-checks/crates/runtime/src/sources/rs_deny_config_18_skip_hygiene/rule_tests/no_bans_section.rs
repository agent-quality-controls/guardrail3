use g3rs_deny_config_checks_assertions::sources::rs_deny_config_18_skip_hygiene::rule as assertions;

use super::helpers::run_check;

#[test]
fn no_bans_section_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
