use guardrail3_domain_report::{CheckResult, Severity};

use super::{copy_fixture, run_family, write_file};

#[test]
fn full_tree_fixture_enforces_workspace_only_toolchain_policy() {
    let tmp = copy_fixture();

    write_file(
        tmp.path(),
        "Cargo.toml",
        "[package]\nname = \"repo-root\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    );
    write_file(
        tmp.path(),
        "rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/devctl/rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/crates/domain/engine/rust-toolchain.toml",
        "[toolchain]\nchannel = \"1.85.1\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_file(
        tmp.path(),
        "apps/backend/docs/rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );
    write_file(tmp.path(), "packages/ui-kit/rust-toolchain", "stable\n");
    write_file(
        tmp.path(),
        "apps/admin/rust-toolchain.toml",
        "[toolchain]\nchannel = \"stable\"\ncomponents = [\"clippy\", \"rustfmt\"]\n",
    );

    let results = run_family(tmp.path());

    assert_result(
        &results,
        "RS-TOOLCHAIN-01",
        Severity::Error,
        Some("apps/worker/rust-toolchain.toml"),
    );
    assert_result(
        &results,
        "RS-TOOLCHAIN-01",
        Severity::Info,
        Some("apps/backend/rust-toolchain.toml"),
    );
    assert_result(
        &results,
        "RS-TOOLCHAIN-01",
        Severity::Info,
        Some("apps/devctl/rust-toolchain.toml"),
    );

    assert_live_files_for_id(
        &results,
        "RS-TOOLCHAIN-06",
        &[
            "apps/backend/crates/domain/engine/rust-toolchain.toml",
            "apps/backend/docs/rust-toolchain.toml",
        ],
    );
    assert_live_files_for_id(
        &results,
        "RS-TOOLCHAIN-07",
        &[
            "apps/admin/rust-toolchain.toml",
            "packages/ui-kit/rust-toolchain",
            "rust-toolchain.toml",
        ],
    );
    assert!(
        !results.iter().any(|result| {
            result.id() == "RS-TOOLCHAIN-07"
                && matches!(
                    result.file(),
                    Some("apps/backend/crates/domain/engine/rust-toolchain.toml")
                        | Some("apps/backend/docs/rust-toolchain.toml")
                )
        }),
        "nested descendant toolchains should be owned by RS-TOOLCHAIN-06, not doubled as RS-TOOLCHAIN-07: {results:#?}"
    );
}

fn assert_result(results: &[CheckResult], id: &str, severity: Severity, file: Option<&str>) {
    assert!(
        results.iter().any(|result| {
            result.id() == id
                && result.severity() == severity
                && file.is_none_or(|expected| result.file() == Some(expected))
        }),
        "expected {id} result not present for {file:?}: {results:#?}"
    );
}

fn assert_live_files_for_id(results: &[CheckResult], id: &str, expected_files: &[&str]) {
    let files = results
        .iter()
        .filter(|result| result.id() == id && !result.inventory())
        .filter_map(CheckResult::file)
        .collect::<Vec<_>>();
    assert_eq!(files, expected_files, "unexpected {id} files: {results:#?}");
}
