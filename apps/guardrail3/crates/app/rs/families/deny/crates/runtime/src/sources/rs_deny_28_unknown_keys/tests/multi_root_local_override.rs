use guardrail3_app_rs_family_deny_assertions::sources::rs_deny_28_unknown_keys as assertions;

use super::helpers::build_fixture_deny_toml;

#[test]
fn local_unknown_keys_only_warn_for_the_owned_local_root() {
    let local_deny = build_fixture_deny_toml("service")
        .replace("[bans]\n", "[bans]\nextra-ban-flag = true\n")
        .replace("[sources]\n", "[sources]\nextra-source-flag = true\n");
    let results = super::helpers::run_check(&local_deny);
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
