use g3_deny_content_checks_assertions::rs_deny_23_skip_hygiene as assertions;

use super::helpers::run_check;

#[test]
fn empty_skip_produces_no_findings() {
    let results = run_check(
        r#"
[bans]
skip = []
"#,
    );
    assertions::assert_no_findings(&results);
}

#[test]
fn no_skip_key_produces_no_findings() {
    let results = run_check(
        r#"
[bans]
multiple-versions = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
