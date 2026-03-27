use guardrail3_app_rs_family_deny_assertions::rs_deny_25_allow_override_channel as assertions;

use super::super::{copy_fixture, set_bans_allow_entries, write_file, build_fixture_deny_toml};

#[test]
fn local_allow_list_presence_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_bans_allow_entries(
            &build_fixture_deny_toml("service"),
            vec![toml::Value::String("totally-custom-crate".to_owned())],
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "bans allow-list present",
            "`apps/devctl/deny.toml` has non-empty `[bans].allow`: totally-custom-crate.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
