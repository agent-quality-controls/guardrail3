use g3_deny_content_checks_assertions::rs_deny_23_skip_hygiene as assertions;

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
