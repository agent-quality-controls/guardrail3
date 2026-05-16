use g3rs_deny_config_checks_assertions::advisories::deprecated_advisories::rule as assertions;

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
