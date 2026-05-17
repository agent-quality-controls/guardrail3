use g3rs_deny_config_checks_assertions::licenses::copyleft_allowlist::rule as assertions;

use super::helpers::run_check;

#[test]
fn single_copyleft_license_produces_warn() {
    let results = run_check(
        r#"
[licenses]
allow = ["MIT", "GPL-3.0"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "copyleft license allowed",
            "`deny.toml` allows copyleft license `GPL-3.0`.",
            "deny.toml",
            false,
        )],
    );
}
