use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_10_multiple_versions_floor as assertions;

use super::super::{build_fixture_deny_toml, remove_section_key};

#[test]
fn warns_when_multiple_versions_is_missing() {
    let results = super::super::run_check(&remove_section_key(
        &build_fixture_deny_toml("service"),
        "bans",
        "multiple-versions",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "multiple-versions missing",
            "`deny.toml` does not set `[bans].multiple-versions`.",
            "deny.toml",
            false,
        )],
    );
}
