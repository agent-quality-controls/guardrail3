use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{build_fixture_clippy_toml, write_file};

use super::super::{copy_fixture_for_tests, run_for_tests};

#[test]
fn inventories_exact_covering_config_for_each_rust_root_in_multi_root_fixture() {
    let tmp = copy_fixture_for_tests();
    write_file(
        tmp.path(),
        "clippy.toml",
        &build_fixture_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/devctl/clippy.toml",
        &build_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "packages/shared-types/clippy.toml",
        &build_clippy_toml("library", false, true, "", ""),
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[
            (
                "workspace root `apps/backend` is covered by `clippy.toml`.",
                Severity::Info,
                true,
                Some("clippy.toml"),
                "Rust unit covered by clippy.toml",
            ),
            (
                "workspace root `apps/devctl` is covered by `apps/devctl/clippy.toml`.",
                Severity::Info,
                true,
                Some("apps/devctl/clippy.toml"),
                "Rust unit covered by clippy.toml",
            ),
            (
                "workspace root `apps/worker` is covered by `clippy.toml`.",
                Severity::Info,
                true,
                Some("clippy.toml"),
                "Rust unit covered by clippy.toml",
            ),
            (
                "workspace root is covered by `clippy.toml`.",
                Severity::Info,
                true,
                Some("clippy.toml"),
                "Rust unit covered by clippy.toml",
            ),
        ],
    );
}
