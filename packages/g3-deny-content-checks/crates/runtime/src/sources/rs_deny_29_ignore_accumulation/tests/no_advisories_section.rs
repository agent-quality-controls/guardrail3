use g3_deny_content_checks_assertions::rs_deny_29_ignore_accumulation as assertions;

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
