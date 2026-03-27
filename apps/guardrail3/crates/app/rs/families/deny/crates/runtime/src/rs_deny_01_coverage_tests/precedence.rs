use guardrail3_app_rs_family_deny_assertions::rs_deny_01_coverage as assertions;

use super::super::{copy_fixture, write_file, build_fixture_deny_toml};

#[test]
fn coverage_uses_highest_precedence_config_at_each_policy_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
    write_file(
        tmp.path(),
        "deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        ".deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        ".cargo/deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/.deny.toml",
        &build_fixture_deny_toml("service"),
    );
    write_file(
        tmp.path(),
        "apps/devctl/deny.toml",
        &build_fixture_deny_toml("service"),
    );

    let results = super::super::run_family(tmp.path());
    assert!(!results.is_empty());
    assertions::assert_contains(
        &results,
        assertions::info(
            "Rust root covered by deny config",
            "validation root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::info(
            "Rust root covered by deny config",
            "workspace root `.` is covered by `deny.toml`.",
            "deny.toml",
            true,
        ),
    );
    assertions::assert_contains(
        &results,
        assertions::info(
            "Rust root covered by deny config",
            "workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`.",
            "apps/devctl/deny.toml",
            true,
        ),
    );
}
