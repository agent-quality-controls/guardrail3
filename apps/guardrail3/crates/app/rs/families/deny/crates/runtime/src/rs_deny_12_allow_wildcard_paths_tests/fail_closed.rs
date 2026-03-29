use guardrail3_app_rs_family_deny_assertions::rs_deny_12_allow_wildcard_paths as assertions;

use super::super::{build_fixture_deny_toml, remove_section};

#[test]
fn errors_when_bans_section_is_missing() {
    let results =
        super::super::run_check(&remove_section(&build_fixture_deny_toml("service"), "bans"));

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
