use g3_deny_content_checks_assertions::rs_deny_04_deprecated_advisories as assertions;

use super::helpers::run_check;

#[test]
fn no_deprecated_fields() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn no_advisories_section() {
    let results = run_check("");
    assertions::assert_no_findings(&results);
}
