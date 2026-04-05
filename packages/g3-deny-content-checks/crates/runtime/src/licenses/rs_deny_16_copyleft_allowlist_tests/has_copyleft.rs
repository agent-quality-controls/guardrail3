use g3_deny_content_checks_assertions::rs_deny_16_copyleft_allowlist as assertions;

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
