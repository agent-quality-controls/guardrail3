use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_06_multiple_versions_floor as assertions;

use super::helpers::{build_fixture_deny_toml, set_section_string};

#[test]
fn warns_when_multiple_versions_is_weaker_than_baseline() {
    let results = super::helpers::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "multiple-versions",
        "warn",
    ));

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "multiple-versions weaker than baseline",
            "`deny.toml` sets `[bans].multiple-versions = \"warn\"`.",
            "deny.toml",
            false,
        )],
    );
}
