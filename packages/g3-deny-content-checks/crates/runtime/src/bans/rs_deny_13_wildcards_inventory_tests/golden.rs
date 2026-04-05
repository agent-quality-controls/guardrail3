use g3_deny_content_checks_assertions::rs_deny_13_wildcards_inventory as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_wildcards_matches_baseline() {
    let results = run_check(
        r#"
[bans]
wildcards = "allow"
"#,
    );

    assertions::assert_no_findings(&results);
}
