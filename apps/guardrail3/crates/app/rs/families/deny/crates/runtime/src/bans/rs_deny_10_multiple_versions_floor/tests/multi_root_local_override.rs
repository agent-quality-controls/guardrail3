use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_06_multiple_versions_floor as assertions;

use super::helpers::{build_fixture_deny_toml, set_section_string};

#[test]
fn local_multiple_versions_weakening_only_warns_for_the_owned_local_root() {
    let results = super::helpers::run_check(&set_section_string(
        &build_fixture_deny_toml("service"),
        "bans",
        "multiple-versions",
        "warn",
    ));
    assert!(!results.is_empty());
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
