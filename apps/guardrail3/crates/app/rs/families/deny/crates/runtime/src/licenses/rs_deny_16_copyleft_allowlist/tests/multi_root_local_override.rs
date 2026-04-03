use guardrail3_app_rs_family_deny_assertions::licenses::rs_deny_16_copyleft_allowlist as assertions;

use super::helpers::{add_allowed_license, build_fixture_deny_toml};

#[test]
fn local_copyleft_allowance_only_warns_for_the_owned_local_root() {
    let results = super::helpers::run_check(&add_allowed_license(
        &build_fixture_deny_toml("service"),
        "GPL-3.0-only",
    ));
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "copyleft license allowed",
            "`deny.toml` allows copyleft license `GPL-3.0-only`.",
            "deny.toml",
            false,
        )],
    );
}
