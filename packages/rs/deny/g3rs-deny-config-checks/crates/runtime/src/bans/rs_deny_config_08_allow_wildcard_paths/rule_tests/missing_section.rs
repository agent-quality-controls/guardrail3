use g3rs_deny_config_checks_assertions::rs_deny_config_08_allow_wildcard_paths as assertions;

use super::helpers::run_check;

#[test]
fn errors_when_bans_section_missing() {
    let results = run_check("");

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
