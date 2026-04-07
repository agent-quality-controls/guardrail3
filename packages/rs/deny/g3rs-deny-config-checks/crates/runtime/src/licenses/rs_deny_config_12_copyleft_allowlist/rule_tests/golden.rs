use g3rs_deny_config_checks_assertions::rs_deny_config_12_copyleft_allowlist as assertions;

use super::helpers::run_check;

#[test]
fn no_copyleft_licenses_produces_no_findings() {
    let results = run_check(
        r#"
[licenses]
allow = ["MIT", "Apache-2.0"]
"#,
    );
    assertions::assert_no_findings(&results);
}
