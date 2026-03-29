use guardrail3_app_rs_family_deny_assertions::rs_deny_16_copyleft_allowlist as assertions;

use super::super::{add_allowed_license, build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn local_copyleft_allowance_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &add_allowed_license(&build_fixture_deny_toml("service"), "GPL-3.0-only"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "copyleft license allowed",
            "`apps/devctl/deny.toml` allows copyleft license `GPL-3.0-only`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
