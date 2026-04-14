use g3rs_deny_config_checks_assertions::rs_deny_config_02_advisories_baseline as assertions;

use super::helpers::run_check;

#[test]
fn correct_baseline_values() {
    let results = run_check(
        r#"
[advisories]
unmaintained = "workspace"
yanked = "deny"
"#,
    );
    assertions::assert_no_findings(&results);
}
