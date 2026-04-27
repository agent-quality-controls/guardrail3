use g3rs_deny_config_checks_assertions::licenses::copyleft_allowlist::rule as assertions;

use super::helpers::run_check;

#[test]
fn multiple_copyleft_licenses_produce_warn_per_license() {
    let results = run_check(
        r#"
[licenses]
allow = ["MIT", "AGPL-3.0", "GPL-3.0", "LGPL-2.1"]
"#,
    );
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "copyleft license allowed",
                "`deny.toml` allows copyleft license `AGPL-3.0`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "copyleft license allowed",
                "`deny.toml` allows copyleft license `GPL-3.0`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "copyleft license allowed",
                "`deny.toml` allows copyleft license `LGPL-2.1`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
