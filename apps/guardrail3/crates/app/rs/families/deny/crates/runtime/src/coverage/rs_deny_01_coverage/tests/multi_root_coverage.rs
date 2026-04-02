use guardrail3_app_rs_family_deny_assertions::coverage::rs_deny_01_coverage as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn inventories_only_the_top_workspace_root_when_nested_workspaces_exist() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/full_golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/deny.toml",
        &build_fixture_deny_toml("library"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_findings(
        &results,
        &[assertions::info(
            "Rust root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        )],
    );
}
