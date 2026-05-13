use g3rs_deny_config_checks_assertions::advisories::advisories_baseline::rule as assertions;

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
