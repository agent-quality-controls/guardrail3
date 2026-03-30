use guardrail3_app_rs_family_deny_assertions::rs_deny_02_allowed_locations as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn flags_nested_workspace_deny_configs_as_forbidden_locations() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_contains(
        &results,
        assertions::error(
            "deny config at forbidden location",
            "`apps/devctl/deny.toml` (deny.toml) is at `apps/devctl` which is not an allowed deny policy root.",
            "apps/devctl/deny.toml",
            false,
        ),
    );
}
