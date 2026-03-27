use guardrail3_app_rs_family_deny_assertions::rs_deny_26_ban_reason_inventory as assertions;

use super::super::{copy_fixture, remove_deny_ban_reason, write_file, build_fixture_deny_toml};

#[test]
fn local_missing_ban_reason_only_inventories_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &remove_deny_ban_reason(&build_fixture_deny_toml("service"), "json5"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "ban entry missing reason",
            "`apps/devctl/deny.toml` ban entry `json5` has no `reason`.",
            "apps/devctl/deny.toml",
            true,
        )],
    );
}
