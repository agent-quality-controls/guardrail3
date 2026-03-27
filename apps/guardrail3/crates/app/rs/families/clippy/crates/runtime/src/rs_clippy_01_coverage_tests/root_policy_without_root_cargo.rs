use guardrail3_app_rs_family_clippy_assertions::rs_clippy_01_coverage as assertions;
use guardrail3_domain_modules::clippy::build_clippy_toml;
use guardrail3_domain_report::Severity;
use test_support::{create_dir_all, create_temp_dir, write_file};

use super::super::run_for_tests;

#[test]
fn validation_root_clippy_covers_descendant_workspace_without_root_cargo() {
    let tmp = create_temp_dir("root-policy-without-root-cargo");
    create_dir_all(&tmp.path().join("apps/backend/crates/core"));
    write_file(
        tmp.path(),
        "clippy.toml",
        &build_clippy_toml("service", false, true, "", ""),
    );
    write_file(
        tmp.path(),
        "apps/backend/Cargo.toml",
        "[workspace]\nmembers = [\"crates/*\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/core/Cargo.toml",
        "[package]\nname = \"core\"\n",
    );

    let results = run_for_tests(tmp.path());
    assertions::assert_multi_root_coverage(
        &results,
        &[(
            "workspace root `apps/backend` is covered by `clippy.toml`.",
            Severity::Info,
            true,
            Some("clippy.toml"),
            "Rust unit covered by clippy.toml",
        )],
    );
}
