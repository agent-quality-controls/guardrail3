use g3_deny_content_checks_assertions::rs_deny_05_advisories_baseline as assertions;

use super::helpers::run_check;

#[test]
fn correct_baseline_values() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "warn"
"#,
    );
    assertions::assert_no_findings(&results);
}
