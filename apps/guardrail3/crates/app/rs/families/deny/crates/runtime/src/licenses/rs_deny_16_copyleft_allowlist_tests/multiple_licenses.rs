use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_16_copyleft_allowlist as assertions;

use super::super::{add_allowed_license, build_fixture_deny_toml};

#[test]
fn warns_once_per_copyleft_license_in_allow_list() {
    let deny = add_allowed_license(
        &add_allowed_license(&build_fixture_deny_toml("service"), "GPL-3.0"),
        "LGPL-3.0",
    );
    let results = super::super::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "copyleft license allowed",
                "`deny.toml` allows copyleft license `GPL-3.0`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "copyleft license allowed",
                "`deny.toml` allows copyleft license `LGPL-3.0`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
