use guardrail3_app_rs_family_deny_assertions::rs_deny_14_license_allow_baseline as assertions;

use super::super::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_licenses_section_is_missing() {
    let results = super::super::run_check(&remove_section(
        &build_fixture_deny_toml("service"),
        "licenses",
    ));

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
