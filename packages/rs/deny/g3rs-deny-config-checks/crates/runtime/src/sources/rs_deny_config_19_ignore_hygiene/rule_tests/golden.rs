use g3rs_deny_config_checks_assertions::rs_deny_config_19_ignore_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn empty_ignore_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
ignore = []
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn no_ignore_key_produces_no_findings() {
    let results = run_check(
        r#"
[advisories]
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
