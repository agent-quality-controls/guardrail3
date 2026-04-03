use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::helpers::build_fixture_deny_toml;

#[test]
fn warns_on_unknown_bans_and_sources_keys() {
    let deny = build_fixture_deny_toml("service")
        .replace("[bans]\n", "[bans]\nextra-ban-flag = true\n")
        .replace("[sources]\n", "[sources]\nextra-source-flag = true\n");
    let results = super::helpers::run_check(&deny);
    assert!(!results.is_empty());

    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unknown bans key",
                "`deny.toml` uses unknown `[bans].extra-ban-flag`.",
                "deny.toml",
                false,
            ),
            assertions::warn(
                "unknown sources key",
                "`deny.toml` uses unknown `[sources].extra-source-flag`.",
                "deny.toml",
                false,
            ),
        ],
    );
}
