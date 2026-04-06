use g3rs_deny_config_checks_assertions::rs_deny_config_06_multiple_versions_floor as assertions;

use super::helpers::run_check;

#[test]
fn warns_when_bans_section_missing() {
    let results = run_check("");

    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "[bans] section missing",
            "`deny.toml` has no `[bans]` section.",
            "deny.toml",
            false,
        )],
    );
}
