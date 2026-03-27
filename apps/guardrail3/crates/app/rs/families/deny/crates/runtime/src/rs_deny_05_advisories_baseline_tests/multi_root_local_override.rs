use guardrail3_app_rs_family_deny_assertions::rs_deny_05_advisories_baseline as assertions;

use super::super::{copy_fixture, set_section_string, write_file, build_fixture_deny_toml};

#[test]
fn local_wrong_advisory_value_only_errors_for_the_owned_local_root() {
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
            "advisories",
            "yanked",
            "deny",
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "advisories `yanked` has wrong value",
            "`apps/devctl/deny.toml` must set `[advisories].yanked = \"warn\"`, found `deny`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
