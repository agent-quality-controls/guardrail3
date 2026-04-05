use g3_deny_content_checks_assertions::rs_deny_11_highlight_inventory as assertions;

use super::helpers::run_check;

#[test]
fn no_findings_when_highlight_matches_baseline() {
    let results = run_check(
        r#"
[bans]
highlight = "all"
"#,
    );

    assertions::assert_no_findings(&results);
}
