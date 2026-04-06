use guardrail3_app_rs_family_deny_assertions::bans::rs_deny_config_08_allow_wildcard_paths as assertions;

use super::helpers::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_bans_section_is_missing() {
    let results =
        super::helpers::run_check(&remove_section(&build_fixture_deny_toml("service"), "bans"));

    assertions::assert_findings(
        &results,
        &[assertions::error(
            "[bans] section missing",
            "`deny.toml` has no `[bans]` section.",
            "deny.toml",
            false,
        )],
    );
}
