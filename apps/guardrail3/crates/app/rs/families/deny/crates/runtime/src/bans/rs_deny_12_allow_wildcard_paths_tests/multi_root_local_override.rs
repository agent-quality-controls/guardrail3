use guardrail3_app_rs_family_deny_assertions::rs_deny_12_allow_wildcard_paths as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, set_section_bool, write_file};

#[test]
fn local_allow_wildcard_paths_drift_only_errors_for_the_owned_local_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &set_section_bool(
            &build_fixture_deny_toml("service"),
            "bans",
            "allow-wildcard-paths",
            false,
        ),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::error(
            "allow-wildcard-paths must be true",
            "`apps/devctl/deny.toml` must set `[bans].allow-wildcard-paths = true`.",
            "apps/devctl/deny.toml",
            false,
        )],
    );
}
