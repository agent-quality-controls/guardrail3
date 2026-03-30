use guardrail3_app_rs_family_deny_assertions::rs_deny_01_coverage as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn nested_local_cargo_deny_variant_does_not_create_another_covered_workspace_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(tmp.path(), "deny.toml", &build_fixture_deny_toml("service"));
    write_file(
        tmp.path(),
        "apps/devctl/.cargo/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_contains(
        &results,
        assertions::info(
            "Rust root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        ),
    );
}
