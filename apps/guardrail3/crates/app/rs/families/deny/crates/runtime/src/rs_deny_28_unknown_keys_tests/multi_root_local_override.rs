use guardrail3_app_rs_family_deny_assertions::rs_deny_28_unknown_keys as assertions;

use super::super::{copy_fixture, write_file, build_fixture_deny_toml};

#[test]
fn local_unknown_keys_only_warn_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    let local_deny = build_fixture_deny_toml("service")
        .replace("[bans]\n", "[bans]\nextra-ban-flag = true\n")
        .replace("[sources]\n", "[sources]\nextra-source-flag = true\n");
    write_file(tmp.path(), "apps/devctl/deny.toml", &local_deny);

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[
            assertions::warn(
                "unknown bans key",
                "`apps/devctl/deny.toml` uses unknown `[bans].extra-ban-flag`.",
                "apps/devctl/deny.toml",
                false,
            ),
            assertions::warn(
                "unknown sources key",
                "`apps/devctl/deny.toml` uses unknown `[sources].extra-source-flag`.",
                "apps/devctl/deny.toml",
                false,
            ),
        ],
    );
}
