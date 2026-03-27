use guardrail3_app_rs_family_deny_assertions::rs_deny_10_multiple_versions_floor as assertions;

use super::super::{copy_fixture, set_section_string, write_file, build_fixture_deny_toml};

#[test]
fn local_multiple_versions_weakening_only_warns_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_string(
            &build_fixture_deny_toml("service"),
            "bans",
            "multiple-versions",
            "warn",
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::warn(
            "multiple-versions weaker than baseline",
            "`apps/devctl/deny.toml` sets `[bans].multiple-versions = \"warn\"`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
