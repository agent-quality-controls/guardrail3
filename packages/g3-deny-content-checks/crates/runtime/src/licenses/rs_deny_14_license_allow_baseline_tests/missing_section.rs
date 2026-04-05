use g3_deny_content_checks_assertions::rs_deny_14_license_allow_baseline as assertions;

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
