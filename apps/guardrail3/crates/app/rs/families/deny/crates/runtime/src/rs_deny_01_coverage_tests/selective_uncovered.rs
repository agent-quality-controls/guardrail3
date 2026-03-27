use guardrail3_app_rs_family_deny_assertions::rs_deny_01_coverage as assertions;

use super::super::{copy_fixture, write_file, build_fixture_deny_toml};

#[test]
fn errors_only_for_effective_roots_without_a_covering_deny_config() {
    let tmp = copy_fixture("../../../../../../../tests/fixtures/r_arch_01/golden");
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
            assertions::error_no_file(
                "Rust root uncovered by deny config",
                "validation root `.` is not covered by any allowed deny config.",
                false,
            ),
            assertions::error_no_file(
                "Rust root uncovered by deny config",
                "workspace root `.` is not covered by any allowed deny config.",
                false,
            ),
            assertions::error_no_file(
                "Rust root uncovered by deny config",
                "workspace root `apps/backend` is not covered by any allowed deny config.",
                false,
            ),
            assertions::info(
                "Rust root covered by deny config",
                "workspace root `apps/devctl` is covered by `apps/devctl/deny.toml`.",
                "apps/devctl/deny.toml",
                true,
            ),
            assertions::error_no_file(
                "Rust root uncovered by deny config",
                "workspace root `apps/worker` is not covered by any allowed deny config.",
                false,
            ),
        ],
    );
}
