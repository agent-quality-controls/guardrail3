use guardrail3_app_rs_family_deny_assertions::rs_deny_01_coverage as assertions;

use super::super::{build_fixture_deny_toml, copy_fixture, write_file};

#[test]
fn inventories_exact_covering_deny_config_for_each_effective_rust_root() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
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
        &[
            assertions::info(
                "Rust root covered by deny config",
                "validation root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "Rust root covered by deny config",
                "workspace root `.` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "Rust root covered by deny config",
                "workspace root `apps/backend` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
            assertions::info(
                "Rust root covered by deny config",
                "workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`.",
                "apps/devctl/deny.toml",
                true,
            ),
            assertions::info(
                "Rust root covered by deny config",
                "workspace root `apps/worker` is covered by `deny.toml`.",
                "deny.toml",
                true,
            ),
        ],
    );
}
