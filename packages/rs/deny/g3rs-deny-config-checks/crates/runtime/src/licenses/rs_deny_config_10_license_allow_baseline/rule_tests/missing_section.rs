use g3rs_deny_config_checks_assertions::licenses::rs_deny_config_10_license_allow_baseline::rule as assertions;

use super::helpers::run_check;

#[test]
fn missing_licenses_section_produces_error() {
    let results = run_check("");
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[licenses] section missing",
            "`deny.toml` has no `[licenses]` section.",
            "deny.toml",
            false,
        )],
    );
}
